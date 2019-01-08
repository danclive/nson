pub const DOUBLE: u8 = 0x01;
pub const I32: u8 = 0x02;
pub const I64: u8 = 0x03;
pub const U32: u8 = 0x04;
pub const U64: u8 = 0x05;
pub const STRING: u8 = 0x06;
pub const ARRAY: u8 = 0x07;
pub const MESSAGE: u8 = 0x08;
pub const BOOLEAN: u8 = 0x09;
pub const NULL: u8 = 0x0A;
pub const BINARY: u8 = 0x0B;
pub const TIMESTAMP: u8 = 0x0C;
pub const UTC_DATETIME: u8 = 0x0D; 

#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum ElementType {
    Double = DOUBLE,
    I32 = I32,
    I64 = I64,
    U32 = U32,
    U64 = U64,
    String = STRING,
    Array = ARRAY,
    Message = MESSAGE,
    Boolean = BOOLEAN,
    Null = NULL,
    Binary = BINARY,
    TimeStamp = TIMESTAMP,
    UTCDatetime = UTC_DATETIME
}

impl ElementType {
    pub fn from(tag: u8) -> Option<ElementType> {
        Some(match tag {
            DOUBLE => ElementType::Double,
            I32 => ElementType::I32,
            I64 => ElementType::I64,
            U32 => ElementType::U32,
            U64 => ElementType::U64, 
            STRING => ElementType::String,
            ARRAY => ElementType::Array,
            MESSAGE => ElementType::Message,
            BOOLEAN => ElementType::Boolean,
            NULL => ElementType::Null,
            BINARY => ElementType::Binary,
            TIMESTAMP => ElementType::TimeStamp,
            UTC_DATETIME => ElementType::UTCDatetime,
            _ => return None
        })
    }
}