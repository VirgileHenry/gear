use std::mem::size_of;

use crate::{NetworkSerializable, DefaultNetworkMessages};

/// Packet errors
#[derive(Debug)]
pub enum PacketError {
    /// Packet could not be converted into the desired data type.
    InvalidForConversion,
    /// Attempt to put too much data in the packet for it's remaining size.
    DataOverflow,
}

/// Packet header is of known size and allow to tell us how many bytes we are expecting on connection.
#[derive(Clone)]
#[repr(C)]
struct PacketHeader {
    is_default: bool,
    size: u64,
    sender_id: u64,
}

/// a packet is the data that is send through the network.
/// It can be build from many things and can carry any data.
#[derive(Clone)]
#[repr(C)]
pub struct Packet {
    header: PacketHeader,
    pub body: Vec<u8>,
}


impl Packet {
    /// Creates a packet from any data implementing the `NetworkSerializable` trait. 
    /// If this is sent from the server, the sender does not matter.
    pub fn from<S: NetworkSerializable>(from: S, sender: u64) -> Packet {
        Packet {
            header: PacketHeader {
                is_default: false,
                size: from.size() as u64,
                sender_id: sender,
            },
            body: from.serialize(),
        }
    }

    /// Creates a packet from the default network message enum. 
    /// If this is sent from the server, the sender does not matter.
    pub fn from_default(from: DefaultNetworkMessages, sender: u64) -> Packet {
        Packet {
            header: PacketHeader {
                is_default: true,
                size: from.size() as u64,
                sender_id: sender,
            },
            body: from.serialize(),
        }
    }

    /// Creates a packet with only a header as valid data. The packet must be completed before use. 
    pub fn from_header(size: u64, sender_id: u64, is_default: bool) -> Packet {
        Packet {
            header: PacketHeader {
                is_default,
                size,
                sender_id,
            },
            body: Vec::with_capacity(size as usize),
        }
    }

    /// Add data to a packet.
    /// returns an error if the packet overflowed (too many data was pushed into it) and won't add the data.
    /// Otherwise, returns true if the packet is full (and ready to be unserialized)
    pub fn push_data(&mut self, data: &[u8]) -> Result<bool, PacketError> {
        match self.body.capacity().cmp(&(self.body.len() + data.len())) {
            std::cmp::Ordering::Less => return Err(PacketError::DataOverflow),
            std::cmp::Ordering::Equal => {
                for val in data {
                    self.body.push(*val);
                }
                return Ok(true);
            },
            std::cmp::Ordering::Greater => {
                for val in data {
                    self.body.push(*val);
                }
                return Ok(false);
            }
        }
    }

    /// Get how many bytes is missing to complete the packet
    pub fn awaiting_size(&self) -> usize {
        self.body.capacity() - self.body.len()
    }

    /// Convert the packet to a byte array.
    pub fn as_bytes(mut self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.header.size as usize + size_of::<PacketHeader>());
        result.append(&mut vec![self.header.is_default as u8]);
        result.append(&mut self.header.size.to_le_bytes().to_vec());
        result.append(&mut self.header.sender_id.to_le_bytes().to_vec());
        result.append(&mut self.body);
        result
    }

    pub fn is_default(&self) -> bool {
        self.header.is_default
    }

    pub fn get_sender(&self) -> u64 {
        self.header.sender_id
    }
    
    /// Try to convert the packet into the given data type.
    pub fn into<S: NetworkSerializable>(self) -> Result<S, PacketError> {
        match S::deserialize(self.body) {
            Ok(result) => Ok(result),
            Err(_e) => Err(PacketError::InvalidForConversion),
        }
    }

    /// Get the size of a packet header. 
    pub fn header_size() -> usize {
        2 * size_of::<u64>() + size_of::<bool>()
    }
}

