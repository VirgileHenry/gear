use std::net::{TcpStream, UdpSocket, SocketAddr};

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

    pub fn try_connect_tcp(&mut self, address: SocketAddr) -> Result<(), String> {
        match TcpStream::connect(address) {
            Ok(stream) => {
                self.connection = match self.connection {
                    ConnectionStatus::Disconnected => ConnectionStatus::TcpOnly(stream),
                    ConnectionStatus::TcpOnly(_old_stream) => ConnectionStatus::TcpOnly(stream), // previous stream will be dropped
                    ConnectionStatus::UdpOnly(udp_stream) => ConnectionStatus::TcpAndUdp(stream, udp_stream),
                    ConnectionStatus::TcpAndUdp(_old_stream, udp_stream) => ConnectionStatus::TcpAndUdp(stream, udp_stream), // previous stream will be dropped
                };
                Ok(())
            }
            Err(e) => Err(format!("Unable to connect to {address} : {e}")) 
        }
    }
}