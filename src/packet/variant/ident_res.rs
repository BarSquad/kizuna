use crate::core::node::{Node, NodeColor, NodeKind};
use crate::core::KizunaState;
use crate::core::{KizunaCtx, KizunaStateKind};
use crate::packet::base::PacketSelfHandler;
use crate::packet::error::{HandlePacketError, ParsePacketError, ParsePacketErrorKind};
use crate::packet::Packet;
use crate::util::addr::{bytes_to_ip, ip_to_bytes};
use async_trait::async_trait;
use byteorder::{ByteOrder, LittleEndian};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::TryFrom;
use std::net::IpAddr;

pub struct IdentResPacket {
    ip: IpAddr,
    port: u16,
}

impl IdentResPacket {
    pub const PKT: u8 = 3;

    pub const IP_LEN: usize = 16;
    pub const PORT_LEN: usize = 2;
    pub const LEN: usize = IdentResPacket::IP_LEN + IdentResPacket::PORT_LEN;

    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self { ip, port }
    }
}

impl Into<Bytes> for &IdentResPacket {
    fn into(self) -> Bytes {
        let mut bytes = BytesMut::new();

        bytes.put(Packet::SIG);
        bytes.put_u8(IdentResPacket::PKT);
        bytes.put(&ip_to_bytes(self.ip)[..]);
        bytes.put_u16_le(self.port);

        Bytes::from(bytes)
    }
}

impl TryFrom<&Bytes> for IdentResPacket {
    type Error = ParsePacketError;

    fn try_from(bytes: &Bytes) -> Result<Self, Self::Error> {
        if bytes.len() != Packet::HEADER_LEN + IdentResPacket::LEN {
            return Err(ParsePacketError::new(ParsePacketErrorKind::InvalidContent));
        }

        let ip_bytes = &bytes[Packet::HEADER_LEN..Packet::HEADER_LEN + IdentResPacket::IP_LEN];
        let port_bytes = &bytes
            [Packet::HEADER_LEN + IdentResPacket::IP_LEN..Packet::HEADER_LEN + IdentResPacket::LEN];

        let ip = match bytes_to_ip(ip_bytes) {
            Some(ip) => ip,
            None => return Err(ParsePacketError::new(ParsePacketErrorKind::InvalidContent)),
        };

        let port = LittleEndian::read_u16(port_bytes);

        Ok(IdentResPacket::new(ip, port))
    }
}

#[async_trait]
impl PacketSelfHandler for IdentResPacket {
    async fn handle(&self, ctx: &KizunaCtx) -> Result<(), HandlePacketError> {
        let color = if ctx.udp.local_addr == self.ip && ctx.udp.local_port == self.port {
            NodeColor::White
        } else {
            NodeColor::Gray
        };

        ctx.state
            .identify(Node {
                kind: NodeKind::Me,
                color,
                ip: self.ip,
                port: self.port,
            })
            .await;

        println!("I am {:?}", ctx.state.me().await);

        ctx.state.change_kind(KizunaStateKind::Initialized).await;

        Ok(())
    }
}
