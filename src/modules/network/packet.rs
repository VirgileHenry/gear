use std::mem::size_of;
use crate::NetworkSerializable;

/// Packet errors
pub enum PacketError {
    /// Packet could not be converted into the desired data type.
    InvalidForConversion,
    /// Attempt to put too much data in the packet for it's remaining size.
    DataOverflow,
}

/// Packet header is of known size and allow to tell us how many bytes we are expecting on connection.
struct PacketHeader {
    size: usize,
}

/// a packet is the data that is send through the network.
/// It can be build from many things and can carry any data.
pub struct Packet {
    header: PacketHeader,
    body: Vec<u8>,
}


impl Packet {
    /// Creates a packet from any data implementing the `NetworkSerializable` trait. 
    pub fn from<S: NetworkSerializable>(from: S) -> Packet {
        Packet {
            header: PacketHeader { size: from.size() },
            body: from.serialize(),
        }
    }

    /// Creates a packet with only a header as valid data. The packet must be completed before use. 
    pub fn from_header(size: usize) -> Packet {
        Packet {
            header: PacketHeader { size: size },
            body: Vec::with_capacity(size),
        }
    }

    /// Add data to a packet.
    /// returns an error if the packet overflowed (too many data was pushed into it) and won't add the data.
    /// Otherwise, returns true if the packet is full (and ready to be unserialized)
    pub fn push_data(&mut self, data: &mut Vec<u8>) -> Result<bool, PacketError> {
        match self.body.capacity().cmp(&(self.body.len() + data.len())) {
            std::cmp::Ordering::Less => return Err(PacketError::DataOverflow),
            std::cmp::Ordering::Equal => {
                self.body.append(data);
                return Ok(true);
            },
            std::cmp::Ordering::Greater => {
                self.body.append(data);
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
        let mut result = Vec::with_capacity(self.header.size + size_of::<PacketHeader>());
        result.append(&mut self.header.size.to_le_bytes().to_vec());
        result.append(&mut self.body);
        result
    }

    /// Convert the packet to a byte array without consuming it. This might disappear when packet implement copy trait.
    pub fn as_new_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.header.size + size_of::<PacketHeader>());
        result.append(&mut self.header.size.to_le_bytes().to_vec());
        result.append(&mut self.body.repeat(1)); // a little wanky ? copy the body but not super efficient ?
        result
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
        size_of::<PacketHeader>()
    }
}

