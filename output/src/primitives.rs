#![allow(dead_code)]
use super::Protodef;

//bool, cstring, (void).

pub mod types {
    use super::*;
    pub mod bool {
        use super::*;
        pub fn parse(input: &mut &[u8]) -> Option<Protodef> {
            let byte = *input.get(0)?;
            *input = &input[1..];
            match byte {
                0 => Some(Protodef::Bool(false)),
                1 => Some(Protodef::Bool(true)),
                _ => None,
            }
        }
        pub fn serial(data: &Protodef, output: &mut Vec<u8>) -> Option<()> {
            if let Protodef::Bool(value) = data {
                output.push(*value as u8);
                Some(())
            } else {
                None
            }
        }
    }
    pub mod cstring {
        use super::*;
        pub fn parse(input: &mut &[u8]) -> Option<Protodef> {
            let till = input.iter().position(|&x| x == 0)?;
            let data = input[..till].to_vec();
            *input = &input[till + 1..];
            Some(Protodef::String(String::from_utf8(data).ok()?))
        }
        pub fn serial(data: &Protodef, output: &mut Vec<u8>) -> Option<()> {
            if let Protodef::String(value) = data {
                output.extend_from_slice(value.as_bytes());
                output.push(0);
                Some(())
            } else {
                None
            }
        }
    }
}
