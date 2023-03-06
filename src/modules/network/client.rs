use std::{net::{TcpStream, UdpSocket, SocketAddr, SocketAddrV4, Ipv4Addr}, thread::JoinHandle, io::Write};
use std::thread;

use foundry::*;
use crate::{NetworkSerializable, DefaultNetworkMessages};

use super::{packet::Packet, buffer::{TcpBuffer, UdpBuffer}};

/// client representation of the connection to the server
pub struct Client<H: ClientHandler> {
    id: Option<u64>,
    client_handler: H,
    // all this tcp stuff could be refactor into own struct ? no uses for now
    tcp_connection: Option<TcpStream>,
    tcp_buffer: TcpBuffer,
    udp_connection: Option<UdpSocket>,
    udp_buffer: UdpBuffer,
    tcp_incoming_packet: Option<Packet>,
    tcp_connecting_thread: Option<JoinHandle<Result<TcpStream, String>>>,
}

impl<H: ClientHandler> Client<H> {
    pub fn new(client_handler: H) -> Client<H> {
        Client {
            id: None,
            client_handler: client_handler,
            tcp_connection: None,
            tcp_buffer: TcpBuffer::new(),
            udp_connection: None,
            udp_buffer: UdpBuffer::new(),
            tcp_incoming_packet: None,
            tcp_connecting_thread: None,
        }
    }

    pub fn try_connect(&mut self, ip: Ipv4Addr, port: u16) {
        // clone the adress so we won't move ourself in the thread
        let address = SocketAddr::V4(SocketAddrV4::new(ip, port));
        self.tcp_connecting_thread = Some(thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(2));
            match TcpStream::connect(&address) {
                Ok(stream) => {
                    match stream.set_nonblocking(true) {
                        Err(e) => println!("[NETWORK CLIENT] -> Unable to set client as nonblocking ({e}). All client actions may freeze the engine."),
                        _ => {},
                    }
                    return Ok(stream);
                }
                Err(e) => return Err(format!("Unable to connect to {address} : {e}")),
            }
        }));
    }

    pub fn disconnect(&mut self, reason: DisconnectReason, components: &mut ComponentTable) {
        self.client_handler.on_disconected(reason, components);
        self.tcp_connection = None;
        self.udp_connection = None;
        println!("[NETWORK CLIENT] -> Got disconnected from server for reason : {reason:?}");
    }

    fn create_udp_from_tcp(&self, tcp: &TcpStream) -> Result<UdpSocket, std::io::Error> {
        let udp = UdpSocket::bind(tcp.local_addr()?)?;
        udp.connect(tcp.peer_addr()?)?;
        udp.set_nonblocking(true)?;
        Ok(udp)
    }

    pub fn get_incoming_packets(&mut self) -> Result<Vec<Packet>, std::io::Error> {
        let mut result = self.get_incoming_tcp()?;
        result.append(&mut self.get_incoming_udp()?);
        Ok(result)
    }

    pub fn get_incoming_tcp(&mut self) -> Result<Vec<Packet>, std::io::Error> {
        match &mut self.tcp_connection {
            Some(connection) => {
                if self.tcp_buffer.read_tcp(connection)? {
                    // let's read !
                    // check if there is an unfinished packet to read
                    let mut result = Vec::new();
                    if let Some(mut packet) = self.tcp_incoming_packet.take() {
                        // try complete the packet
                        if self.tcp_buffer.try_complete_packet(&mut packet) {
                            result.push(packet);
                        }
                        else {
                            // buffer is empty, we can return
                            return Ok(result);
                        }
                    }
                    loop {
                        let new_packet = self.tcp_buffer.try_read_packet();
                        match new_packet {
                            Some(packet) => {
                                if packet.awaiting_size() == 0 {
                                    // packet complete, keep getting more !
                                    result.push(packet);
                                }
                                else {
                                    // put the packet as awaiting and break
                                    self.tcp_incoming_packet = Some(packet);
                                    break;
                                }
                            },
                            None => break,
                        }
                    }
                    Ok(result)
                }
                else {
                    Ok(Vec::with_capacity(0))
                }
            }
            None => Ok(Vec::with_capacity(0)),
        }
    }

    fn get_incoming_udp(&mut self) -> Result<Vec<Packet>, std::io::Error> {
        match &mut self.udp_connection {
            Some(connection) => {
                let mut result = Vec::new();
                while let Some(packet) = self.udp_buffer.read_tcp(connection)? {
                    result.push(packet.0); // todo; check it is the server ?
                }
                Ok(result)
            }
            None => Ok(Vec::with_capacity(0)),
        }
    }

    pub fn send_tcp(&mut self, message: H::ClientsMessages) {
        let packet = Packet::from(message, match self.id {
            Some(id) => id,
            None => {
                println!("Unable to send tcp packet : no known id.");
                return;
            }
        });
        match &mut self.tcp_connection {
            Some(connection) => {
                match connection.write(&packet.as_bytes()) {
                    Ok(_amount_written) => {/* all good */},
                    Err(e) => println!("[NETWORK CLIENT] -> Error while sending data : {e}")
                };
            }
            None => println!("[NETWORK CLIENT] => Unable to send data to server : no active tcp connection !"),
        }
    }

    pub fn send_udp(&mut self, message: H::ClientsMessages) {
        let packet = Packet::from(message, match self.id {
            Some(id) => id,
            None => {
                println!("Unable to send tcp packet : no known id.");
                return;
            }
        });
        match &mut self.udp_connection {
            Some(connection) => {
                let bytes = packet.as_bytes();
                match connection.send(&bytes) {
                    Ok(_amount_written) => { /* all good */},
                    Err(e) => println!("[NETWORK CLIENT] -> Error while sending data : {e}")
                };
            }
            None => println!("[NETWORK CLIENT] => Unable to send data to server : no active udp connection !"),
        }
    }

    pub fn handle_default(&mut self, default_message: DefaultNetworkMessages) {
        match default_message {
            DefaultNetworkMessages::Welcome(id) => {
                self.id = Some(id);
                println!("[NETWORK CLIENT] -> Connected to server as client {id}.");
            },
            _ => {},
        } 
    }

}

impl<H: ClientHandler + 'static> Updatable for Client<H> {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, _user_data: &mut dyn std::any::Any) {
        // check if we are trying to connect to a server
        if let Some(handle) = self.tcp_connecting_thread.take() {
            if handle.is_finished() {
                match handle.join() {
                    Ok(thread_result) => {
                        match thread_result {
                            Ok(tcp_stream) => {
                                // try to create a udp 'connection'
                                let udp = self.create_udp_from_tcp(&tcp_stream);

                                self.tcp_connection = Some(tcp_stream);
                                self.client_handler.on_connected(components);
                                // create the udp !
                                match udp {
                                    Ok(udp) => self.udp_connection = Some(udp),
                                    Err(e) => println!("[NETWORK CLIENT] -> Fail to use udp with the server : {e}."),
                                }
                            }
                            Err(_e) => {self.client_handler.on_connection_failed(components);},
                        }
                    },
                    Err(_e) => { /* unable to finish join thread properly */},
                }
            }
            else {
                // move the thread back, as we moved it to get ownership of it
                self.tcp_connecting_thread = Some(handle);
            }
        }

        let mut to_send_messages = Vec::new();

        // handle incoming tcp messages
        match self.get_incoming_packets() {
            Ok(packets) => {
                for packet in packets {
                    match packet.is_default() {
                        true => match packet.into::<DefaultNetworkMessages>() {
                            Ok(message) => self.handle_default(message),
                            Err(_e) => println!("[NETWORK CLIENT] -> Unable to deserialize packet !"),
                        }
                        false => match packet.into() {
                            Ok(data) => to_send_messages.append(&mut self.client_handler.handle_message(data, components)),
                            Err(_) => println!("[NETWORK CLIENT] -> Unable to deserialize packet !"),
                        }
                    }
                }
            },
            Err(e) => {
                println!("[NETWORK CLIENT] -> Error while receiving tcp packets : {e}.");
                self.disconnect(DisconnectReason::ServerShutDown, components);
            }
        }

        // user stuff
        to_send_messages.append(&mut self.client_handler.update(components, delta));

        // send all messages !
        for message in to_send_messages.into_iter() {
            match message {
                ClientMessage::Tcp(message) => self.send_tcp(message),
                ClientMessage::Udp(message) => self.send_udp(message), 
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DisconnectReason {
    ClientShutDown,
    ServerShutDown,
    ServerKick,
}

pub enum ClientMessage<E: NetworkSerializable> {
    Tcp(E),
    Udp(E),
}

pub trait ClientHandler {
    type ServerMessages: NetworkSerializable;
    type ClientsMessages: NetworkSerializable;
    fn on_connected(&mut self, components: &mut ComponentTable) -> Vec<ClientMessage<Self::ClientsMessages>>;
    fn on_connection_failed(&mut self, components: &mut ComponentTable);
    fn on_disconected(&mut self, reason: DisconnectReason, components: &mut ComponentTable);
    fn update(&mut self, components: &mut ComponentTable, delta: f32) -> Vec<ClientMessage<Self::ClientsMessages>>;
    fn handle_message(&mut self, message: Self::ServerMessages, components: &mut ComponentTable) -> Vec<ClientMessage<Self::ClientsMessages>>;
}