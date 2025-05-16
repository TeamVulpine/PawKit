use std::{
    collections::HashMap,
    random::random,
    sync::{Arc, RwLock},
};

use just_webrtc::types::{ICECandidate, SessionDescription};
use socket::ServerSocket;
use tokio::{
    net::TcpListener,
    sync::mpsc::{self, UnboundedSender},
};
use tokio_tungstenite::accept_async;

use crate::model::{
    c2s::{client_peer::ClientPeerMessageC2S, host_peer::HostPeerMessageC2S, SignalMessageC2S},
    s2c::{host_peer::HostPeerMessageS2C, SignalMessageS2C},
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
}

impl SimpleSignalingServer {
    pub async fn new(addr: &str, server_url: String) -> Option<Arc<Self>> {
        let listener = TcpListener::bind(addr).await.ok()?;

        return Some(Arc::new(Self {
            listener,
            server_url,
            host_peers: RwLock::new(HashMap::new()),
        }));
    }

    fn acquire_lobby(
        &self,
        game_id: u32,
        send: UnboundedSender<HostPeerMessageS2C>,
    ) -> Option<u32> {
        let Ok(mut peers) = self.host_peers.write() else {
            pawkit_logger::error("Cannot write to host_peers");
            return None;
        };

        let lobby = loop {
            let lobby_id = random::<u32>();

            let lobby = PackedGameLobby { game_id, lobby_id };

            if !peers.contains_key(&lobby) {
                break lobby;
            }
        };

        peers.insert(lobby, send);

        return Some(lobby.lobby_id);
    }

    fn release_lobby(&self, game_id: u32, lobby_id: u32) {
        let Ok(mut peers) = self.host_peers.write() else {
            pawkit_logger::error("Cannot write to host_peers");
            return;
        };

        peers.remove(&PackedGameLobby { game_id, lobby_id });
    }

    async fn host_peer(&self, mut socket: ServerSocket, game_id: u32) {
        let (send, mut recv) = mpsc::unbounded_channel::<HostPeerMessageS2C>();

        let Some(lobby_id) = self.acquire_lobby(game_id, send) else {
            return;
        };

        let host_id = HostId {
            server_url: self.server_url.clone(),
            lobby_id,
            shard_id: 0,
        };

        socket
            .send(SignalMessageS2C::HostPeer {
                value: HostPeerMessageS2C::Registered {
                    host_id: host_id.clone(),
                },
            })
            .await;

        loop {
            tokio::select! {
                Some(_msg) = socket.recv() => {
                    todo!()
                },

                Some(msg) = recv.recv() => {
                    socket.send(SignalMessageS2C::HostPeer { value: msg }).await;
                }

                else => break
            }
        }

        self.release_lobby(game_id, lobby_id);
    }

    async fn client_peer(
        &self,
        _game_id: u32,
        _host_id: HostId,
        _offer: SessionDescription,
        _candidates: Vec<ICECandidate>,
    ) {
        todo!()
    }

    pub async fn start(self: Arc<Self>) {
        while let Ok((stream, _)) = self.listener.accept().await {
            let cloned = self.clone();
            tokio::spawn(async move {
                let Ok(ws_stream) = accept_async(stream).await else {
                    pawkit_logger::error("Websocket handshake failed");
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
                            ClientPeerMessageC2S::Connect {
                                game_id,
                                host_id,
                                offer,
                                candidates,
                            },
                    } => {
                        cloned
                            .client_peer(game_id, host_id, offer, candidates)
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
