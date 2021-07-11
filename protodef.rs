use std::collections::HashMap;

// #[macro_use]
macro_rules! scope {
    ($var:ident, $val: expr, $post: expr) => {{
        let $var = Protodef::Void();
        let mut $var = $val;
        $post;
        $var
    }};
}

fn main() {
    println!("Hello, world!");
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3];

    println!("{:?}", try_parse(&mut &vec[..]));
}

fn try_parse(input: &mut &[u8]) -> Option<Protodef> {
    Some(scope!(root_0, Protodef::new_object(), {
        root_0.set("bob", Protodef::new_object());
        root_0
            .get_mut("bob")?
            .set("array", Protodef::parse_f32(input)?);
        root_0
            .get_mut("bob")?
            .set("bobarray", Protodef::parse_varint(input)?);
    }))
}

#[derive(Debug)]
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
macro_rules! parse_num {
    {$id: ident, $real: ident, $count: ident, $name: ident} => {
        pub fn $name(input: &mut &[u8]) -> Option<Self> {
            let bytes = get_bits::$count(input)?;
            Some(Self::$id($real::from_be_bytes(bytes)))
        }
    };
}

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

#[allow(dead_code)]
impl Protodef {
    //Unsigned integers:
    parse_num! {Uint8, u8, read_8, parse_u8}
    parse_num! {Uint16, u16, read_16, parse_u16}
    parse_num! {Uint32, u32, read_32, parse_u32}
    parse_num! {Uint64, u64, read_64, parse_u64}
    serial_num! {Uint8, serial_u8}
    serial_num! {Uint16, serial_u16}
    serial_num! {Uint32, serial_u32}
    serial_num! {Uint64, serial_u64}
    //Signed integers:
    parse_num! {Int8, i8, read_8, parse_i8}
    parse_num! {Int16, i16, read_16, parse_i16}
    parse_num! {Int32, i32, read_32, parse_i32}
    parse_num! {Int64, i64, read_64, parse_i64}
    serial_num! {Int8, serial_i8}
    serial_num! {Int16, serial_i16}
    serial_num! {Int32, serial_i32}
    serial_num! {Int64, serial_i64}
    //Floating point numbers.
    parse_num! {Float, f32, read_32, parse_f32}
    parse_num! {Double, f64, read_64, parse_f64}
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

mod get_bits {
    pub fn read_8(input: &mut &[u8]) -> Option<[u8; 1]> {
        let byte = input.get(0)?;
        *input = &input[1..];
        Some([*byte])
    }
    pub fn read_16(input: &mut &[u8]) -> Option<[u8; 2]> {
        let bytes = input.get(..2)?;
        *input = &input[2..];
        Some([bytes[0], bytes[1]])
    }
    pub fn read_32(input: &mut &[u8]) -> Option<[u8; 4]> {
        let bytes = input.get(..4)?;
        let mut array = [0; 4];
        array.copy_from_slice(bytes);
        *input = &input[4..];
        Some(array)
    }
    pub fn read_64(input: &mut &[u8]) -> Option<[u8; 8]> {
        let bytes = input.get(..8)?;
        let mut array = [0; 8];
        array.copy_from_slice(bytes);
        *input = &input[8..];
        Some(array)
    }
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
    pub fn set(&mut self, field: &str, value: Self) -> Option<()> {
        match self {
            Self::Object(x) => {
                x.insert(field.to_string(), value);
                Some(())
            }
            _ => None,
        }
    }
}
