use crate::core::server::KizunaServer;
use std::io;

mod core;
mod packet;
mod udp;
mod util;

static ADDR: &'static str = "0.0.0.0:12345";

#[tokio::main]
async fn main() -> io::Result<()> {
    let s = KizunaServer::bind(ADDR)?;

    s.run().await?;

    Ok(())
}
