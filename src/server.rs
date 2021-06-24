use bytes::Bytes;
use std::fmt::Debug;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct RequestCtx {
    pub sock: Arc<UdpSocket>,
    pub addr: SocketAddr,
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
        let sock = UdpSocket::bind(addr).await?;

        println!("Started server on: {:?}", sock.local_addr()?);

        Ok(Self {
            sock: Arc::new(sock),
            handler: None,
        })
    }

    pub async fn run(&self) -> io::Result<()> {
        let sock = self.sock.clone();
        let handler = match &self.handler {
            Some(handler) => handler.clone(),
            None => panic!("UdpServer: self.handler is not set"),
        };

        let (tx, mut rx) = mpsc::channel::<RequestCtx>(1024);

        tokio::spawn(async move {
            while let Some(ctx) = rx.recv().await {
                let handler = handler.clone();

                tokio::spawn(async move {
                    match handler(ctx).await {
                        Err(err) => eprintln!("{:?}", err),
                        _ => (),
                    }
                });
            }
        });

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                // TODO: Убрать unwrap
                let (len, addr) = sock.recv_from(&mut buf).await.unwrap();
                let ctx = RequestCtx {
                    sock: sock.clone(),
                    addr,
                    bytes: Bytes::from(buf[..len].to_vec()),
                };

                println!("{:?}", ctx);

                // TODO: Убрать unwrap
                tx.send(ctx).await.unwrap();
            }
        })
        .await?;

        Ok(())
    }

    pub fn set_handler(&mut self, handler: F) {
        self.handler = Some(Arc::new(handler));
    }
}
