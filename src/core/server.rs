use crate::core::node::Node;
use crate::packet::{Packet, PacketSelfHandler};
use crate::udp::{UdpCtx, UdpHandler, UdpServer};
use async_trait::async_trait;
use std::convert::TryFrom;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};

pub struct KizunaState {
    pub me: Option<Node>,
    pub nodes: Vec<Node>,
}

pub struct KizunaCtx {
    pub(crate) udp: UdpCtx,
    pub(crate) state: Arc<Mutex<KizunaState>>,
}

pub struct KizunaServer {
    udp: UdpServer,
    state: Arc<Mutex<KizunaState>>,
}

struct KizunaHandler {
    state: Arc<Mutex<KizunaState>>,
}

#[async_trait]
impl UdpHandler for KizunaHandler {
    async fn handle(&self, ctx: UdpCtx) -> Result<(), ()> {
        let packet = Packet::try_from(ctx.bytes.clone()).unwrap();
        let ctx = KizunaCtx {
            udp: ctx,
            state: self.state.clone(),
        };

        Ok(packet.handle(&ctx).await.unwrap())
    }
}

impl KizunaServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let state = Arc::new(Mutex::new(KizunaState {
            me: None,
            nodes: Vec::new(),
        }));

        let handler = KizunaHandler {
            state: state.clone(),
        };

        let udp = UdpServer::bind(addr, handler)?;

        Ok(Self { udp, state })
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.udp.run()
    }
}
