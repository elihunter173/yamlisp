// Just required to implement serialize and deserialize
use lexpr::Value;

pub mod ser;

pub struct LispIR<'a>(pub &'a Value);
