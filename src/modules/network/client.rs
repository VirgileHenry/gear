use std::{net::{TcpStream, UdpSocket, SocketAddr}, time::Duration, thread::{JoinHandle, sleep_ms}, marker::PhantomData, io::Write};
use std::thread;

use foundry::*;
use crate::NetworkSerializable;

use super::{packet::Packet, buffer::NetworkBuffer};

/// client representation of the connection to the server
pub struct Client<E: NetworkSerializable, H: ClientHandler<E>> {
    client_handler: H,
    enum_marker: PhantomData<E>,
    server_addr: SocketAddr,
    // all this tcp stuff could be refactor into own struct ? no uses for now
    tcp_connection: Option<TcpStream>,
    tcp_buffer: NetworkBuffer,
    tcp_incoming_packet: Option<Packet>,
    tcp_connecting_thread: Option<JoinHandle<Result<TcpStream, String>>>,
    // let's implement tcp !
}

impl<E: NetworkSerializable, H: ClientHandler<E>> Client<E, H> {
    pub fn new(client_handler: H, server_addr: SocketAddr) -> Client<E, H> {
        Client {
            client_handler: client_handler,
            enum_marker: PhantomData,
            server_addr: server_addr,
            tcp_connection: None,
            tcp_buffer: NetworkBuffer::new(),
            tcp_incoming_packet: None,
            tcp_connecting_thread: None,
        }
    }

    pub fn try_connect_tcp(&mut self) {
        // clone the adress so we won't move ourself in the thread
        let address = self.server_addr;
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

    pub fn get_incoming_packets(&mut self) -> Vec<Packet> {
        return match self.tcp_buffer.read_tcp(match &mut self.tcp_connection {
            Some(connection) => connection,
            None => return Vec::new(), // can't read if no connection
        }) {
            Ok(new_data_available) => {
                if new_data_available {
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
                            return result;
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
                    result
                }
                else {
                    // no new messages.
                    Vec::new()
                }
            },
            Err(_e) => Vec::new(), // todo : handle this (reading error)
        }
    }

    pub fn send_tcp(&mut self, message: E) {
        let packet = Packet::from(message);
        if let Some(mut connection) = self.tcp_connection.take() {
            match connection.write(&packet.as_bytes()) {
                Ok(_amount_written) => {/* all good */},
                Err(e) => println!("[NETWORK SERVER] -> Error while sending data : {e}")
            };
            self.tcp_connection = Some(connection); // moving twice really ugly // todo
        }
    }

}

impl<E: NetworkSerializable + 'static, H: ClientHandler<E> + 'static> Updatable for Client<E, H> {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, user_data: &mut dyn std::any::Any) {
        // check if we are trying to connect to a server
        if let Some(handle) = self.tcp_connecting_thread.take() {
            if handle.is_finished() {
                match handle.join() {
                    Ok(thread_result) => {
                        match thread_result {
                            Ok(tcp_stream) => {
                                self.tcp_connection = Some(tcp_stream);
                                self.client_handler.on_connected(components);
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
        for packet in self.get_incoming_packets() {
            match packet.into() {
                Ok(data) => to_send_messages.append(&mut self.client_handler.handle_tcp_message(data, components)),
                Err(_e) => {println!("Unable to convert packet back to serialized message !")} // todo : handle packet reading error !
            }
        }

        // user stuff
        to_send_messages.append(&mut self.client_handler.update(components, delta));

        // send all messages !
        for message in to_send_messages.into_iter() {
            self.send_tcp(message);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub enum DisconnectReason {
    ClientShutDown,
    ServerShutDown,
}

pub trait ClientHandler<E: NetworkSerializable> {
    fn on_connected(&mut self, components: &mut ComponentTable) -> Vec<E>;
    fn on_connection_failed(&mut self, components: &mut ComponentTable);
    fn on_disconected(&mut self, reason: DisconnectReason, components: &mut ComponentTable);
    fn update(&mut self, components: &mut ComponentTable, delta: f32) -> Vec<E>;
    fn handle_tcp_message(&mut self, message: E, components: &mut ComponentTable) -> Vec<E>;
}