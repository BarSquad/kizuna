use crate::core::server::KizunaCtx;
use crate::packet::base::PacketSelfHandler;
use crate::packet::error::{HandlePacketError, ParsePacketError};
use crate::packet::Packet;
use async_trait::async_trait;
use bytes::Bytes;
use std::convert::TryFrom;
use udp_sas::UdpSas;

pub struct EchoPacket {
    rest: Vec<u8>,
}

impl EchoPacket {
    pub const PKT: u8 = 0;

    pub fn new(rest: Vec<u8>) -> Self {
        Self { rest }
    }
}

impl TryFrom<Bytes> for EchoPacket {
    type Error = ParsePacketError;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        let rest = bytes[Packet::HEADER_LEN..].to_vec();

        Ok(EchoPacket::new(rest))
    }
}

#[async_trait]
impl PacketSelfHandler for EchoPacket {
    async fn handle(&self, ctx: &KizunaCtx) -> Result<(), HandlePacketError> {
        ctx.udp
            .sock
            .send_sas(&self.rest[..], &ctx.udp.addr, &ctx.udp.local_addr)?;

        Ok(())
    }
}
