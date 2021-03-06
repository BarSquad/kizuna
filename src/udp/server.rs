use crate::udp::{UdpCtx, UdpError};
use async_trait::async_trait;
use bytes::Bytes;
use std::io;
use std::net::{ToSocketAddrs, UdpSocket};
use std::sync::Arc;
use udp_sas::UdpSas;

#[async_trait]
pub trait UdpHandler: Sync + Send + 'static {
    async fn handle(&self, ctx: UdpCtx) -> Result<(), UdpError>;
}

pub struct UdpServer {
    pub sock: Arc<UdpSocket>,
    pub handler: Arc<dyn UdpHandler>,
}

// TODO: Реализовать нормальное логирование
impl UdpServer {
    pub fn bind<A: ToSocketAddrs, H: UdpHandler>(addr: A, handler: H) -> io::Result<Self> {
        let sock = UdpSocket::bind_sas(addr)?;

        println!("Started server on: {}", sock.local_addr()?);

        Ok(Self {
            sock: Arc::new(sock),
            handler: Arc::new(handler),
        })
    }

    pub fn run(&self) -> io::Result<()> {
        let sock = self.sock.clone();
        let local_port = sock.local_addr()?.port();

        let mut buf = [0; 1024];

        loop {
            let (len, addr, local_addr) = sock.recv_sas(&mut buf)?;
            let ctx = UdpCtx {
                sock: sock.clone(),
                addr,
                local_addr,
                local_port,
                bytes: Bytes::from(buf[..len].to_vec()),
            };

            println!("{:?}", ctx);

            let handler = self.handler.clone();

            tokio::spawn(async move {
                match handler.handle(ctx).await {
                    Err(err) => eprintln!("{}", err),
                    _ => (),
                };
            });
        }
    }
}
