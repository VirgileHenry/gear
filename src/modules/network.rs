mod server;
mod client;
mod packet;
mod buffer;
mod serialization;
mod default_messages;

pub use server::*;
pub use client::*;
pub use packet::*;
pub use serialization::*;
pub(crate) use default_messages::*;
/*
A lot of network code is a first implementation, and could be refactored in a better way.

To explain a little :
A Server is a system for a server,
A client is a game system for a client,
packets are what are sent through the network. Even if what is really sent are u8 arrays, 
packets are nice wrappers around those to easely create, write, read, parse them from any struct implementing :
NetworkSerializable is a trait on every object that need to be sent through the network.
network buffer is a wrapper arround a u8 buffer to easely read packets out of raw io streams
That's it ! to send data, it goes :

struct / enum -> packet -> (client / server).send(packet) -> OS AND RAW CONNECTION
OS AND RAW CONNECTION -> (client / server).handle -> network buffer -> packet -> struct / enum
*/