use crate::core::node::{NodeColor, NodeKind};
use crate::core::state::KizunaStateKind;
use crate::core::tick_handler::{TickHandler, TickHandlerCtx};
use crate::packet::{IdentReqPacket, Packet};
use async_trait::async_trait;
use rand::seq::IteratorRandom;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct IdentTickHandler {}

impl IdentTickHandler {
    pub fn new_box() -> Box<Self> {
        Box::new(IdentTickHandler {})
    }
}

#[async_trait]
impl TickHandler for IdentTickHandler {
    async fn tick(&self, ctx: Arc<TickHandlerCtx>) {
        let state = ctx.state.read().await;
        let nodes = &state.nodes;
        let target = nodes
            .into_iter()
            .filter(|node| node.kind == NodeKind::Friend && node.color == NodeColor::White)
            .choose(&mut rand::thread_rng());

        if let Some(target) = target {
            println!("ident : {:?}", target);
            ctx.send(
                &Packet::IdentReq(IdentReqPacket::new()),
                &SocketAddr::new(target.ip, target.port),
            )
            .ok();
        }
    }

    fn tick_period(&self) -> u64 {
        20
    }

    fn can_handle(&self, state_kind: KizunaStateKind) -> bool {
        state_kind == KizunaStateKind::Created
    }
}
