use just_webrtc::types::{ICECandidate, SessionDescription};

use crate::model::{
    HostId,
    c2s::{SignalMessageC2S, client_peer::ClientPeerMessageC2S},
    s2c::{SignalMessageS2C, client_peer::ClientPeerMessageS2C},
};

use super::socket::ClientSocket;

pub struct ClientPeerSignalingClient {
    sock: ClientSocket,
    game_id: u32,
}

pub struct HostConnectionCandidate {
    pub offer: SessionDescription,
    pub candidates: Vec<ICECandidate>,
}

impl ClientPeerSignalingClient {
    pub async fn new(server_url: &str, game_id: u32) -> Option<Self> {
        let sock = ClientSocket::open(server_url, crate::SendMode::Cbor).await?;

        return Some(Self { sock, game_id });
    }

    pub fn is_open(&self) -> bool {
        return self.sock.is_open();
    }

    pub async fn offer_connection(
        &mut self,
        host_id: HostId,
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
    ) -> Option<HostConnectionCandidate> {
        self.sock
            .send(SignalMessageC2S::ClientPeer {
                value: ClientPeerMessageC2S::RequestConnection {
                    offer,
                    candidates,
                    host_id,
                    game_id: self.game_id,
                },
            })
            .await;

        let SignalMessageS2C::ClientPeer {
            value: ClientPeerMessageS2C::ConnectionAccepted { offer, candidates },
        } = self.sock.recv().await?
        else {
            return None;
        };

        return Some(HostConnectionCandidate { offer, candidates });
    }
}
