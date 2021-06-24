use bytes::Bytes;
use std::fmt::Debug;
use std::future::Future;
use std::io;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::Arc;
use udp_sas::UdpSas;

#[derive(Debug)]
pub struct RequestCtx {
    pub sock: Arc<UdpSocket>,
    pub addr: SocketAddr,
    pub local_addr: IpAddr,
    pub bytes: Bytes,
}

pub struct UdpServer<F> {
    // TODO: Убрать pub
    pub sock: Arc<UdpSocket>,
    handler: Option<Arc<F>>,
}

// TODO: Реализовать нормальное логгирование
impl<F, R, E> UdpServer<F>
where
    F: Fn(RequestCtx) -> R + Send + Sync + 'static,
    R: Future<Output = Result<(), E>> + Send,
    E: Debug,
{
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let sock = UdpSocket::bind_sas(addr)?;

        println!("Started server on: {:?}", sock.local_addr()?);

        Ok(Self {
            sock: Arc::new(sock),
            handler: None,
        })
    }

    pub fn run(&self) -> io::Result<()> {
        let sock = self.sock.clone();
        let handler = match &self.handler {
            Some(handler) => handler.clone(),
            None => panic!("UdpServer: self.handler is not set"),
        };

        let mut buf = [0; 1024];

        loop {
            let (len, addr, local_addr) = sock.recv_sas(&mut buf)?;
            let ctx = RequestCtx {
                sock: sock.clone(),
                addr,
                local_addr,
                bytes: Bytes::from(buf[..len].to_vec()),
            };

            println!("{:?}", ctx);

            let handler = handler.clone();

            tokio::spawn(async move {
                match handler(ctx).await {
                    Err(err) => eprintln!("{:?}", err),
                    _ => (),
                };
            });
        }
    }

    pub fn set_handler(&mut self, handler: F) {
        self.handler = Some(Arc::new(handler));
    }
}
