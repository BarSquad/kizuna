use crate::core::node::Node;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub struct KizunaStateStruct {
    pub me: Option<Node>,
    pub nodes: Vec<Node>,
}

pub trait KizunaState {
    fn new() -> Self;

    fn identify(&self, node: Node);
    fn me(&self) -> Option<Node>;
}

impl KizunaState for Arc<Mutex<KizunaStateStruct>> {
    fn new() -> Self {
        Arc::new(Mutex::new(KizunaStateStruct {
            me: None,
            nodes: Vec::new(),
        }))
    }

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
