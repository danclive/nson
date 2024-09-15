//! Spec

pub const BOOL: u8 = 0x01;
pub const NULL: u8 = 0x02;
pub const F32: u8 = 0x11;
pub const F64: u8 = 0x12;
pub const I32: u8 = 0x13;
pub const I64: u8 = 0x14;
pub const U32: u8 = 0x15;
pub const U64: u8 = 0x16;
pub const STRING: u8 = 0x21;
pub const BINARY: u8 = 0x22;
pub const ARRAY: u8 = 0x31;
pub const MAP: u8 = 0x32;
pub const TIMESTAMP: u8 = 0x41;
pub const ID: u8 = 0x42;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ElementType {
    Bool = BOOL,
    Null = NULL,
    F32 = F32,
    F64 = F64,
    I32 = I32,
    I64 = I64,
    U32 = U32,
    U64 = U64,
    String = STRING,
    Binary = BINARY,
    Array = ARRAY,
    Map = MAP,
    TimeStamp = TIMESTAMP,
    Id = ID,
}

impl ElementType {
    pub fn from(tag: u8) -> Option<ElementType> {
        Some(match tag {
            BOOL => ElementType::Bool,
            NULL => ElementType::Null,
            F32 => ElementType::F32,
            F64 => ElementType::F64,
            I32 => ElementType::I32,
            I64 => ElementType::I64,
            U32 => ElementType::U32,
            U64 => ElementType::U64,
            STRING => ElementType::String,
            BINARY => ElementType::Binary,
            ARRAY => ElementType::Array,
            MAP => ElementType::Map,
            TIMESTAMP => ElementType::TimeStamp,
            ID => ElementType::Id,
            _ => return None,
        })
    }
}
