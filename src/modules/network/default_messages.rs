use crate::{NetworkSerializable, NetworkUnserializeError};


#[derive(NetworkSerializable)]
pub enum DefaultNetworkMessages {
    Welcome(u64), // Server -> client / client id
    Disconnecting, // client -> server
}
