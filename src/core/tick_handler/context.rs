use crate::core::KizunaStateStruct;
use crate::packet::Packet;
use bytes::Bytes;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TickHandlerCtx {
    pub sock: Arc<UdpSocket>,
    pub state: Arc<RwLock<KizunaStateStruct>>,
}

impl TickHandlerCtx {
    pub fn send(&self, packet: &Packet, addr: &SocketAddr) -> io::Result<usize> {
        let bytes: Bytes = packet.into();

        self.send_bytes(&bytes, addr)
    }

    pub fn send_bytes(&self, bytes: &Bytes, addr: &SocketAddr) -> io::Result<usize> {
        self.sock.send_to(&bytes[..], addr)
    }

    pub fn send_raw(&self, bytes: &[u8], addr: &SocketAddr) -> io::Result<usize> {
        self.sock.send_to(&bytes[..], addr)
    }
}
