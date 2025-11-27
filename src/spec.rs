//! Spec

pub const BOOL: u8 = 0x01;
pub const NULL: u8 = 0x02;
pub const F32: u8 = 0x11;
pub const F64: u8 = 0x12;
pub const I32: u8 = 0x13;
pub const I64: u8 = 0x14;
pub const U32: u8 = 0x15;
pub const U64: u8 = 0x16;
pub const I8: u8 = 0x17;
pub const U8: u8 = 0x18;
pub const I16: u8 = 0x19;
pub const U16: u8 = 0x1A;
pub const STRING: u8 = 0x21;
pub const BINARY: u8 = 0x22;
pub const ARRAY: u8 = 0x31;
pub const MAP: u8 = 0x32;
pub const TIMESTAMP: u8 = 0x41;
pub const ID: u8 = 0x42;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DataType {
    Bool = BOOL,
    Null = NULL,
    F32 = F32,
    F64 = F64,
    I32 = I32,
    I64 = I64,
    U32 = U32,
    U64 = U64,
    I8 = I8,
    U8 = U8,
    I16 = I16,
    U16 = U16,
    String = STRING,
    Binary = BINARY,
    Array = ARRAY,
    Map = MAP,
    TimeStamp = TIMESTAMP,
    Id = ID,
}

impl DataType {
    pub fn from(tag: u8) -> Option<DataType> {
        Some(match tag {
            BOOL => DataType::Bool,
            NULL => DataType::Null,
            F32 => DataType::F32,
            F64 => DataType::F64,
            I32 => DataType::I32,
            I64 => DataType::I64,
            U32 => DataType::U32,
            U64 => DataType::U64,
            I8 => DataType::I8,
            U8 => DataType::U8,
            I16 => DataType::I16,
            U16 => DataType::U16,
            STRING => DataType::String,
            BINARY => DataType::Binary,
            ARRAY => DataType::Array,
            MAP => DataType::Map,
            TIMESTAMP => DataType::TimeStamp,
            ID => DataType::Id,
            _ => return None,
        })
    }
}
