use std::{net::{TcpStream, UdpSocket, SocketAddr}, time::Duration};

use foundry::*;

enum ConnectionStatus{
    Disconnected, 
    TcpOnly(TcpStream),
    UdpOnly(UdpSocket),
    TcpAndUdp(TcpStream, UdpSocket)
}

/// client representation of the connection to the server
pub struct Client {
    connection: ConnectionStatus,
}

impl Client {
    pub fn new() -> Client {
        Client {
            connection: ConnectionStatus::Disconnected,
        }
    }

    pub fn try_connect_tcp(mut self, address: SocketAddr) -> Result<Client, (Client, String)> {
        println!("[NETWORK CLIENT] -> Attempting connection to {address}.");
        match TcpStream::connect(&address) {
            Ok(stream) => {
                match stream.set_nonblocking(true) {
                    Err(e) => println!("[NETWORK CLIENT] -> unable to set client as nonblocking ({e}). All client actions may freeze the engine."),
                    _ => {},
                }
                self.connection = match self.connection {
                    ConnectionStatus::Disconnected => ConnectionStatus::TcpOnly(stream),
                    ConnectionStatus::TcpOnly(_old_stream) => ConnectionStatus::TcpOnly(stream), // previous stream will be dropped
                    ConnectionStatus::UdpOnly(udp_stream) => ConnectionStatus::TcpAndUdp(stream, udp_stream),
                    ConnectionStatus::TcpAndUdp(_old_stream, udp_stream) => ConnectionStatus::TcpAndUdp(stream, udp_stream), // previous stream will be dropped
                };
                println!("[NETWORK CLIENT] -> Connected to server at {address}");
                Ok(self)
            }
            Err(e) => Err((self, format!("[NETWORK CLIENT] -> Unable to connect to {address} : {e}")))
        }
    }
}

impl Updatable for Client {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, user_data: &mut dyn std::any::Any) {
        
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}