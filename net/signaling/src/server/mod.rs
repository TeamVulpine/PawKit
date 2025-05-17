use std::{
    collections::HashMap,
    random::random,
    sync::{Arc, PoisonError, RwLockReadGuard},
    time::Duration,
};

use just_webrtc::types::{ICECandidate, SessionDescription};
use pawkit_holy_array::HolyArray;
use socket::ServerSocket;
use tokio::{
    net::TcpListener,
    sync::{
        mpsc::{self, UnboundedSender},
        RwLock,
    },
    time::sleep,
};
use tokio_tungstenite::accept_async;

use crate::model::{
    c2s::{client_peer::ClientPeerMessageC2S, host_peer::HostPeerMessageC2S, SignalMessageC2S},
    s2c::{client_peer::ClientPeerMessageS2C, host_peer::HostPeerMessageS2C, SignalMessageS2C},
    HostId, SignalingError,
};

mod socket;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PackedGameLobby {
    pub game_id: u32,
    pub lobby_id: u32,
}

/// A simple signaling server.
///
/// Does not provide utilities for clustering,
/// and does not proxy connection requests to other signaling addresses.
pub struct SimpleSignalingServer {
    listener: TcpListener,
    server_url: String,
    host_peers: RwLock<HashMap<PackedGameLobby, UnboundedSender<HostPeerMessageS2C>>>,
    client_peers: RwLock<HolyArray<UnboundedSender<ClientPeerMessageS2C>>>,
}

impl SimpleSignalingServer {
    pub async fn new(addr: &str, server_url: String) -> Option<Arc<Self>> {
        let listener = TcpListener::bind(addr).await.ok()?;

        return Some(Arc::new(Self {
            listener,
            server_url,
            host_peers: RwLock::new(HashMap::new()),
            client_peers: RwLock::new(HolyArray::new()),
        }));
    }

    async fn acquire_lobby(&self, game_id: u32, send: UnboundedSender<HostPeerMessageS2C>) -> u32 {
        let mut peers = self.host_peers.write().await;

        let lobby = loop {
            let lobby_id = random::<u32>();

            let lobby = PackedGameLobby { game_id, lobby_id };

            if !peers.contains_key(&lobby) {
                break lobby;
            }
        };

        peers.insert(lobby, send);

        return lobby.lobby_id;
    }

    async fn release_lobby(&self, game_id: u32, lobby_id: u32) {
        let mut peers = self.host_peers.write().await;

        peers.remove(&PackedGameLobby { game_id, lobby_id });
    }

    async fn get_lobby(
        &self,
        game_id: u32,
        lobby_id: u32,
    ) -> Option<UnboundedSender<HostPeerMessageS2C>> {
        let peers = self.host_peers.read().await;

        let Some(peer) = peers.get(&PackedGameLobby { game_id, lobby_id }) else {
            return None;
        };

        return Some(peer.clone());
    }

    async fn acquire_client(&self, send: UnboundedSender<ClientPeerMessageS2C>) -> usize {
        let mut peers = self.client_peers.write().await;

        return peers.acquire(send);
    }

    async fn release_client(&self, client_id: usize) {
        let mut peers = self.client_peers.write().await;

        peers.release(client_id);
    }

    async fn get_client_peer(
        &self,
        client_id: usize,
    ) -> Result<
        Option<UnboundedSender<ClientPeerMessageS2C>>,
        PoisonError<RwLockReadGuard<'_, HolyArray<UnboundedSender<ClientPeerMessageS2C>>>>,
    > {
        let peers = self.client_peers.read().await;

        let Some(peer) = peers.get(client_id) else {
            return Ok(None);
        };

        return Ok(Some(peer.clone()));
    }

    async fn host_peer(&self, mut socket: ServerSocket, game_id: u32) {
        let (send, mut recv) = mpsc::unbounded_channel::<HostPeerMessageS2C>();

        let lobby_id = self.acquire_lobby(game_id, send).await;

        let host_id = HostId {
            server_url: self.server_url.clone(),
            lobby_id,
            shard_id: 0,
        };

        pawkit_logger::info(&format!(
            "Host peer connected with Game ID {} and Host ID {}",
            game_id, host_id
        ));

        socket
            .send(SignalMessageS2C::HostPeer {
                value: HostPeerMessageS2C::Registered {
                    host_id: host_id.clone(),
                },
            })
            .await;

        while socket.is_open() {
            tokio::select! {
                Some(msg) = socket.recv() => {
                    match msg {
                        SignalMessageC2S::HostPeer {
                            value:
                                HostPeerMessageC2S::RejectConnection {
                                    client_id,
                                },
                        } => {
                            let Ok(peer) = self.get_client_peer(client_id).await else {
                                socket
                                    .send(SignalMessageS2C::Error {
                                        value: SignalingError::InternalError,
                                    })
                                    .await;

                                pawkit_logger::info(&format!("Disconnecting host peer with Game ID {} and Host ID {}", game_id, host_id));
                                self.release_lobby(game_id, lobby_id).await;
                                return;
                            };

                            let Some(peer) = peer else {
                                socket
                                    .send(SignalMessageS2C::Error {
                                        value: SignalingError::UnknownClientId,
                                    })
                                    .await;
                                continue;
                            };

                            if peer.send(ClientPeerMessageS2C::ConnectionRejected).is_err() {
                                socket
                                    .send(SignalMessageS2C::Error {
                                        value: SignalingError::InternalError,
                                    })
                                    .await;

                                pawkit_logger::info(&format!("Disconnecting host peer with Game ID {} and Host ID {}", game_id, host_id));
                                self.release_lobby(game_id, lobby_id).await;
                                return;
                            }
                        }

                        SignalMessageC2S::HostPeer {
                            value:
                                HostPeerMessageC2S::AcceptConnection {
                                    offer,
                                    candidates,
                                    client_id,
                                },
                        } => {
                            let Ok(peer) = self.get_client_peer(client_id).await else {
                                socket
                                    .send(SignalMessageS2C::Error {
                                        value: SignalingError::InternalError,
                                    })
                                    .await;

                                pawkit_logger::info(&format!("Disconnecting host peer with Game ID {} and Host ID {}", game_id, host_id));
                                self.release_lobby(game_id, lobby_id).await;
                                return;
                            };

                            let Some(peer) = peer else {
                                socket
                                    .send(SignalMessageS2C::Error {
                                        value: SignalingError::UnknownClientId,
                                    })
                                    .await;
                                continue;
                            };

                            if peer.send(ClientPeerMessageS2C::ConnectionAccepted { offer, candidates }).is_err() {
                                socket
                                    .send(SignalMessageS2C::Error {
                                        value: SignalingError::InternalError,
                                    })
                                    .await;

                                pawkit_logger::info(&format!("Disconnecting host peer with Game ID {} and Host ID {}", game_id, host_id));
                                self.release_lobby(game_id, lobby_id).await;
                                return;
                            }
                        }

                        _ => {
                            socket
                                .send(SignalMessageS2C::Error {
                                    value: SignalingError::InvalidExpectedMessage,
                                })
                                .await;
                        }
                    }
                },

                Some(msg) = recv.recv() => {
                    socket.send(SignalMessageS2C::HostPeer { value: msg }).await;
                }

                _ = sleep(Duration::from_millis(500)) => {}

                else => break
            }
        }

        pawkit_logger::info(&format!(
            "Host peer with Game ID {} and Host ID {} disconnected.",
            game_id, host_id
        ));

        self.release_lobby(game_id, lobby_id).await;
    }

    async fn client_peer(
        &self,
        mut socket: ServerSocket,
        game_id: u32,
        host_id: HostId,
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
    ) {
        let (send, mut recv) = mpsc::unbounded_channel::<ClientPeerMessageS2C>();

        let client_id = self.acquire_client(send).await;

        let peer = self.get_lobby(game_id, host_id.lobby_id).await;

        let Some(peer) = peer else {
            socket
                .send(SignalMessageS2C::Error {
                    value: SignalingError::UnknownHostId,
                })
                .await;
            self.release_client(client_id).await;
            return;
        };

        if let Err(err) = peer.send(HostPeerMessageS2C::ConnectionRequested {
            offer,
            candidates,
            client_id,
        }) {
            pawkit_logger::error(&format!("{:#?}", err));
            socket
                .send(SignalMessageS2C::Error {
                    value: SignalingError::InternalError,
                })
                .await;
            self.release_client(client_id).await;
            return;
        }

        while socket.is_open()
            && let Some(msg) = recv.recv().await
        {
            socket
                .send(SignalMessageS2C::ClientPeer { value: msg })
                .await;
        }

        self.release_client(client_id).await;
    }

    pub async fn start(self: Arc<Self>) {
        while let Ok((stream, _)) = self.listener.accept().await {
            let cloned = self.clone();
            tokio::spawn(async move {
                let ws_res = accept_async(stream).await;
                let Ok(ws_stream) = ws_res else {
                    pawkit_logger::error(&format!(
                        "Websocket handshake failed: {}",
                        ws_res.unwrap_err()
                    ));
                    return;
                };
                let mut socket = ServerSocket::new(ws_stream, crate::SendMode::Cbor);

                let Some(message) = socket.recv().await else {
                    pawkit_logger::debug("Websocket connection closed before sending any messages");
                    return;
                };

                match message {
                    SignalMessageC2S::HostPeer {
                        value: HostPeerMessageC2S::Register { game_id },
                    } => {
                        cloned.host_peer(socket, game_id).await;
                    }

                    SignalMessageC2S::ClientPeer {
                        value:
                            ClientPeerMessageC2S::RequestConnection {
                                game_id,
                                host_id,
                                offer,
                                candidates,
                            },
                    } => {
                        cloned
                            .client_peer(socket, game_id, host_id, offer, candidates)
                            .await;
                    }

                    _ => {
                        socket
                            .send(SignalMessageS2C::Error {
                                value: SignalingError::InvalidExpectedMessage,
                            })
                            .await;
                    }
                }
            });
        }
    }
}
