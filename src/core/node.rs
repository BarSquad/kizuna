use std::net::IpAddr;

#[derive(Debug)]
pub enum NodeKind {
    Me,
    Familiar,
    Comrade,
    Friend,
}

#[derive(Debug)]
pub enum NodeColor {
    White,
    Gray,
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub ip: IpAddr,
    pub port: u16,
    pub color: NodeColor,
}
