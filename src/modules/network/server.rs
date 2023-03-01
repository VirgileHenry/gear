use std::{
    collections::HashMap,
    net::{
        UdpSocket, TcpStream, TcpListener, SocketAddr,
    },
    io::{Write},
};

use foundry::*;

use crate::{NetworkSerializable, DefaultNetworkMessages};

use super::{packet::Packet, buffer::{TcpBuffer, UdpBuffer}};

/// Server system. When created, will start to listen tcp connections and create Connection when receiving them.
/// H is the server handler
pub struct Server<H: ServerHandler> {
    server_handler: H,
    connections: HashMap<u64, Connection>,
    max_client_count: u64,
    current_client_count: u64,
    next_available_id: u64,
    tcp_listener: TcpListener,
    udp_socket: UdpSocket,
}

impl<H: ServerHandler> Server<H> {
    pub fn new(server_handler: H, port: u64, max_client_count: u64) -> std::io::Result<Server<H>> {
        Ok(Server { 
            server_handler: server_handler,
            connections: HashMap::new(),
            max_client_count: max_client_count,
            current_client_count: 0,
            next_available_id: 1,
            tcp_listener: {
                let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?; // todo : ip adress management
                listener.set_nonblocking(true)?;
                listener
            },
            udp_socket: {
                let socket = UdpSocket::bind(format!("0.0.0.0:{port}"))?;
                socket.set_nonblocking(true)?;
                socket
            },
        })
    }

    pub fn handle_incoming_connection(&mut self, mut stream: TcpStream, adress: SocketAddr, components: &mut ComponentTable) -> Vec<ServerMessage<H::ServerMessages>>{
        println!("[NETWORK SERVER] -> incoming connection : {adress}.");
        if self.current_client_count < self.max_client_count {
            println!("[NETWORK SERVER] -> accepted connection as Connection {}.", self.next_available_id);
            let client = match Connection::new(self.next_available_id, stream) {
                Ok(client) => {
                    client
                },
                Err(e) => {
                    println!("[NETWORK SERVER] -> Error while handling incoming connection : {e}.");
                    return Vec::new();
                }
            };
            // send the welcome message
            let result = self.server_handler.on_client_connected(client.id, components);
            self.connections.insert(self.next_available_id, client);
            self.send_default_message(self.next_available_id, DefaultNetworkMessages::Welcome(self.next_available_id), components);
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
    fn disconnect_client(&mut self, client: u64, components: &mut ComponentTable) {
        match self.connections.remove(&client) {
            Some(_) => println!("[NETWORK SERVER] -> Disconnected client {client}."),
            None => println!("[NETWORK SERVER] -> Unable to disconnect client {client} : not registered."),
        }
        self.server_handler.on_client_disconnected(client, components);
    }

    pub fn send_default_message(&mut self, to: u64, message: DefaultNetworkMessages, components: &mut ComponentTable) {
        let bytes = Packet::from_default(message, 0).as_bytes();
        match self.connections.get_mut(&to) {
            Some(client) => {
                match client.tcp_connection.write(&bytes) {
                    Ok(_amount_written) => { /* all good */}, // todo : check amount written is enough, or re-write
                    Err(e) => {
                        println!("[NETWORK SERVER] -> Error while sending data to client {to} ({e}). Disconnecting client {to}.");
                        self.disconnect_client(to, components);
                    },
                };
            },
            None => println!("[NETWORK SERVER] -> Attempted to send packet to Connection {to} but Connection was not found."),
        };
    }

    pub fn send_tcp_to_client(&mut self, to: u64, message: H::ServerMessages, components: &mut ComponentTable) {
        let packet = Packet::from(message, 0);
        match self.connections.get_mut(&to) {
            Some(client) => {
                match client.tcp_connection.write(&packet.as_bytes()) {
                    Ok(_amount_written) => {/* all good */}, // todo : check amount written is enough, or re-write
                    Err(e) => {
                        println!("[NETWORK SERVER] -> Error while sending data to client {to} ({e}). Disconnecting client {to}.");
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
        let packet = Packet::from(message, 0);
        let bytes = packet.as_bytes();
        self.connections.retain(|id, connection| {
            match connection.tcp_connection.write(&bytes) {
                Ok(_amount_written) => true, // all good
                Err(e) => {
                    println!("[NETWORK SERVER] -> Error while sending data to client {id} ({e}). Disconnecting client {id}.");
                    self.server_handler.on_client_disconnected(*id, components);
                    false // don't retain connection
                },
            }
        });
    }

    pub fn send_tcp_to_all_except(&mut self, except: u64, message: H::ServerMessages, components: &mut ComponentTable) {
        // this gets a little tricky :
        // as if connections fail, we have to disconnect the client, use retain method to loop through all values
        // and use the retain closure to send messages, and if fail, do not retain the connection
        let packet = Packet::from(message, 0);
        let bytes = packet.as_bytes();
        self.connections.retain(|id, connection| {
            match *id == except {
                true => true, // if it's the id of the skipped client, don't send data and retain
                false => match connection.tcp_connection.write(&bytes) {
                    Ok(_amount_written) => true, // all good
                    Err(e) => {
                        println!("[NETWORK SERVER] -> Error while sending data to client {id} ({e}). Disconnecting client {id}.");
                        self.server_handler.on_client_disconnected(*id, components);
                        false // don't retain conenction
                    },
                }
            }
        });
    }

    pub fn send_udp_to_client(&mut self, to: u64, message: H::ServerMessages, components: &mut ComponentTable) {
        let packet = Packet::from(message, 0);
        match self.connections.get_mut(&to) {
            Some(client) => {
                match client.tcp_connection.peer_addr() {
                    Ok(addr) => {
                        // send data 
                        match self.udp_socket.send_to(&packet.as_bytes(), addr) {
                            Ok(_) => {/* all good */},
                            Err(e) => {
                                println!("[NETWORK SERVER] -> Error while sending data to client {to} ({e}). Disconnecting client {to}.");
                                self.disconnect_client(to, components);
                            },
                        }
                    },
                    Err(_) => {
                        println!("[NETWORK SERVER] -> Unable to send udp to client {to} (Unable to find address). Disconnecting client {to}");
                        self.disconnect_client(to, components);
                    },
                };
            },
            None => println!("[NETWORK SERVER] -> Attempted to send packet to Connection {to} but Connection was not found."),
        };
    }

    pub fn send_udp_to_all(&mut self, message: H::ServerMessages, components: &mut ComponentTable) {
        // this gets a little tricky :
        // as if connections fail, we have to disconnect the client, use retain method to loop through all values
        // and use the retain closure to send messages, and if fail, do not retain the connection
        let packet = Packet::from(message, 0);
        let bytes = packet.as_bytes();
        self.connections.retain(|id, connection| {
            match connection.tcp_connection.peer_addr() {
                Ok(addr) => {
                    // send data 
                    match self.udp_socket.send_to(&bytes, addr) {
                        Ok(_) => true,
                        Err(e) => {
                            println!("[NETWORK SERVER] -> Error while sending data to client {id} ({e}). Disconnecting client {id}.");
                            self.server_handler.on_client_disconnected(*id, components);
                        false // don't retain connection
                        },
                    }
                },
                Err(_) => {
                    println!("[NETWORK SERVER] -> Unable to send udp to client {id} (Unable to find address). Disconnecting client {id}");
                    self.server_handler.on_client_disconnected(*id, components);
                        false // don't retain connection
                },
            }
        });
    }

    pub fn send_udp_to_all_except(&mut self, except: u64, message: H::ServerMessages, components: &mut ComponentTable) {
        // this gets a little tricky :
        // as if connections fail, we have to disconnect the client, use retain method to loop through all values
        // and use the retain closure to send messages, and if fail, do not retain the connection
        let packet = Packet::from(message, 0);
        let bytes = packet.as_bytes();
        self.connections.retain(|id, connection| {
            match *id == except {
                true => true, // if it's the id of the skipped client, don't send data and retain
                false => match connection.tcp_connection.peer_addr() {
                    Ok(addr) => {
                        // send data 
                        match self.udp_socket.send_to(&bytes, addr) {
                            Ok(_) => true,
                            Err(e) => {
                                println!("[NETWORK SERVER] -> Error while sending data to client {id} ({e}). Disconnecting client {id}.");
                                self.server_handler.on_client_disconnected(*id, components);
                            false // don't retain connection
                            },
                        }
                    },
                    Err(_) => {
                        println!("[NETWORK SERVER] -> Unable to send udp to client {id} (Unable to find address). Disconnecting client {id}");
                        self.server_handler.on_client_disconnected(*id, components);
                            false // don't retain connection
                    },
                }
            }
        });
    }

    fn handle_default(&mut self, client: u64, default_message: DefaultNetworkMessages, components: &mut ComponentTable) {
        match default_message {
            DefaultNetworkMessages::Disconnecting => {
                // disconnect client
                self.disconnect_client(client, components);
            }
            DefaultNetworkMessages::Welcome(_) => {}, // this is a server -> client message
        }
    }

    fn get_incoming_udp(&mut self) -> Result<Vec<(u64, Packet)>, std::io::Error> {
        let mut buffer = UdpBuffer::new();
        let mut result = Vec::new();
        while let Some((packet, from)) = buffer.read_tcp(&mut self.udp_socket)? {
            // check the packet is sent by correct user
            match self.connections.get(&packet.get_sender()) {
                Some(connection) => {
                    if connection.peer_addr()?.ip() == from.ip() {
                        result.push((packet.get_sender(), packet));
                    }
                    else {
                        println!("[NETWORK SERVER] -> Packet was signed by id of another client !");
                    }
                }
                None => println!("[NETWORK SERVER] -> Packet was signed by unregistered id !"),
            }
        };
        Ok(result)
    }

}

impl<H: ServerHandler + 'static> Updatable for Server<H> {
    fn update(&mut self, components: &mut ComponentTable, delta: f32, _user_data: &mut dyn std::any::Any) {
        
        // create a vec of any incoming messages to send
        let mut to_send_messages = Vec::new();
        let mut default_messages = Vec::new();
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
        let mut disconnecting = Vec::with_capacity(self.connections.len());
        for (client_id, connection) in self.connections.iter_mut() {
            // look for incoming Connection messages.
            // At the end of this, the Connection buffer should be empty
            for packet in match connection.get_incoming_packets() {
                Ok(packet) => packet,
                Err(e) => {
                    println!("[NETWOTK SERVER] -> Error while receiving packets ({e}) from client {client_id}. Disconnectin client.");
                    disconnecting.push(*client_id);
                    break;
                },
            } {
                match packet.is_default() {
                    true => match packet.into::<DefaultNetworkMessages>() {
                        Ok(message) => default_messages.push((*client_id, message)),
                        Err(_e) => println!("[NETWORK CLIENT] -> Unable to deserialize packet !"),
                    }
                    false => match packet.into() {
                        Ok(data) => to_send_messages.append(&mut self.server_handler.handle_message(*client_id, data, components)),
                        Err(_) => println!("[NETWORK CLIENT] -> Unable to deserialize packet !"),
                    }
                }
            }
        }

        match self.get_incoming_udp() {
            Ok(messages) => for (client, packet) in messages {
                match packet.is_default() {
                    true => match packet.into::<DefaultNetworkMessages>() {
                        Ok(message) => default_messages.push((client, message)),
                        Err(_e) => println!("[NETWORK CLIENT] -> Unable to deserialize packet !"),
                    }
                    false => match packet.into() {
                        Ok(data) => to_send_messages.append(&mut self.server_handler.handle_message(client, data, components)),
                        Err(_) => println!("[NETWORK CLIENT] -> Unable to deserialize packet !"),
                    }
                }
            },
            Err(e) => println!("[GEAR SERVER] -> Error while receiving tcp packets: {e}"),
        };

        // disconnect error clients
        for client_id in disconnecting.into_iter() {
            self.disconnect_client(client_id, components);
        }
        // handle defaults
        for (client_id, message) in default_messages.into_iter() {
            self.handle_default(client_id, message, components)
        }

        // user update
        to_send_messages.append(&mut self.server_handler.update(components, delta));
        


        // send all messages we got from our diverse calls
        for return_message in to_send_messages {
            match return_message {
                ServerMessage::TcpToClient(client_id, message) => self.send_tcp_to_client(client_id, message, components),
                ServerMessage::TcpToAll(message) => self.send_tcp_to_all(message, components),
                ServerMessage::TcpToExcept(except_id, message) => self.send_tcp_to_all_except(except_id, message, components),
                ServerMessage::UdpToClient(client_id, message) => self.send_udp_to_client(client_id, message, components),
                ServerMessage::UdpToAll(message) => self.send_udp_to_all(message, components),
                ServerMessage::UdpToExcept(except_id, message) => self.send_udp_to_all_except(except_id, message, components),
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
    TcpToClient(u64, E),
    TcpToAll(E),
    TcpToExcept(u64, E),
    UdpToClient(u64, E),
    UdpToAll(E),
    UdpToExcept(u64, E),
}

pub trait ServerHandler where Self: Sized {
    type ServerMessages: NetworkSerializable;
    type ClientsMessages: NetworkSerializable;
    fn on_client_connected(&mut self, client: u64, components: &mut ComponentTable) -> Vec<ServerMessage<Self::ServerMessages>>;
    fn on_client_disconnected(&mut self, client: u64, components: &mut ComponentTable) -> Vec<ServerMessage<Self::ServerMessages>>;
    fn update(&mut self, components: &mut ComponentTable, delta: f32) -> Vec<ServerMessage<Self::ServerMessages>>;
    fn handle_message(&mut self, client: u64, message: Self::ClientsMessages, components: &mut ComponentTable) -> Vec<ServerMessage<Self::ServerMessages>>;
}


/// Server representation of a Client
pub struct Connection {
    id: u64,
    tcp_connection: TcpStream,
    tcp_buffer: TcpBuffer,
    incoming_packet: Option<Packet>,
}

impl Connection {
    pub fn new(id: u64, tcp_connection: TcpStream) -> Result<Connection, std::io::Error> {
        let udp = UdpSocket::bind(tcp_connection.local_addr()?)?;
        udp.connect(tcp_connection.peer_addr()?)?;
        udp.set_nonblocking(true)?;
        Ok(Connection { 
            id: id,
            tcp_connection,
            tcp_buffer: TcpBuffer::new(),
            incoming_packet: None,
        })
    }

    pub fn peer_addr(&self) -> Result<SocketAddr, std::io::Error> {
        self.tcp_connection.peer_addr()
    }

    pub fn get_incoming_packets(&mut self) -> Result<Vec<Packet>, std::io::Error> {
         // start by reading if there are any incoming data
         if self.tcp_buffer.read_tcp(&mut self.tcp_connection)? {
            // let's read !
            // check if there is an unfinished packet to read
            let mut result = Vec::new();
            if let Some(mut packet) = self.incoming_packet.take() {
                // try complete the packet
                if self.tcp_buffer.try_complete_packet(&mut packet) {
                    result.push(packet);
                }
            }
            loop {
                let new_packet = self.tcp_buffer.try_read_packet();
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
