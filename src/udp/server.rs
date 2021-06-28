use async_trait::async_trait;
use bytes::Bytes;
use std::io;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::Arc;
use udp_sas::UdpSas;

#[derive(Debug)]
pub struct UdpCtx {
    pub sock: Arc<UdpSocket>,
    pub addr: SocketAddr,
    pub local_addr: IpAddr,
    pub bytes: Bytes,
}

#[async_trait]
pub trait UdpHandler: Sync + Send + 'static {
    async fn handle(&self, ctx: &UdpCtx) -> Result<(), ()>;
}

pub struct UdpServer {
    sock: Arc<UdpSocket>,
    handler: Arc<dyn UdpHandler>,
}

// TODO: Добавить прокидывание ошибок
// TODO: Реализовать нормальное логгирование
impl UdpServer {
    pub fn bind<A: ToSocketAddrs, H: UdpHandler>(addr: A, handler: H) -> io::Result<Self> {
        let sock = UdpSocket::bind_sas(addr)?;

        println!("Started server on: {:?}", sock.local_addr()?);

        Ok(Self {
            sock: Arc::new(sock),
            handler: Arc::new(handler),
        })
    }

    pub fn run(&self) -> io::Result<()> {
        let sock = self.sock.clone();

        let mut buf = [0; 1024];

        loop {
            let (len, addr, local_addr) = sock.recv_sas(&mut buf)?;
            let ctx = UdpCtx {
                sock: sock.clone(),
                addr,
                local_addr,
                bytes: Bytes::from(buf[..len].to_vec()),
            };

            println!("{:?}", ctx);

            let handler = self.handler.clone();

            tokio::spawn(async move {
                match handler.handle(&ctx).await {
                    Err(err) => eprintln!("{:?}", err),
                    _ => (),
                };
            });
        }
    }
}
