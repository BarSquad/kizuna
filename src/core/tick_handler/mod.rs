mod context;
mod ident;
mod keepalive_comrade;
mod keepalive_familiar;
mod keepalive_friend;

pub use self::context::TickHandlerCtx;
pub use self::ident::IdentTickHandler;
pub use self::keepalive_comrade::KeepaliveComradeTickHandler;
pub use self::keepalive_familiar::KeepaliveFamiliarTickHandler;
pub use self::keepalive_friend::KeepaliveFriendTickHandler;

use crate::core::state::KizunaStateKind;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait TickHandler: Send + Sync + 'static {
    async fn tick(&self, ctx: Arc<TickHandlerCtx>);

    fn tick_period(&self) -> u64;
    fn can_handle(&self, state_kind: KizunaStateKind) -> bool;
}

pub fn get_tick_handlers() -> Vec<Box<dyn TickHandler>> {
    vec![
        IdentTickHandler::new_box(),
        KeepaliveFamiliarTickHandler::new_box(),
        KeepaliveComradeTickHandler::new_box(),
        KeepaliveFriendTickHandler::new_box(),
    ]
}
