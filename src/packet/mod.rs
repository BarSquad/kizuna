pub mod error;

mod base;
mod variant;

pub use self::base::{Packet, PacketSelfHandler};

pub use self::variant::echo::EchoPacket;
pub use self::variant::ident_req::IdentReqPacket;
pub use self::variant::ident_res::IdentResPacket;
pub use self::variant::ping::PingPacket;
