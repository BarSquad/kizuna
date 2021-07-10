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
    pub fn send(&self, bytes: &Bytes, addr: &SocketAddr) -> io::Result<usize> {
        self.sock.send_sas(&bytes[..], addr, &self.local_addr)
    }

    pub fn send_raw(&self, bytes: &[u8], addr: &SocketAddr) -> io::Result<usize> {
        self.sock.send_sas(&bytes[..], addr, &self.local_addr)
    }

    pub fn resp(&self, bytes: &Bytes) -> io::Result<usize> {
        self.send(bytes, &self.addr)
    }

    pub fn resp_raw(&self, bytes: &[u8]) -> io::Result<usize> {
        self.send_raw(bytes, &self.addr)
    }
}
