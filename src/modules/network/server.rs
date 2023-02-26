use std::{
    collections::HashMap,
    net::{
        UdpSocket, TcpStream, TcpListener, SocketAddr,
    },
    io::{Write},
};

use foundry::*;

use crate::NetworkSerializable;

use super::{packet::Packet, buffer::NetworkBuffer};

/// Server system. When created, will start to listen tcp connections and create Connection when receiving them.
/// H is the server handler
pub struct Server<H: ServerHandler> {
    server_handler: H,
    connections: HashMap<usize, Connection>,
    max_client_count: usize,
    current_client_count: usize,
    next_available_id: usize,
    tcp_listener: TcpListener,
}

impl<H: ServerHandler> Server<H> {
    pub fn new(server_handler: H, port: usize, max_client_count: usize) -> Result<Server<H>, String> {
        Ok(Server { 
            server_handler: server_handler,
            connections: HashMap::new(),
            max_client_count: max_client_count,
            current_client_count: 0,
            next_available_id: 1,
            tcp_listener: match TcpListener::bind(format!("127.0.0.1:{port}")) { // todo : ip adress management
                Ok(listener) => {
                    match listener.set_nonblocking(true) {
                        Ok(()) => println!("[NETWORK SERVER] -> server started on port {port}"),
                        Err(e) => println!("[GEAR ENGINE] -> unable to set tcp listener as non-blocking. This will cause a engine freeze until connections are received ({})", e),
                    };
                    listener
                },
                Err(e) => return Err(format!("Unable to create server : can't bind tcp listener : {e}").to_string()),
            },
        })
    }

    pub fn handle_incoming_connection(&mut self, stream: TcpStream, adress: SocketAddr, components: &mut ComponentTable) -> Vec<ServerMessage<H::ServerMessages>>{
        println!("[NETWORK SERVER] -> incoming connection : {adress}.");
        if self.current_client_count < self.max_client_count {
            println!("[NETWORK SERVER] -> accepted connection as Connection {}.", self.next_available_id);
            let client = match Connection::new(self.next_available_id, stream) {
                Ok(client) => client,
                Err(e) => {
                    println!("[NETWORK SERVER] -> Error while handling incoming connection : {e}.");
                    return Vec::new();
                }
            };
            let result = self.server_handler.on_client_connected(client.id, components);
            self.connections.insert(self.next_available_id, client);
            self.current_client_count += 1;
            self.next_available_id += 1;
            result
        }
        else {
            println!("[NETWORK SERVER] -> rejected connection : server full.");
            // stream.shutdown(std::net::Shutdown::Both);
            Vec::new()
        }
    }

    /// Disconnect the given client.
    fn disconnect_client(&mut self, client: usize, components: &mut ComponentTable) {
        match self.connections.remove(&client) {
            Some(_) => println!("[SERVER] -> Disconnected client {client}."),
            None => println!("[SERVER] -> Unable to disconnect client {client} : not registered."),
        }
        self.server_handler.on_client_disconnected(client, components);
    }

    pub fn send_tcp_to_client(&mut self, to: usize, message: H::ServerMessages, components: &mut ComponentTable) {
        let packet = Packet::from(message);
        match self.connections.get_mut(&to) {
            Some(client) => {
                match client.tcp_connection.write(&packet.as_bytes()) {
                    Ok(_amount_written) => {/* all good */}, // todo : check amount written is enough, or re-write
                    Err(e) => {
                        println!("[NETWORK SERVER] -> Error while sending data to client {to} ({e}). Disconnecting client.");
                        self.disconnect_client(to, components);
                    },
                };
            },
            None => println!("[NETWORK SERVER] -> Attempted to send packet to Connection {to} but Connection was not found."),
        };
    }

    pub fn send_tcp_to_all(&mut self, message: H::ServerMessages, components: &mut ComponentTable) {
        // this gets a little tricky :
        // as if connections fail, we have to disconnect the client, use retain method to loop through all values
        // and use the retain closure to send messages, and if fail, do not retain the connection
        let packet = Packet::from(message);
        self.connections.retain(|id, connection| {
            match connection.tcp_connection.write(&packet.as_new_bytes()) {
                Ok(_amount_written) => true, // all good
                Err(e) => {
                    println!("[NETWORK SERVER] -> Error while sending data to client {id} ({e}). Disconnecting client.");
                    self.server_handler.on_client_disconnected(*id, components);
                    false // don't retain connection
                },
            }
        });
    }

    pub fn send_tcp_to_all_except(&mut self, except: usize, message: H::ServerMessages, components: &mut ComponentTable) {
        // this gets a little tricky :
        // as if connections fail, we have to disconnect the client, use retain method to loop through all values
        // and use the retain closure to send messages, and if fail, do not retain the connection
        let packet = Packet::from(message);
        self.connections.retain(|id, connection| {
            match *id == except {
                true => true, // if it's the id of the skipped client, don't send data and retain
                false => match connection.tcp_connection.write(&packet.as_new_bytes()) {
                    Ok(_amount_written) => true, // all good
                    Err(e) => {
                        println!("[NETWORK SERVER] -> Error while sending data to client {id} ({e}). Disconnecting client.");
                        self.server_handler.on_client_disconnected(*id, components);
                        false // don't retain conenction
                    },
                }
            }
        });
    }

    pub fn send_udp_to_client(&mut self, _to: usize, _message: H::ServerMessages, _components: &mut ComponentTable) {
        unimplemented!()
    }

    pub fn send_udp_to_all(&mut self, _message: H::ServerMessages, _components: &mut ComponentTable) {
        unimplemented!()
    }

    pub fn send_udp_to_all_except(&mut self, _except: usize, _message: H::ServerMessages, _components: &mut ComponentTable) {
        unimplemented!()
    }

}

impl<H: ServerHandler + 'static> Updatable for Server<H> {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, _user_data: &mut dyn std::any::Any) {
        
        // create a vec of any incoming messages to send
        let mut to_send_messages = Vec::new();
        // process any incoming requests
        loop {
            match self.tcp_listener.accept() {
                Ok((stream, adress)) => {
                    to_send_messages.append(&mut self.handle_incoming_connection(stream, adress, components));
                },
                Err(_e) => break, // no more incoming connections for now
            }
        }


        // process incoming messages
        for (_client_id, connection) in self.connections.iter_mut() {
            // look for incoming Connection messages.
            // At the end of this, the Connection buffer should be empty
            for packet in match connection.get_incoming_packets() {
                Ok(packets) => packets,
                Err(_e) => {
                    // todo : better handling. For now, let's disconnect the client ?
                    self.server_handler.on_client_disconnected(connection.id, components);

                    continue;
                },
            } {
                match packet.into() {
                    Ok(data) => to_send_messages.append(&mut self.server_handler.handle_tcp_message(connection.id, data, components)),
                    Err(_e) => {} // todo :error handling
                }
                
            }
        }

        // user update
        to_send_messages.append(&mut self.server_handler.update(components, delta));


        // send all messages we got from our diverse calls
        for return_message in to_send_messages {
            match return_message {
                ServerMessage::TcpToClient(client_id, message) => self.send_tcp_to_client(client_id, message, components),
                ServerMessage::TcpToAll(message) => self.send_tcp_to_all(message, components),
                ServerMessage::TcpToExcept(except_id, message) => self.send_tcp_to_all_except(except_id, message, components),
                _ => {},
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

pub enum ServerMessage<E: NetworkSerializable> {
    TcpToClient(usize, E),
    TcpToAll(E),
    TcpToExcept(usize, E),
    UdpToClient(usize, E),
    UdpToAll(E),
    UdpToExcept(usize, E),
}

pub trait ServerHandler where Self: Sized {
    type ServerMessages: NetworkSerializable;
    type ClientsMessages: NetworkSerializable;
    fn on_client_connected(&mut self, client: usize, components: &mut ComponentTable) -> Vec<ServerMessage<Self::ServerMessages>>;
    fn on_client_disconnected(&mut self, client: usize, components: &mut ComponentTable) -> Vec<ServerMessage<Self::ServerMessages>>;
    fn update(&mut self, components: &mut ComponentTable, delta: f32) -> Vec<ServerMessage<Self::ServerMessages>>;
    fn handle_tcp_message(&mut self, client: usize, message: Self::ClientsMessages, components: &mut ComponentTable) -> Vec<ServerMessage<Self::ServerMessages>>;
}


/// Server representation of a Client
pub struct Connection {
    id: usize,
    tcp_connection: TcpStream,
    _udp_connection: UdpSocket,
    incoming_packet: Option<Packet>,
    buffer: NetworkBuffer,
}

impl Connection {
    pub fn new(id: usize, tcp_connection: TcpStream) -> Result<Connection, std::io::Error> {
        let udp = UdpSocket::bind(tcp_connection.local_addr()?)?;
        udp.connect(tcp_connection.peer_addr()?)?;
        Ok(Connection { 
            id: id,
            tcp_connection,
            _udp_connection: udp,
            incoming_packet: None,
            buffer: NetworkBuffer::new(),
        })
    }

    pub fn get_incoming_packets(&mut self) -> Result<Vec<Packet>, std::io::Error> {
        self.get_incoming_udp()
    }

    fn get_incoming_udp(&mut self) -> Result<Vec<Packet>, std::io::Error> {
        // start by reading if there are any incoming data
        if self.buffer.read_tcp(&mut self.tcp_connection)? {
            // let's read !
            // check if there is an unfinished packet to read
            let mut result = Vec::new();
            if let Some(mut packet) = self.incoming_packet.take() {
                // try complete the packet
                if self.buffer.try_complete_packet(&mut packet) {
                    result.push(packet);
                }
            }
            loop {
                let new_packet = self.buffer.try_read_packet();
                match new_packet {
                    Some(packet) => {
                        if packet.awaiting_size() == 0 {
                            // packet complete, keep getting more !
                            result.push(packet)
                        }
                        else {
                            // put the packet as awaiting and break
                            self.incoming_packet = Some(packet);
                            break;
                        }
                    },
                    None => break,
                }
            }
            Ok(result)
        }
        else {
            // no new messages.
            Ok(Vec::new())
        }
    }
}