use crate::packet::base::PacketSelfHandler;
use crate::packet::error::{HandlePacketError, ParsePacketError};
use crate::packet::{IdentResPacket, Packet};
use crate::KizunaCtx;
use async_trait::async_trait;
use bytes::{BufMut, Bytes, BytesMut};
use std::convert::TryFrom;

pub struct IdentReqPacket {}

impl IdentReqPacket {
    pub const PKT: u8 = 2;

    pub fn new() -> Self {
        Self {}
    }
}

impl Into<Bytes> for IdentReqPacket {
    fn into(self) -> Bytes {
        let mut bytes = BytesMut::new();

        bytes.put(Packet::SIG);
        bytes.put_u8(IdentReqPacket::PKT);

        Bytes::from(bytes)
    }
}

impl TryFrom<Bytes> for IdentReqPacket {
    type Error = ParsePacketError;

    fn try_from(_: Bytes) -> Result<Self, Self::Error> {
        Ok(IdentReqPacket::new())
    }
}

#[async_trait]
impl PacketSelfHandler for IdentReqPacket {
    async fn handle(&self, ctx: &KizunaCtx) -> Result<(), HandlePacketError> {
        let res = IdentResPacket::new(ctx.req.addr.ip(), ctx.req.addr.port());
        let res_bytes: Bytes = res.into();

        ctx.req
            .sock
            .send_to(res_bytes.as_ref(), ctx.req.addr)
            .await?;

        Ok(())
    }
}
