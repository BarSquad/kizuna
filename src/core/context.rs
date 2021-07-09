use crate::core::KizunaStateStruct;
use crate::packet::Packet;
use crate::udp::UdpCtx;
use bytes::Bytes;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct KizunaCtx {
    pub udp: UdpCtx,
    pub state: Arc<RwLock<KizunaStateStruct>>,
}

impl KizunaCtx {
    pub fn send(self, packet: &Packet, addr: &SocketAddr) -> io::Result<usize> {
        let bytes: Bytes = packet.into();

        self.udp.send(&bytes, addr)
    }

    pub fn resp(&self, packet: &Packet) -> io::Result<usize> {
        let bytes: Bytes = packet.into();

        self.udp.resp(&bytes)
    }
}
