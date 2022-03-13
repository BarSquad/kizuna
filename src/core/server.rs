use crate::core::{KizunaCtx, KizunaStateStruct};
use crate::packet::{Packet, PacketSelfHandler};
use crate::udp::{UdpCtx, UdpError, UdpHandler, UdpServer};
use async_trait::async_trait;
use std::convert::TryFrom;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct KizunaServer {
    udp: UdpServer,
}

struct KizunaHandler {
    state: Arc<RwLock<KizunaStateStruct>>,
}

impl KizunaHandler {
    pub fn new(state: Arc<RwLock<KizunaStateStruct>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl UdpHandler for KizunaHandler {
    async fn handle(&self, ctx: UdpCtx) -> Result<(), UdpError> {
        let packet = Packet::try_from(&ctx.bytes)?;
        let ctx = KizunaCtx {
            udp: ctx,
            state: self.state.clone(),
        };

        Ok(packet.handle(&ctx).await?)
    }
}

impl KizunaServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let state = Arc::new(RwLock::new(KizunaStateStruct::new()));
        let handler = KizunaHandler::new(state.clone());
        let udp = UdpServer::bind(addr, handler)?;

        Ok(Self { udp })
    }

    pub async fn run(&self) -> io::Result<()> {
        self.udp.run()
    }
}
