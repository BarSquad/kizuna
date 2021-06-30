use crate::core::KizunaStateStruct;
use crate::packet::{Packet, PacketSelfHandler};
use crate::udp::{UdpCtx, UdpError, UdpHandler, UdpServer};
use async_trait::async_trait;
use std::convert::TryFrom;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};

pub struct KizunaCtx {
    pub udp: UdpCtx,
    pub state: Arc<Mutex<KizunaStateStruct>>,
}

pub struct KizunaServer {
    udp: UdpServer,
    state: Arc<Mutex<KizunaStateStruct>>,
}

struct KizunaHandler {
    state: Arc<Mutex<KizunaStateStruct>>,
}

#[async_trait]
impl UdpHandler for KizunaHandler {
    async fn handle(&self, ctx: UdpCtx) -> Result<(), UdpError> {
        let packet = Packet::try_from(ctx.bytes.clone())?;
        let ctx = KizunaCtx {
            udp: ctx,
            state: self.state.clone(),
        };

        Ok(packet.handle(&ctx).await?)
    }
}

impl KizunaServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let state = Arc::new(Mutex::new(KizunaStateStruct {
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
