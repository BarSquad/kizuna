use crate::packet::{IdentReqPacket, Packet, PacketSelfHandler};
use crate::server::{RequestCtx, UdpServer};
use bytes::Bytes;
use std::convert::TryFrom;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use udp_sas::UdpSas;

mod packet;
mod server;
mod util;

const ADDR: &'static str = "0.0.0.0:12345";

pub struct Friend {
    pub addr: IpAddr,
}

pub struct KizunaApp {
    pub friends: Vec<Friend>,
}

pub struct KizunaCtx {
    pub app: Arc<KizunaApp>,
    pub req: RequestCtx,
}

// TODO: Объединить UdpServer и KizunaApp
// TODO: Написать для RequestCtx хелпующие методы, которые позволят отправлять Bytes и Packet

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Arc::new(KizunaApp {
        friends: Vec::new(),
    });

    let mut serv = UdpServer::bind(ADDR).await?;

    serv.set_handler(move |ctx: RequestCtx| {
        let app = app.clone();

        async move {
            let packet = Packet::try_from(ctx.bytes.clone())?;
            let ctx = KizunaCtx { app, req: ctx };

            packet.handle(&ctx).await
        }
    });

    let req = IdentReqPacket::new();
    let req_bytes: Bytes = req.into();

    // TODO: Удалить тестовый запрос
    let s = serv.sock.clone();
    tokio::spawn(async move {
        sleep(Duration::from_secs(10)).await;

        s.send_sas(
            req_bytes.as_ref(),
            &SocketAddr::from_str("81.177.140.148:12345").unwrap(),
            &s.local_addr().unwrap().ip(),
        )
        .unwrap();
    });

    serv.run()?;

    Ok(())
}
