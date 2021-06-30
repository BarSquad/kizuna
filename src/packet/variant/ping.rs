use crate::core::server::KizunaCtx;
use crate::packet::base::PacketSelfHandler;
use crate::packet::error::{HandlePacketError, ParsePacketError};
use async_trait::async_trait;
use bytes::Bytes;
use std::convert::TryFrom;

const PONG_BYTES: &'static [u8] = "Pong\n".as_bytes();

pub struct PingPacket {}

impl PingPacket {
    pub const PKT: u8 = 1;

    pub fn new() -> Self {
        Self {}
    }
}

impl TryFrom<Bytes> for PingPacket {
    type Error = ParsePacketError;

    fn try_from(_: Bytes) -> Result<Self, Self::Error> {
        Ok(PingPacket::new())
    }
}

#[async_trait]
impl PacketSelfHandler for PingPacket {
    async fn handle(&self, ctx: &KizunaCtx) -> Result<(), HandlePacketError> {
        ctx.udp.resp_raw(PONG_BYTES)?;

        Ok(())
    }
}
