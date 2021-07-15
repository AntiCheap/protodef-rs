#![allow(dead_code)]
use super::Protodef;

macro_rules! count_all {
    ($val: expr) => {
        Some(*$val as usize)
    };
}

macro_rules! count_signed {
    ($val: expr) => {{
        if $val.is_negative() {
            None
        } else {
            Some(*$val as usize)
        }
    }};
}

macro_rules! count_float {
    ($val: expr) => {{
        let pos = $val.is_finite() && $val.is_sign_positive();
        if $val.fract() == 0.0 && pos {
            Some(*$val as usize)
        } else {
            None
        }
    }};
}

impl Protodef {
    pub fn as_count(&self) -> Option<usize> {
        match self {
            Protodef::Uint8(val) => count_all!(val),
            Protodef::Uint16(val) => count_all!(val),
            Protodef::Uint32(val) => count_all!(val),
            Protodef::Uint64(val) => count_all!(val),
            Protodef::Int8(val) => count_signed!(val),
            Protodef::Int16(val) => count_signed!(val),
            Protodef::Int32(val) => count_signed!(val),
            Protodef::Int64(val) => count_signed!(val),
            Protodef::Float(val) => count_float!(val),
            Protodef::Double(val) => count_float!(val),
            _ => None,
        }
    }
    pub fn as_string<'a, T>(&'a self, func: fn(x: &str) -> T) -> T {
        match self {
            Protodef::Uint8(val) => func(&val.to_string()[..]),
            Protodef::Uint16(val) => func(&val.to_string()[..]),
            Protodef::Uint32(val) => func(&val.to_string()[..]),
            Protodef::Uint64(val) => func(&val.to_string()[..]),
            Protodef::Int8(val) => func(&val.to_string()[..]),
            Protodef::Int16(val) => func(&val.to_string()[..]),
            Protodef::Int32(val) => func(&val.to_string()[..]),
            Protodef::Int64(val) => func(&val.to_string()[..]),
            Protodef::Float(val) => func(&val.to_string()[..]),
            Protodef::Double(val) => func(&val.to_string()[..]),
            Protodef::Bool(val) => func(&val.to_string()[..]),
            Protodef::String(val) => func(&val),
            _ => func(""),
        }
    }
}

//Javascript stores numbers as double precision floats.
//Integers and unsigned up to 32 bits can be represented.
//Types needing extra check: Uint64, Int64, Float, Double, bool.
