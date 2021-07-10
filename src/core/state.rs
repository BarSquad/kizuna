use crate::core::node::Node;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KizunaStateKind {
    Created,
    Initialized,
}

pub struct KizunaStateStruct {
    pub kind: KizunaStateKind,
    pub me: Option<Node>,
    pub nodes: Vec<Node>,

    pub current_tick: u64,
}

impl KizunaStateStruct {
    pub fn new() -> Self {
        Self {
            kind: KizunaStateKind::Created,
            me: None,
            nodes: Vec::new(),

            current_tick: 0,
        }
    }
}

#[async_trait]
pub trait KizunaState {
    async fn identify(&self, node: Node);
    async fn me(&self) -> Option<Node>;
}

#[async_trait]
impl KizunaState for Arc<RwLock<KizunaStateStruct>> {
    async fn identify(&self, node: Node) {
        self.write().await.me = Some(node);
    }

    async fn me(&self) -> Option<Node> {
        self.read().await.me
    }
}
