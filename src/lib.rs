use std::net::{IpAddr, SocketAddr};

use net2::UdpSocketExt;
use std::net::UdpSocket;

#[repr(u16)]
enum StunMessage {
    BindingRequest = 0x0001,
    BindingResponse = 0x0101,
}

fn stun_test(socket: &UdpSocket) {}

fn get_nat_type(socket: &UdpSocket, stun_host: &str, stun_port: u16) {
    let ret = stun_test(socket);
}

pub fn get_ip_info(source_ip: IpAddr, source_port: u16, stun_host: &str, stun_port: u16) {
    let socket = UdpSocket::bind(SocketAddr::from((source_ip, source_port))).unwrap();
}
