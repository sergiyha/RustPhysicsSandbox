use tokio::net;
use tokio::prelude::*;
use std::net::SocketAddr;
use tokio::io::Error;

struct ConnectionData
{
    d: SocketAddr,
    msg: String,
}

struct UDPListener
{
    socket: net::UdpSocket
}

impl UDPListener
{
    async fn new() -> UDPListener//here may be an address
    {
        UDPListener { socket: net::UdpSocket::bind("127.0.0.1:9878").await.expect("Failed to initialize socket with address: 127.0.0.1:9878") }
    }

    pub async fn receive_connection(&mut self) -> Result<ConnectionData, Error>
    {
        let mut buf = [0; 1024];

        match self.socket.recv_from(&mut buf).await {
            Ok((l, a)) => {
                let buf_new = &mut buf[..l];
                let buf_result = std::str::from_utf8(&buf_new).expect("data provided till connection is corrupted");
                Ok(ConnectionData { d: a, msg: buf_result.to_string() })
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut udp_connection_listener = UDPListener::new().await;


    let handler = tokio::spawn(async move {
        loop {
            println!("waiting for connection...");
            match udp_connection_listener.receive_connection().await {
                Ok(a) => {
                    println!("congrats msg:{0} and address:{1}", a.msg, a.d.to_string());
                    if a.msg == "exit"
                    {
                        break;
                    }
                }
                Err(err) => { println!("Connection Error: {}", err) }
            };
        }
    });

    let result = handler.await;
}
