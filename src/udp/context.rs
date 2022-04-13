use bytes::Bytes;
use std::io;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::Arc;
use udp_sas::UdpSas;

#[derive(Debug)]
pub struct UdpCtx {
    pub sock: Arc<UdpSocket>,
    pub addr: SocketAddr,
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub bytes: Bytes,
}

impl UdpCtx {
    pub fn send(&self, bytes: &[u8]) -> io::Result<usize> {
        self.sock.send_sas(&bytes, &self.addr, &self.local_addr)
    }
}
