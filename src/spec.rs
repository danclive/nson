//! Spec

pub const F32: u8 = 0x01;
pub const F64: u8 = 0x02;
pub const I32: u8 = 0x03;
pub const I64: u8 = 0x04;
pub const U32: u8 = 0x05;
pub const U64: u8 = 0x06;
pub const STRING: u8 = 0x07;
pub const ARRAY: u8 = 0x08;
pub const MAP: u8 = 0x09;
pub const BOOL: u8 = 0x0A;
pub const NULL: u8 = 0x0B;
pub const BINARY: u8 = 0x0C;
pub const TIMESTAMP: u8 = 0x0D;
pub const ID: u8 = 0x0E;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ElementType {
    F32 = F32,
    F64 = F64,
    I32 = I32,
    I64 = I64,
    U32 = U32,
    U64 = U64,
    String = STRING,
    Array = ARRAY,
    Map = MAP,
    Bool = BOOL,
    Null = NULL,
    Binary = BINARY,
    TimeStamp = TIMESTAMP,
    Id = ID,
}

impl ElementType {
    pub fn from(tag: u8) -> Option<ElementType> {
        Some(match tag {
            F32 => ElementType::F32,
            F64 => ElementType::F64,
            I32 => ElementType::I32,
            I64 => ElementType::I64,
            U32 => ElementType::U32,
            U64 => ElementType::U64,
            STRING => ElementType::String,
            ARRAY => ElementType::Array,
            MAP => ElementType::Map,
            BOOL => ElementType::Bool,
            NULL => ElementType::Null,
            BINARY => ElementType::Binary,
            TIMESTAMP => ElementType::TimeStamp,
            ID => ElementType::Id,
            _ => return None,
        })
    }
}
