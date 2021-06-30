use crate::core::server::KizunaServer;
use std::io;

mod core;
mod packet;
mod udp;
mod util;

static ADDR: &'static str = "0.0.0.0:12345";

// TODO: Написать для RequestCtx хелпующие методы, которые позволят отправлять Bytes и Packet

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut s = KizunaServer::bind(ADDR)?;
    s.run()?;

    Ok(())
}
