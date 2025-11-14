use std::{net::Ipv4Addr, net::SocketAddr, sync::Arc};

use tokio::{io::{AsyncBufReadExt, AsyncReadExt, BufReader}, net::UdpSocket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    // let room = args.get(0).expect("The multicast address is not provided");
    // let name = args.get(1).expect("The user name is not provided");
    
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    let shared_socket = Arc::new(socket);
    
    // let room_ip: SocketAddr = room.parse().unwrap();
    let recv_socket = Arc::clone(&shared_socket);
    let interface = "127.0.0.1".parse().unwrap();
    let room: Ipv4Addr = "224.0.0.1".parse().unwrap();
    recv_socket.join_multicast_v4(room, interface).unwrap();
    
    let send_socket = Arc::clone(&shared_socket);
    tokio::task::spawn(async move {
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
    }).await.unwrap();

    Ok(())
}
