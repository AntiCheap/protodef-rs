#![macro_use]
#![allow(unused_variables)]

use std::collections::HashMap;

mod reference;

mod numbers;
mod primitives;

mod types {
    use super::*;
    pub use numbers::types::*;
    pub use primitives::types::*;
}

macro_rules! scope {
    ($var:ident, $val: expr, $post: expr) => {{
        let $var = Protodef::Zero();
        let mut $var = $val;
        $post;
        $var
    }};
}

fn main() {}

#[derive(Debug)]
pub enum Protodef {
    Object(HashMap<String, Protodef>),
    Array(Vec<Protodef>),
    Bool(bool),
    Buffer(Vec<u8>),
    String(String),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    Zero(),
}

impl Protodef {
    pub fn new_object() -> Self {
        Self::Object(HashMap::new())
    }
    pub fn new_array() -> Self {
        Self::Array(vec![])
    }
    pub fn get(&self, field: &str) -> Option<&Self> {
        match self {
            Self::Object(x) => x.get(field),
            _ => None,
        }
    }
    pub fn get_mut(&mut self, field: &str) -> Option<&mut Self> {
        match self {
            Self::Object(x) => x.get_mut(field),
            _ => None,
        }
    }
    pub fn set(&mut self, field: &str, value: Self) {
        match self {
            Self::Object(x) => {
                x.insert(field.to_string(), value);
            }
            _ => panic!(),
        }
    }
}
