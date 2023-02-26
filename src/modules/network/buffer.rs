use std::{net::TcpStream, io::Read, mem::size_of};
use super::packet::Packet;

/// Wrapper arround a u8 buffer to ease reading on tcp streams
pub struct NetworkBuffer{
    buffer: [u8; 256],
    read_pointer: usize, // could use u8 but makes type coercion forced insome functions
    remaining_data_size: usize,
}

impl NetworkBuffer {
    pub fn new() -> NetworkBuffer {
        NetworkBuffer { buffer: [0; 256], read_pointer: 0, remaining_data_size: 0 }
    }

    /// returns true if there was any new data to read
    pub fn read_tcp(&mut self, stream: &mut TcpStream) -> Result<bool, std::io::Error> {
        self.remaining_data_size = stream.read(&mut self.buffer)?; // error propagation
        // new data, so reset the read pointer
        self.read_pointer = 0;
        Ok(self.remaining_data_size != 0) // send true if we managed to read any data
    }

    pub fn get_usize(&mut self) -> Result<usize, ()> {
        if size_of::<usize>() > self.remaining_data_size {
            Err(())
        }
        else {
            // we are assured it works because we checked the size 
            let buf: [u8; size_of::<usize>()] = self.buffer[self.read_pointer..self.read_pointer+size_of::<usize>()].try_into().unwrap();
            self.read_pointer += 8;
            self.remaining_data_size -= 8;
            Ok(usize::from_le_bytes(buf))
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
            let size = self.get_usize().unwrap();
            let mut packet = Packet::from_header(size);
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