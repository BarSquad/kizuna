use crate::core::node::NodeKind;
use crate::core::state::KizunaStateKind;
use crate::core::tick_handler::{TickHandler, TickHandlerCtx};
use crate::packet::{KeepalivePacket, Packet};
use async_trait::async_trait;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct KeepaliveFamiliarTickHandler {}

impl KeepaliveFamiliarTickHandler {
    pub fn new_box() -> Box<Self> {
        Box::new(KeepaliveFamiliarTickHandler {})
    }
}

#[async_trait]
impl TickHandler for KeepaliveFamiliarTickHandler {
    async fn tick(&self, ctx: Arc<TickHandlerCtx>) {
        let state = ctx.state.read().await;

        for node in &state.nodes {
            if node.kind != NodeKind::Familiar {
                continue;
            }

            ctx.send(
                &Packet::Keepalive(KeepalivePacket::new()),
                &SocketAddr::new(node.ip, node.port),
            )
            .ok();
        }
    }

    fn tick_period(&self) -> u64 {
        1
    }

    fn can_handle(&self, state_kind: KizunaStateKind) -> bool {
        state_kind == KizunaStateKind::Initialized
    }
}
