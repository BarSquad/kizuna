use crate::core::KizunaCtx;
use crate::packet::base::PacketSelfHandler;
use crate::packet::error::{HandlePacketError, ParsePacketError};
use crate::packet::Packet;
use async_trait::async_trait;
use bytes::{BufMut, Bytes, BytesMut};
use std::convert::TryFrom;

// TODO: Удалить этот пакет
pub struct EchoPacket {
    rest: Vec<u8>,
}

impl EchoPacket {
    pub const PKT: u8 = 0;

    pub fn new(rest: Vec<u8>) -> Self {
        Self { rest }
    }
}

impl Into<Bytes> for &EchoPacket {
    fn into(self) -> Bytes {
        let mut bytes = BytesMut::new();

        bytes.put(Packet::SIG);
        bytes.put_u8(EchoPacket::PKT);

        Bytes::from(bytes)
    }
}

impl TryFrom<&Bytes> for EchoPacket {
    type Error = ParsePacketError;

    fn try_from(bytes: &Bytes) -> Result<Self, Self::Error> {
        let rest = bytes[Packet::HEADER_LEN..].to_vec();

        Ok(EchoPacket::new(rest))
    }
}

#[async_trait]
impl PacketSelfHandler for EchoPacket {
    async fn handle(&self, ctx: &KizunaCtx) -> Result<(), HandlePacketError> {
        ctx.udp.send(&self.rest[..])?;

        Ok(())
    }
}
