use crate::core::KizunaCtx;
use crate::packet::error::{HandlePacketError, ParsePacketError, ParsePacketErrorKind};
use crate::packet::{EchoPacket, IdentReqPacket, IdentResPacket, KeepalivePacket, PingPacket};
use async_trait::async_trait;
use bytes::Bytes;
use std::convert::TryFrom;

#[async_trait]
pub trait PacketSelfHandler {
    async fn handle(&self, ctx: &KizunaCtx) -> Result<(), HandlePacketError>;
}

pub enum Packet {
    Echo(EchoPacket),
    Ping(PingPacket),
    IdentReq(IdentReqPacket),
    IdentRes(IdentResPacket),
    Keepalive(KeepalivePacket),
}

impl Packet {
    pub const SIG: &'static [u8] = "KIZNA".as_bytes();
    pub const TYPE_ORD: usize = Packet::SIG.len();
    pub const HEADER_LEN: usize = Packet::SIG.len() + 1;
}

impl Into<Bytes> for &Packet {
    fn into(self) -> Bytes {
        match self {
            Packet::Echo(packet) => packet.into(),
            Packet::Ping(packet) => packet.into(),
            Packet::IdentReq(packet) => packet.into(),
            Packet::IdentRes(packet) => packet.into(),
            Packet::Keepalive(packet) => packet.into(),
        }
    }
}

impl TryFrom<&Bytes> for Packet {
    type Error = ParsePacketError;

    fn try_from(bytes: &Bytes) -> Result<Self, Self::Error> {
        if !bytes.starts_with(Packet::SIG) {
            return Err(ParsePacketError::new(ParsePacketErrorKind::InvalidSig));
        }

        match bytes.get(Packet::TYPE_ORD) {
            Some(&EchoPacket::PKT) => Ok(Packet::Echo(EchoPacket::try_from(bytes)?)),
            Some(&PingPacket::PKT) => Ok(Packet::Ping(PingPacket::try_from(bytes)?)),
            Some(&IdentReqPacket::PKT) => Ok(Packet::IdentReq(IdentReqPacket::try_from(bytes)?)),
            Some(&IdentResPacket::PKT) => Ok(Packet::IdentRes(IdentResPacket::try_from(bytes)?)),
            Some(&KeepalivePacket::PKT) => Ok(Packet::Keepalive(KeepalivePacket::try_from(bytes)?)),
            Some(_) => Err(ParsePacketError::new(ParsePacketErrorKind::UnknownType)),
            None => Err(ParsePacketError::new(ParsePacketErrorKind::InvalidType)),
        }
    }
}

#[async_trait]
impl PacketSelfHandler for Packet {
    async fn handle(&self, ctx: &KizunaCtx) -> Result<(), HandlePacketError> {
        match self {
            Packet::Echo(packet) => packet.handle(ctx).await,
            Packet::Ping(packet) => packet.handle(ctx).await,
            Packet::IdentReq(packet) => packet.handle(ctx).await,
            Packet::IdentRes(packet) => packet.handle(ctx).await,
            Packet::Keepalive(packet) => packet.handle(ctx).await,
        }
    }
}
