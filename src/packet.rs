use serde::{Serialize, Deserialize};
use crc::{Crc, CRC_16_IBM_SDLC};
use serde_big_array::BigArray;

pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
pub const MAX_PAYLOAD_SIZE: usize = 4096;

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub uid: u32,
    #[serde(with = "BigArray")]
    pub payload: [u8; MAX_PAYLOAD_SIZE],
    pub payload_len: usize,
    pub payload_type: u8,
    pub version: u8,
    pub checksum: u16,
}

impl Packet {
    pub fn new(uid: u32, payload_type: u8, payload: &[u8]) -> Self {
        let mut buffer = [0u8; MAX_PAYLOAD_SIZE];
        let len = payload.len().min(MAX_PAYLOAD_SIZE);
        buffer[..len].copy_from_slice(&payload[..len]);
        Self {
            uid: uid,
            payload: buffer,
            payload_len: len,
            payload_type,
            version: 0,
            checksum: X25.checksum(&payload[..len]),
        }
    }
    pub fn is_valid(&self) -> bool {
        X25.checksum(&self.payload[..self.payload_len]) == self.checksum
    }
    pub fn serialize<'a>(&self, buffer: &'a mut [u8]) -> Result<usize, Box<dyn std::error::Error>> {
        let size = bincode::serialized_size(self)? as usize;
        if buffer.len() < size {
            return Err("Buffer provided is too small".into());
        }
        bincode::serialize_into(&mut buffer[..size], self)?;
        Ok(size)
    }
    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize(bytes)?)
    }
}