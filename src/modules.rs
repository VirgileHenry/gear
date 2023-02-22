//pub use compute_shader::*; not yet implemented
pub use network::*;

mod network;

pub use network::*;
pub use gear_macros_derive::NetworkSerializable;
