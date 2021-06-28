use crate::packet::{Packet, PacketSelfHandler};
use crate::server::{RequestCtx, UdpServer};
use std::convert::TryFrom;
use std::io;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

mod packet;
mod server;
mod util;

const ADDR: &'static str = "0.0.0.0:12345";

#[derive(Debug)]
pub enum NodeColor {
    White,
    Gray,
}

#[derive(Debug)]
pub struct Node {
    pub ip: IpAddr,
    pub port: u16,
    pub color: NodeColor,
}

#[derive(Debug)]
pub struct KizunaApp {
    pub me: Option<Node>,
    pub friends: Vec<Node>,
}

impl KizunaApp {
    pub fn set_me(&mut self, node: Node) {
        self.me = Some(node);
    }
}

#[derive(Debug)]
pub struct KizunaCtx {
    pub app: Arc<Mutex<KizunaApp>>,
    pub req: RequestCtx,
}

// TODO: Объединить UdpServer и KizunaApp
// TODO: Написать для RequestCtx хелпующие методы, которые позволят отправлять Bytes и Packet

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Arc::new(Mutex::new(KizunaApp {
        me: None,
        friends: Vec::new(),
    }));

    let mut serv = UdpServer::bind(ADDR).await?;

    serv.set_handler(move |ctx: RequestCtx| {
        let app = app.clone();

        async move {
            let packet = Packet::try_from(ctx.bytes.clone())?;
            let ctx = &mut KizunaCtx { app, req: ctx };

            packet.handle(ctx).await
        }
    });

    serv.run()?;

    Ok(())
}
