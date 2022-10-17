use std::{
    collections::HashMap,
    marker::PhantomData,
    net::{
        UdpSocket, TcpStream, TcpListener, SocketAddr,
    },
};

use foundry::ecs::system::Updatable;


/// M is the message enum 
pub struct Server {
    clients: HashMap<usize, Client>,
    max_client_count: usize,
    current_client_count: usize,
    next_available_id: usize,
    tcp_listener: TcpListener,
}


impl Server {
    pub fn new(max_client_count: usize) -> Result<Server, String> {
        Ok(Server { 
            clients: HashMap::new(),
            max_client_count: max_client_count,
            current_client_count: 0,
            next_available_id: 1,
            tcp_listener: match TcpListener::bind("127.0.0.1:31415") {
                Ok(listener) => {
                    match listener.set_nonblocking(true) {
                        Ok(()) => {},
                        Err(e) => println!("[GEAR ENGINE] -> unable to set tcp listener as non-blocking. This will cause a engine freeze until connections are received ({})", e),
                    };
                    listener
                },
                Err(e) => return Err("Unable to create server : can't bind tcp listener".to_string()),
            },
        })
    }

    pub fn handle_incoming_connection(&mut self, stream: TcpStream, adress: SocketAddr) {
        println!("[SERVER] -> incoming connection : {adress}.");
        if self.current_client_count < self.max_client_count {
            println!("[SERVER] -> accepted connection as client {}.", self.next_available_id);
            self.clients.insert(self.next_available_id, Client::new(self.next_available_id, stream));
            self.current_client_count += 1;
            self.next_available_id += 1;
        }
        else {
            println!("[SERVER] -> rejected connection : server full.");
            stream.shutdown(std::net::Shutdown::Both);
        }

    }

    pub fn send_to<M>(to: usize, message: M) {

    }

}

impl Updatable for Server {
    fn update(&mut self, components: &mut foundry::ecs::component_table::ComponentTable, delta: f32, user_data: &mut dyn std::any::Any) {
        // process any incoming requests
        loop {
            match self.tcp_listener.accept() {
                Ok((stream, adress)) => {
                    self.handle_incoming_connection(stream, adress);
                },
                Err(_e) => break, // no more incoming connections for now
            }
        }

        // process incoming messages

    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}


/// Server representation of a client
pub struct Client {
    id: usize,
    tcp_connection: TcpStream,
    udp_connection: Option<UdpSocket>,
}

impl Client {
    pub fn new(id: usize, connection: TcpStream) -> Client {
        Client { 
            id: id,
            tcp_connection: connection,
            udp_connection: None,
        }
    }
}