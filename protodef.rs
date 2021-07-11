use std::collections::HashMap;

macro_rules! serial_num {
    {$id: ident, $name: ident} => {
        pub fn $name(data: &Protodef, output: &mut Vec<u8>) -> Option<()> {
            if let Protodef::$id(val) = data {
                output.extend(&val.to_be_bytes());
                Some(())
            } else {
                None
            }
        }
    };
}

macro_rules! parse_num {
    {$id: ident,$real: ident,$count: ident, $name: ident} => {
        pub fn $name(input: &mut &[u8]) -> Option<Self> {
            let bytes = get_bytes::$count(input)?;
            Some(Self::$id($real::from_be_bytes(bytes)))
        }
    };
}

macro_rules! count_all {
    ($val: ident) => {
        Some(*$val as usize)
    };
}

macro_rules! count_signed {
    ($val: ident) => {{
        if $val.is_negative() {
            None
        } else {
            Some(*$val as usize)
        }
    }};
}

macro_rules! count_float {
    ($val: ident) => {{
        let pos = $val.is_finite() && $val.is_sign_positive();
        if $val.fract() == 0.0 && pos {
            Some(*$val as usize)
        } else {
            None
        }
    }};
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Protodef {
    Object(HashMap<String, Protodef>),
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
    Void(),
    Bool(bool),
    String(String),
    Buffer(Vec<u8>),
    Array(Vec<Protodef>),
}

//NUMBERS
#[allow(dead_code)]
impl Protodef {
    //Unsigned integers:
    parse_num! {Uint8, u8, one, parse_u8}
    parse_num! {Uint16, u16, two, parse_u16}
    parse_num! {Uint32, u32,four, parse_u32}
    parse_num! {Uint64, u64, eight, parse_u64}
    serial_num! {Uint8, serial_u8}
    serial_num! {Uint16, serial_u16}
    serial_num! {Uint32, serial_u32}
    serial_num! {Uint64, serial_u64}
    //Signed integers:
    parse_num! {Int8, i8, one, parse_i8}
    parse_num! {Int16, i16, two, parse_i16}
    parse_num! {Int32, i32, four, parse_i32}
    parse_num! {Int64, i64, eight, parse_i64}
    serial_num! {Int8, serial_i8}
    serial_num! {Int16, serial_i16}
    serial_num! {Int32, serial_i32}
    serial_num! {Int64, serial_i64}
    //Floating point numbers.
    parse_num! {Float, f32, four, parse_f32}
    parse_num! {Double, f64, eight, parse_f64}
    serial_num! {Float, serial_f32}
    serial_num! {Double, serial_f64}
    //Variable sized integer.
    pub fn parse_varint(input: &mut &[u8]) -> Option<Self> {
        let mut value: u32 = 0;
        let mut shift: u32 = 0;
        let mut i: usize = 0;
        loop {
            let n = *input.get(i)? as u32;
            if n & 0x80 == 0 {
                if i == 4 && (n & 0xF0 != 0) {
                    return None;
                } else {
                    value |= n << shift;
                    *input = &input[i + 1..];
                    return Some(Self::Int32(value as i32));
                }
            };
            if i == 4 {
                return None;
            }
            value |= (0x7F & n) << shift;
            shift += 7;
            i += 1;
        }
    }
    pub fn serial_varint(data: &Self, output: &mut Vec<u8>) -> Option<()> {
        if let Self::Int32(value) = data {
            let mut val = *value as u32;
            while (val >> 7) != 0 {
                output.push(((val & 0x7F) | 0x80) as u8);
                val = val >> 7;
            }
            output.push(val as u8);
            Some(())
        } else {
            None
        }
    }
}

//PRIMITIVES:
#[allow(dead_code)]
impl Protodef {
    pub fn parse_bool(input: &mut &[u8]) -> Option<Self> {
        let byte = *input.get(0)?;
        *input = &input[1..];
        match byte {
            0 => Some(Self::Bool(false)),
            1 => Some(Self::Bool(true)),
            _ => None,
        }
    }
    pub fn serial_bool(data: &Self, output: &mut Vec<u8>) -> Option<()> {
        if let Self::Bool(value) = data {
            output.push(*value as u8);
            Some(())
        } else {
            None
        }
    }
    pub fn parse_cstring(input: &mut &[u8]) -> Option<Self> {
        let till = input.iter().position(|&x| x == 0)?;
        let data = input[..till].to_vec();
        *input = &input[till + 1..];
        Some(Self::String(String::from_utf8(data).ok()?))
    }
    pub fn serial_cstring(data: &Self, output: &mut Vec<u8>) -> Option<()> {
        if let Self::String(value) = data {
            output.extend_from_slice(value.as_bytes());
            output.push(0);
            Some(())
        } else {
            None
        }
    }
    pub fn parse_void(_: &mut &[u8]) -> Option<Self> {
        Some(Self::Void())
    }
    pub fn serial_void(_: &Self, _: &mut Vec<u8>) -> Option<()> {
        //What a beautiful world.
        Some(())
    }
}

//UTILITY
#[allow(dead_code)]
impl Protodef {
    pub fn from_buffer(input: &mut &[u8], len: usize) -> Option<Self> {
        let buffer = input.get(..len)?.to_vec();
        *input = &input[len..];
        Some(Self::Buffer(buffer))
    }
    pub fn from_pstring(input: &mut &[u8], len: usize) -> Option<Self> {
        let buffer = input.get(..len)?.to_vec();
        *input = &input[len..];
        Some(Self::String(String::from_utf8(buffer).ok()?))
    }
    pub fn back_to_buffer(&self) -> Option<&Vec<u8>> {
        match self {
            Self::Buffer(buf) => Some(buf),
            _ => None,
        }
    }
    pub fn back_to_pstring(&self) -> Option<&[u8]> {
        match self {
            Self::String(txt) => Some(txt.as_bytes()),
            _ => None,
        }
    }
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
            _ => {}
        };
    }
    pub fn as_array(&self) -> Option<&Vec<Self>> {
        match self {
            Self::Array(arr) => Some(arr),
            _ => None,
        }
    }
    pub fn as_count(&self) -> Option<usize> {
        match self {
            Self::Uint8(val) => count_all!(val),
            Self::Uint16(val) => count_all!(val),
            Self::Uint32(val) => count_all!(val),
            Self::Uint64(val) => count_all!(val),
            Self::Int8(val) => count_signed!(val),
            Self::Int16(val) => count_signed!(val),
            Self::Int32(val) => count_signed!(val),
            Self::Int64(val) => count_signed!(val),
            Self::Float(val) => count_float!(val),
            Self::Double(val) => count_float!(val),
            _ => None,
        }
    }
}

mod get_bytes {
    pub fn one(input: &mut &[u8]) -> Option<[u8; 1]> {
        let byte = input.get(0)?;
        *input = &input[1..];
        Some([*byte])
    }
    pub fn two(input: &mut &[u8]) -> Option<[u8; 2]> {
        let bytes = input.get(..2)?;
        *input = &input[2..];
        Some([bytes[0], bytes[1]])
    }
    pub fn four(input: &mut &[u8]) -> Option<[u8; 4]> {
        let bytes = input.get(..4)?;
        let mut array = [0; 4];
        array.copy_from_slice(bytes);
        *input = &input[4..];
        Some(array)
    }
    pub fn eight(input: &mut &[u8]) -> Option<[u8; 8]> {
        let bytes = input.get(..8)?;
        let mut array = [0; 8];
        array.copy_from_slice(bytes);
        *input = &input[8..];
        Some(array)
    }
}
