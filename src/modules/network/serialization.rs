pub enum NetworkUnserializeError {
    InvalidId,
    IncompleteData,
}

// todo : use only u8 slices and arrays, as vectors allocate memory and we don't need it
pub trait NetworkSerializable {
    fn size(&self) -> usize;
    fn serialize(self) -> Vec<u8>;
    fn deserialize(data: Vec<u8>) -> Result<Self, NetworkUnserializeError> where Self: Sized;
}



// As the NetworkSerializable rely on itself to be implemented on enums, 
// let's implement it on all default data types. 

macro_rules! implement_network_serialization_for {
    ($type:ident) => {
        impl NetworkSerializable for $type {
            fn size(&self) -> usize {
                std::mem::size_of::<Self>()
            }
            fn serialize(self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
            fn deserialize(data: Vec<u8>) -> Result<Self, NetworkUnserializeError> {
                match TryInto::<[u8; std::mem::size_of::<Self>()]>::try_into(data) {
                    Ok(val) => Ok(Self::from_le_bytes(val)),
                    Err(_data) => Err(NetworkUnserializeError::IncompleteData)
                }
            }
        }
    }
}

implement_network_serialization_for!(f32);
implement_network_serialization_for!(f64);
implement_network_serialization_for!(u8);
implement_network_serialization_for!(u16);
implement_network_serialization_for!(u32);
implement_network_serialization_for!(u64);
implement_network_serialization_for!(usize);
implement_network_serialization_for!(i8);
implement_network_serialization_for!(i16);
implement_network_serialization_for!(i32);
implement_network_serialization_for!(i64);


impl NetworkSerializable for bool {
    fn size(&self) -> usize {
        1 // bool is 1 byte
    }
    fn serialize(self) -> Vec<u8> {
        match self {
            true => vec![255],
            false => vec![0],
        }
    }
    fn deserialize(data: Vec<u8>) -> Result<Self, NetworkUnserializeError> {
        match TryInto::<[u8; 1]>::try_into(data) {
            Ok(val) => {
                if val[0] >= 127 { // todo : actually count the number of one's and use this ? but it's safe enough
                    Ok(true)
                }
                else {
                    Ok(false)
                }
            },
            Err(_data) => Err(NetworkUnserializeError::IncompleteData)
        }
    }
}
