use std::net::IpAddr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NodeKind {
    Me,
    Familiar,
    Comrade,
    Friend,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NodeColor {
    White,
    Gray,
}

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub ip: IpAddr,
    pub port: u16,
    pub color: NodeColor,
}
