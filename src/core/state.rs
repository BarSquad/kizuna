use crate::core::node::Node;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub struct KizunaStateStruct {
    pub me: Option<Node>,
    pub nodes: Vec<Node>,
}

impl KizunaStateStruct {
    pub fn new() -> Self {
        Self {
            me: None,
            nodes: Vec::new(),
        }
    }
}

pub trait KizunaState {
    fn new() -> Arc<Mutex<KizunaStateStruct>> {
        Arc::new(Mutex::new(KizunaStateStruct::new()))
    }

    fn identify(&self, node: Node);
    fn me(&self) -> Option<Node>;
}

impl KizunaState for Arc<Mutex<KizunaStateStruct>> {
    fn identify(&self, node: Node) {
        match self.lock() {
            Ok(mut state) => state.me = Some(node),
            Err(err) => lock_panic(&err),
        };
    }

    fn me(&self) -> Option<Node> {
        match self.lock() {
            Ok(state) => state.me,
            Err(err) => {
                lock_panic(&err);
                None
            }
        }
    }
}

fn lock_panic(err: &dyn Debug) {
    panic!("KizunaState: lock failed {:?}", err);
}
