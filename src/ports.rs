use rayon::prelude::*;
use crate::{
  model::{Port, Subdomain},
  most_common::MOST_COMMON_PORTS_100
};
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};


pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
  println!("{:?}", subdomain);
  let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
    .to_socket_addrs()
    .expect("port scanner: Creating socket address")
    .collect();
  
    println!("{:?}", socket_addresses);

  if socket_addresses.len() == 0 {
    return subdomain;
  }

  subdomain.open_ports = MOST_COMMON_PORTS_100
    .into_par_iter()
    .map(|port| scan_port(socket_addresses[0], *port))
    .filter(|port| port.is_open) // filter closed ports
    .collect();

    println!("{:?}", subdomain);

  subdomain
}

pub fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_millis(250);
    socket_address.set_port(port);
    let is_open = if let Ok(_) = TcpStream::connect_timeout(&socket_address, timeout) {
        true
    } else {
        false
    };

    Port {
        port,
        is_open,
    }
}
