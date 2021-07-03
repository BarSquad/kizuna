use crate::core::{KizunaCtx, KizunaState, KizunaStateStruct};
use crate::packet::{Packet, PacketSelfHandler};
use crate::udp::{UdpCtx, UdpError, UdpHandler, UdpServer};
use async_trait::async_trait;
use std::convert::TryFrom;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};

pub struct KizunaServer {
    udp: UdpServer,
    state: Arc<Mutex<KizunaStateStruct>>,
}

struct KizunaHandler {
    state: Arc<Mutex<KizunaStateStruct>>,
}

impl KizunaHandler {
    pub fn new(state: Arc<Mutex<KizunaStateStruct>>) -> Self {
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
        let state = KizunaState::new();
        let handler = KizunaHandler::new(state.clone());
        let udp = UdpServer::bind(addr, handler)?;

        Ok(Self { udp, state })
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.udp.run()
    }
}
