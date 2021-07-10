mod ident;

pub use self::ident::IdentTickHandler;

use crate::core::state::KizunaStateKind;
use crate::core::KizunaStateStruct;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait TickHandler: Send + Sync + 'static {
    async fn tick(&self, state: Arc<RwLock<KizunaStateStruct>>);

    fn tick_period(&self) -> u64;
    fn can_handle(&self, state_kind: KizunaStateKind) -> bool;
}

pub fn get_tick_handlers() -> Vec<Box<dyn TickHandler>> {
    vec![IdentTickHandler::new_box()]
}
