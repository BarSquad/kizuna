use crate::core::state::KizunaStateKind;
use crate::core::tick_handler::TickHandler;
use crate::core::KizunaStateStruct;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct IdentTickHandler {}

impl IdentTickHandler {
    pub fn new_box() -> Box<Self> {
        Box::new(IdentTickHandler {})
    }
}

#[async_trait]
impl TickHandler for IdentTickHandler {
    async fn tick(&self, state: Arc<RwLock<KizunaStateStruct>>) {
        state.write().await.kind = KizunaStateKind::Created;
        println!(
            "{:?} - {:?}",
            state.read().await.current_tick,
            state.read().await.kind
        );
    }

    fn tick_period(&self) -> u64 {
        4
    }

    fn can_handle(&self, state_kind: KizunaStateKind) -> bool {
        state_kind == KizunaStateKind::Created || state_kind == KizunaStateKind::Initialized
    }
}
