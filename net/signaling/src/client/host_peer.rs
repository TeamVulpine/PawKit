use super::socket::ClientSocket;
use just_webrtc::types::{ICECandidate, SessionDescription};

use crate::model::{
    HostId,
    c2s::{SignalMessageC2S, host_peer::HostPeerMessageC2S},
    s2c::{SignalMessageS2C, host_peer::HostPeerMessageS2C},
};

pub struct HostPeerSignalingClient {
    sock: ClientSocket,
    pub host_id: HostId,
}

pub struct ClientConnectionCandidate {
    pub offer: SessionDescription,
    pub candidates: Vec<ICECandidate>,
    pub client_id: u64,
}

impl HostPeerSignalingClient {
    pub async fn new(server_url: &str, game_id: u32, request_proxy: bool) -> Option<Self> {
        let mut sock = ClientSocket::open(server_url, crate::SendMode::Cbor).await?;

        sock.send(SignalMessageC2S::HostPeer {
            value: HostPeerMessageC2S::Register {
                game_id,
                request_proxy,
            },
        })
        .await;

        let Some(SignalMessageS2C::HostPeer {
            value: HostPeerMessageS2C::Registered { host_id },
        }) = sock.recv().await
        else {
            return None;
        };

        return Some(Self { sock, host_id });
    }

    pub fn is_open(&self) -> bool {
        return self.sock.is_open();
    }

    pub async fn next_candidate(&mut self) -> Option<ClientConnectionCandidate> {
        let SignalMessageS2C::HostPeer {
            value:
                HostPeerMessageS2C::ConnectionRequested {
                    offer,
                    candidates,
                    client_id,
                },
        } = self.sock.recv().await?
        else {
            return None;
        };

        return Some(ClientConnectionCandidate {
            offer,
            candidates,
            client_id,
        });
    }

    pub async fn accept_candidate(
        &mut self,
        client_id: u64,
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
    ) {
        self.sock
            .send(SignalMessageC2S::HostPeer {
                value: HostPeerMessageC2S::AcceptConnection {
                    offer,
                    candidates,
                    client_id,
                },
            })
            .await;
    }

    pub async fn reject_candidate(&mut self, client_id: u64) {
        self.sock
            .send(SignalMessageC2S::HostPeer {
                value: HostPeerMessageC2S::RejectConnection { client_id },
            })
            .await;
    }
}
