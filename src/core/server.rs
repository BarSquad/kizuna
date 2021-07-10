use crate::core::tick_handler::{get_tick_handlers, TickHandler};
use crate::core::{KizunaCtx, KizunaStateStruct};
use crate::packet::{Packet, PacketSelfHandler};
use crate::udp::{UdpCtx, UdpError, UdpHandler, UdpServer};
use async_trait::async_trait;
use std::convert::TryFrom;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

const TICK_RATE: u64 = 500;

pub struct KizunaServer {
    udp: UdpServer,
    state: Arc<RwLock<KizunaStateStruct>>,
    tick_handlers: Arc<Vec<Box<dyn TickHandler>>>,
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
        let tick_handlers = Arc::new(get_tick_handlers());

        Ok(Self {
            udp,
            state,
            tick_handlers,
        })
    }

    pub async fn run(&self) -> io::Result<()> {
        self.run_ticker().await;

        self.udp.run()
    }

    pub async fn run_ticker(&self) {
        let state = self.state.clone();
        let tick_handlers = self.tick_handlers.clone();

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(TICK_RATE)).await;

                KizunaServer::tick(state.clone(), tick_handlers.clone()).await;
            }
        });
    }

    pub async fn tick(
        state: Arc<RwLock<KizunaStateStruct>>,
        tick_handlers: Arc<Vec<Box<dyn TickHandler>>>,
    ) {
        let (state_kind, current_tick) = {
            let state_unlocked = state.read().await;

            let state_kind = state_unlocked.kind;
            let current_tick = state_unlocked.current_tick;

            (state_kind, current_tick)
        };

        for handler in tick_handlers.iter() {
            if handler.can_handle(state_kind) && current_tick % handler.tick_period() == 0 {
                handler.tick(state.clone()).await;
            }
        }

        state.write().await.current_tick += 1;
    }
}
