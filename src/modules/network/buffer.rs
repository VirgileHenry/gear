use std::{net::{TcpStream, UdpSocket, SocketAddr}, io::Read, mem::size_of};
use super::packet::Packet;

/// Wrapper arround a u8 buffer to ease reading on tcp streams
pub struct TcpBuffer{
    buffer: [u8; 256],
    read_pointer: usize, // could use u8 but makes type coercion forced insome functions
    remaining_data_size: usize,
}

impl TcpBuffer {
    pub fn new() -> TcpBuffer {
        TcpBuffer { buffer: [0; 256], read_pointer: 0, remaining_data_size: 0 }
    }

    /// returns true if there was any new data to read
    pub fn read_tcp(&mut self, stream: &mut TcpStream) -> Result<bool, std::io::Error> {
        self.remaining_data_size = match stream.read(&mut self.buffer) {
            Ok(data_size) => data_size,
            Err(e) => {
                // ! non blocking sockets can throw errors if they found nothing. Prevent this error, throw the rest
                match e.kind() {
                    std::io::ErrorKind::WouldBlock => return Ok(false),
                    _ => return Err(e),
                }
            }
        };
        // new data, so reset the read pointer
        self.read_pointer = 0;
        Ok(self.remaining_data_size != 0) // send true if we managed to read any data
    }

    pub fn get_bool(&mut self) -> Result<bool, ()> {
        if size_of::<bool>() > self.remaining_data_size {
            Err(())
        }
        else {
            let result = self.buffer[self.read_pointer] != 0;
            self.read_pointer += size_of::<bool>();
            self.remaining_data_size -= size_of::<bool>();
            Ok(result)
        }
    }

    pub fn get_u64(&mut self) -> Result<u64, ()> {
        if size_of::<u64>() > self.remaining_data_size {
            Err(())
        }
        else {
            // we are assured it works because we checked the size 
            let buf: [u8; size_of::<u64>()] = self.buffer[self.read_pointer..self.read_pointer+size_of::<u64>()].try_into().unwrap();
            self.read_pointer += size_of::<u64>();
            self.remaining_data_size -= size_of::<u64>();
            Ok(u64::from_le_bytes(buf))
        }
    }

    /// returns true if the packet is complete, false otherwise. 
    /// If false is returned, the buffer is empty
    pub fn try_complete_packet(&mut self, packet: &mut Packet) -> bool {
        let read_amount = std::cmp::min(packet.awaiting_size(), self.remaining_data_size);
        match packet.push_data(&mut self.buffer[self.read_pointer..self.read_pointer + read_amount].to_vec()) {
            Ok(_packet_finished) => { /* all good */},
            Err(_e) => { /* todo : error handling */} // todo : better way
        }
        self.read_pointer += read_amount;
        self.remaining_data_size -= read_amount;
        packet.awaiting_size() == 0
    }

    /// try to create a new packet from the buffer
    /// returns true if there is remaining data to read
    /// returns a packet if we managed to create one (not necessarely complete)
    pub fn try_read_packet(&mut self) -> Option<Packet> {
        // check if we can read a header
        if self.remaining_data_size >= Packet::header_size() {
            // we can at least parse a header !
            // we can unwrap safely as we know there is enough space (we measured)
            let is_default = self.get_bool().unwrap();
            let size = self.get_u64().unwrap();
            let sender = self.get_u64().unwrap();
            let mut packet = Packet::from_header(size, sender, is_default);
            // check if we can read the packet !
            self.try_complete_packet(&mut packet);
            // if the packet is complete, there might be remaining data !
            // otherwise, there is not because we read it all to try completing the packet
            Some(packet)
        }
        else {
            // nothing to do
            None
        }
    }

}

/// Wrapper arround a u8 buffer to ease reading on udp sockets
pub struct UdpBuffer{
    buffer: [u8; 256],
    read_pointer: usize,
}

impl UdpBuffer {
    pub fn new() -> UdpBuffer {
        UdpBuffer {
            buffer: [0u8; 256],
            read_pointer: 0
        }
    }
    /// returns true if there was any new data to read
    pub fn read_tcp(&mut self, socket: &mut UdpSocket) -> Result<Option<(Packet, SocketAddr)>, std::io::Error> {
        let data_size = match socket.recv_from(&mut self.buffer) {
            Ok(data_size) => data_size,
            Err(e) => {
                // ! non blocking sockets can throw errors if they found nothing. Prevent this error, throw the rest
                match e.kind() {
                    std::io::ErrorKind::WouldBlock => return Ok(None),
                    _ => return Ok(None), // todo : silence errors for now, maybe treat them later ?
                }
            }
        };
        self.read_pointer = 0; // reset read pointers
        match data_size.0 > 0 {
            false => Ok(None),
            true => {
                let is_default = self.get_bool();
                let size = self.get_u64()?;
                let sender = self.get_u64()?;
                let mut result_packet = Packet::from_header(size, sender, is_default);
                match result_packet.push_data(&self.buffer[Packet::header_size()..data_size.0]) {
                    Ok(complete) => if complete {
                        Ok(Some((result_packet, data_size.1)))
                    }
                    else {
                        println!("couldn't finish udp packet : throw it away");
                        Ok(None) // it's udp, so packets come all at once. no chance for cathcing up !
                    }
                    Err(e) => {
                        println!("Error while building udp packet : {e:?}");
                        Ok(None)
                    },
                }
            }
        }

    }


// TODO : rewrite both these func with serialization trait ?
    fn get_bool(&mut self) -> bool {
        match self.buffer.get(self.read_pointer) {
            Some(val) => {
                self.read_pointer += size_of::<bool>();
                *val != 0
            },
            None => false,
        }
    }

    fn get_u64(&mut self) -> Result<u64, std::io::Error> {
        let value = u64::from_le_bytes(match self.buffer[self.read_pointer..self.read_pointer+size_of::<u64>()].try_into() {
            Ok(val) => val,
            Err(e) => return Err( std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        });
        self.read_pointer += size_of::<u64>();
        Ok(value)
    }
}
