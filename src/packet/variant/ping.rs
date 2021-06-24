use crate::packet::base::PacketSelfHandler;
use crate::packet::error::{HandlePacketError, ParsePacketError};
use crate::KizunaCtx;
use async_trait::async_trait;
use bytes::Bytes;
use std::convert::TryFrom;
use udp_sas::UdpSas;

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
        ctx.req
            .sock
            .send_sas(PONG_BYTES, &ctx.req.addr, &ctx.req.local_addr)?;

        Ok(())
    }
}
