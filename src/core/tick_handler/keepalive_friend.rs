use crate::core::state::KizunaStateKind;
use crate::core::tick_handler::{TickHandler, TickHandlerCtx};
use crate::packet::{KeepalivePacket, Packet};
use async_trait::async_trait;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct KeepaliveFriendTickHandler {}

impl KeepaliveFriendTickHandler {
    pub fn new_box() -> Box<Self> {
        Box::new(KeepaliveFriendTickHandler {})
    }
}

#[async_trait]
impl TickHandler for KeepaliveFriendTickHandler {
    async fn tick(&self, ctx: Arc<TickHandlerCtx>) {
        let state = ctx.state.read().await;

        for node in &state.nodes {
            println!("keepalive : {:?}", node);
            ctx.send(
                &Packet::Keepalive(KeepalivePacket::new()),
                &SocketAddr::new(node.ip, node.port),
            )
            .ok();
        }
    }

    fn tick_period(&self) -> u64 {
        4
    }

    fn can_handle(&self, _: KizunaStateKind) -> bool {
        true
    }
}
