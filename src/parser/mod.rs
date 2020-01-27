extern crate anyhow;
extern crate byteorder;

use anyhow::*;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

pub enum Endian {
    Big,
    Little,
}

pub struct BinaryParser {
    buffer: Vec<u8>,
    position: usize,
    endian: Endian,
}

impl BinaryParser {
    fn new(buffer: Vec<u8>, position: usize, endian: Endian) -> BinaryParser {
        BinaryParser {
            buffer,
            position,
            endian
        }
    }

    fn init(vec: Vec<u8>) -> BinaryParser {
        let pos = 0;
        let endian = Endian::Big;
        BinaryParser::new(vec, pos, endian)
    }

    pub fn from_vec(vec: &Vec<u8>) -> BinaryParser {
        BinaryParser::init(vec.to_vec())
    }

    pub fn from_u8(u8_array: &'static [u8]) -> BinaryParser {
        BinaryParser::init(u8_array.to_vec())
    }

    pub fn from_u8_slice(u8_silice: &[u8]) -> BinaryParser {
        BinaryParser::init(u8_silice.to_vec())
    }

    pub fn set_big_endian(&mut self) {
        self.endian = Endian::Big;
    }

    pub fn set_little_endian(&mut self) {
        self.endian = Endian::Little;
    }

    pub fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    pub fn forward_position(&mut self, pos: usize) {
        self.position += pos;
    }

    pub fn back_position(&mut self, pos: usize) {
        self.position -= pos;
    }

    pub fn read_string(&mut self) -> anyhow::Result<String> {
        let buffer = &self.buffer;
        let mut vec = Vec::new();
        let mut pos = 0;
        while self.position + pos < self.buffer.len() {
            let bin = buffer[&self.position + pos];
            if bin == 0x00 {
                self.position += pos + 1;
                break
            }
            else {
                vec.push(bin);
                pos += 1;
            }
        }
        let result = String::from_utf8(vec)?;
        Ok(result)
    }

    pub fn read_i8(&mut self) -> anyhow::Result<i8> {
        let bin = &get_slice(&self.buffer, self.position, 1);
        self.position += 1;
        if bin.len() != 0 { Ok(bin[0] as i8) } else { Err(anyhow!(0)) }
    }

    pub fn read_i16(&mut self) -> anyhow::Result<i16> {
        let mut bin = get_slice(&self.buffer, self.position, 2);
        self.position += 2;
        match self.endian {
            Endian::Big => { let result = bin.read_i16::<BigEndian>()?; Ok(result) },
            Endian::Little => { let result = bin.read_i16::<LittleEndian>()?; Ok(result) }
        }
    }

    pub fn read_i32(&mut self) -> anyhow::Result<i32> {
        let mut bin = get_slice(&self.buffer, self.position, 4);
        self.position += 4;
        match self.endian {
            Endian::Big => { let result = bin.read_i32::<BigEndian>()?; Ok(result) },
            Endian::Little => { let result = bin.read_i32::<LittleEndian>()?; Ok(result) }
        }
    }

    pub fn read_i64(&mut self) -> anyhow::Result<i64> {
        let mut bin = get_slice(&self.buffer, self.position, 8);
        self.position += 8;
        match self.endian {
            Endian::Big => { let result = bin.read_i64::<BigEndian>()?; Ok(result) },
            Endian::Little => { let result = bin.read_i64::<LittleEndian>()?; Ok(result) }
        }
    }

    pub fn read_u8(&mut self) -> anyhow::Result<u8> {
        let bin = get_slice(&self.buffer, self.position, 1);
        self.position += 1;
        if bin.len() != 0 { Ok(bin[0]) } else { Err(anyhow!(0)) }
    }

    pub fn read_u16(&mut self) -> anyhow::Result<u16> {
        let mut bin = get_slice(&self.buffer, self.position, 2);
        self.position += 2;
        match self.endian {
            Endian::Big => { let result = bin.read_u16::<BigEndian>()?; Ok(result) },
            Endian::Little => { let result = bin.read_u16::<LittleEndian>()?; Ok(result) }
        }
    }

    pub fn read_u32(&mut self) -> anyhow::Result<u32> {
        let mut bin = get_slice(&self.buffer, self.position, 4);
        self.position += 4;
        match self.endian {
            Endian::Big => { let result = bin.read_u32::<BigEndian>()?; Ok(result) },
            Endian::Little => { let result = bin.read_u32::<LittleEndian>()?; Ok(result) }
        }
    }

    pub fn read_u64(&mut self) -> anyhow::Result<u64> {
        let mut bin = get_slice(&self.buffer, self.position, 8);
        self.position += 8;
        match self.endian {
            Endian::Big => { let result = bin.read_u64::<BigEndian>()?; Ok(result) },
            Endian::Little => { let result = bin.read_u64::<LittleEndian>()?; Ok(result) }
        }
    }

    pub fn read_f32(&mut self) -> anyhow::Result<f32> {
        let mut bin = get_slice(&self.buffer, self.position, 4);
        self.position += 4;
        match self.endian {
            Endian::Big => { let result = bin.read_f32::<BigEndian>()?; Ok(result) },
            Endian::Little => { let result = bin.read_f32::<LittleEndian>()?; Ok(result) }
        }
    }

    pub fn read_f64(&mut self) -> anyhow::Result<f64> {
        let mut bin = get_slice(&self.buffer, self.position, 8);
        self.position += 8;
        match self.endian {
            Endian::Big => { let result = bin.read_f64::<BigEndian>()?; Ok(result) },
            Endian::Little => { let result = bin.read_f64::<LittleEndian>()?; Ok(result) }
        }
    }
}

fn get_slice(slice: &[u8], position: usize, length: usize) -> &[u8] {
    if position + length > slice.len() {
        &[] as &[u8]
    }
    else {
        &slice[position..position + length]
    }
}

#[test]
fn test_basic_binary_parse() {
    let binary = [0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x00, 0x3f, 0x8c, 0xcc, 0xcd];
    let mut parse = BinaryParser::from_u8_slice(&binary);

    let result = parse.read_string();
    assert_ne!(true, result.is_err());
    assert_eq!("hello world", result.unwrap());

    let result = parse.read_f32();
    assert_ne!(true, result.is_err());
    assert_eq!(1.1, result.unwrap());
}

#[test]
fn test_basic_binary_parse_failed() {
    let binary = [0xFF, 0xFF, 0xFF];
    let mut parse = BinaryParser::from_u8_slice(&binary);

    let result = parse.read_string();
    assert_eq!(true, result.is_err());

    let result = parse.read_f32();
    assert_eq!(true, result.is_err());
}

#[test]
fn test_binary_parse_with_empty_array() {
    let binary = [];
    let mut parse = BinaryParser::from_u8_slice(&binary);

    let result = parse.read_string();
    assert_eq!(true, result.is_ok());
    assert_eq!("", result.unwrap());

    let result = parse.read_f32();
    assert_eq!(true, result.is_err());
}
