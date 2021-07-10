use crate::core::KizunaCtx;
use crate::packet::error::{HandlePacketError, ParsePacketError};
use crate::packet::{Packet, PacketSelfHandler};
use async_trait::async_trait;
use bytes::{BufMut, Bytes, BytesMut};
use std::convert::TryFrom;

pub struct KeepalivePacket {}

impl KeepalivePacket {
    pub const PKT: u8 = 4;

    pub fn new() -> Self {
        Self {}
    }
}

impl Into<Bytes> for &KeepalivePacket {
    fn into(self) -> Bytes {
        let mut bytes = BytesMut::new();

        bytes.put(Packet::SIG);
        bytes.put_u8(KeepalivePacket::PKT);

        Bytes::from(bytes)
    }
}

impl TryFrom<&Bytes> for KeepalivePacket {
    type Error = ParsePacketError;

    fn try_from(_: &Bytes) -> Result<Self, Self::Error> {
        Ok(KeepalivePacket::new())
    }
}

#[async_trait]
impl PacketSelfHandler for KeepalivePacket {
    async fn handle(&self, ctx: &KizunaCtx) -> Result<(), HandlePacketError> {
        // TODO: Перенос знакомых в приятелей, приятелей в друзей

        Ok(())
    }
}
