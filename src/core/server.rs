use crate::core::{KizunaCtx, KizunaState, KizunaStateStruct};
use crate::packet::{Packet, PacketSelfHandler};
use crate::udp::{UdpCtx, UdpError, UdpHandler, UdpServer};
use async_trait::async_trait;
use std::convert::TryFrom;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

pub struct KizunaServer {
    udp: UdpServer,
    state: Arc<RwLock<KizunaStateStruct>>,
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

        Ok(Self { udp, state })
    }

    pub async fn run(&self) -> io::Result<()> {
        self.run_ticker().await;

        self.udp.run()
    }

    pub async fn run_ticker(&self) {
        let state = self.state.clone();

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(state.get_tick_rate().await)).await;

                KizunaServer::tick(state.clone()).await;
            }
        });
    }

    pub async fn tick(state: Arc<RwLock<KizunaStateStruct>>) {
        let state = state.read().await;

        println!("{:?}", state.kind);
    }
}
