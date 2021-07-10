use crate::core::node::{Node, NodeColor, NodeKind};
use async_trait::async_trait;
use std::net::IpAddr;
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
        let test_init_nodes = vec![Node {
            kind: NodeKind::Friend,
            ip: IpAddr::V4("81.177.140.148".parse().unwrap()),
            port: 12345,
            color: NodeColor::White,
        }];

        Self {
            kind: KizunaStateKind::Created,
            me: None,
            nodes: test_init_nodes,

            current_tick: 0,
        }
    }
}

#[async_trait]
pub trait KizunaState {
    async fn change_kind(&self, kind: KizunaStateKind);
    async fn identify(&self, node: Node);
    async fn me(&self) -> Option<Node>;
}

#[async_trait]
impl KizunaState for Arc<RwLock<KizunaStateStruct>> {
    async fn change_kind(&self, kind: KizunaStateKind) {
        self.write().await.kind = kind;
    }

    async fn identify(&self, node: Node) {
        self.write().await.me = Some(node);
    }

    async fn me(&self) -> Option<Node> {
        self.read().await.me
    }
}
