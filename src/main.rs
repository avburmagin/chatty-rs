use std::{net::Ipv4Addr, net::SocketAddr, sync::Arc};

use socket2::{Domain, Protocol, Socket, Type};
use tokio::{io::{AsyncBufReadExt, BufReader}, net::UdpSocket, task::JoinSet};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    // let room = args.get(0).expect("The multicast address is not provided");
    // let name = args.get(1).expect("The user name is not provided");
    
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_reuse_address(true)?;
    socket.set_nonblocking(true)?;

    let address: std::net::SocketAddr = "0.0.0.0:8080".parse().unwrap();
    socket.bind(&address.into())?;

    let std_socket: std::net::UdpSocket = socket.into();
    let socket = UdpSocket::from_std(std_socket)?;

    let shared_socket = Arc::new(socket);
    
    // let room_ip: SocketAddr = room.parse().unwrap();
    let recv_socket = Arc::clone(&shared_socket);
    let interface = "127.0.0.1".parse().unwrap();
    let room: Ipv4Addr = "224.0.0.1".parse().unwrap();
    recv_socket.join_multicast_v4(room, interface).unwrap();
    

    println!("Entered chat room successfully");

    let mut set = JoinSet::new();
    let send_socket = Arc::clone(&shared_socket);
    set.spawn(async move {
        let mut buf = String::new();
        let mut reader = BufReader::new(tokio::io::stdin());
        loop {
            let result = reader.read_line(&mut buf).await;
            match result {
                Ok(_) => {
                    send_socket.send_to(buf.as_bytes(), "224.0.0.1:8080".parse::<SocketAddr>().unwrap()).await.unwrap();
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    });

    set.spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            match recv_socket.recv_from(&mut buf).await {
                Ok((length, _)) => {
                    println!("{}", std::str::from_utf8(&buf[..length]).unwrap());
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    });

    set.join_all().await;

    Ok(())
}
