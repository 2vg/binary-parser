extern crate byteorder;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

pub enum Endian {
    Big,
    Little,
}

pub struct BinaryParser {
    buffer: Vec<u8>,
    position: usize,
    length: usize,
    endian: Endian,
}

impl BinaryParser {
    fn new(buffer: Vec<u8>, position: usize, length: usize, endian: Endian) -> BinaryParser {
        BinaryParser {
            buffer,
            position,
            length,
            endian
        }
    }

    fn init(vec: Vec<u8>) -> BinaryParser {
        let pos = 0;
        let length = vec.len();
        let endian = Endian::Big;
        BinaryParser::new(vec, pos, length, endian)
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

    pub fn read_string(&mut self) -> String {
        let buffer = &self.buffer;
        let mut vec = Vec::new();
        let mut pos = 0;
        while self.position + pos < self.length {
            let bin = buffer[&self.position + pos];
            if bin == 0x00 {
                self.position += pos;
                break
            }
            else {
                vec.push(bin);
                pos += 1;
            }
        }
        String::from_utf8(vec).unwrap()
    }

    pub fn read_i8(&mut self) -> std::io::Result<i8> {
        let bin = self.buffer[self.position];
        self.position += 1;
        Ok(bin as i8)
    }

    pub fn read_i16(&mut self) -> std::io::Result<i16> {
        let mut bin = &self.buffer[self.position..self.position + 2];
        self.position += 2;
        match self.endian {
            Endian::Big => { bin.read_i16::<BigEndian>() },
            Endian::Little => { bin.read_i16::<LittleEndian>() }
        }
    }

    pub fn read_i32(&mut self) -> std::io::Result<i32> {
        let mut bin = &self.buffer[self.position..self.position + 4];
        self.position += 4;
        match self.endian {
            Endian::Big => { bin.read_i32::<BigEndian>() },
            Endian::Little => { bin.read_i32::<LittleEndian>() }
        }
    }

    pub fn read_i64(&mut self) -> std::io::Result<i64> {
        let mut bin = &self.buffer[self.position..self.position + 8];
        self.position += 8;
        match self.endian {
            Endian::Big => { bin.read_i64::<BigEndian>() },
            Endian::Little => { bin.read_i64::<LittleEndian>() }
        }
    }

    pub fn read_u8(&mut self) -> std::io::Result<u8> {
        let bin = self.buffer[self.position];
        self.position += 1;
        Ok(bin as u8)
    }

    pub fn read_u16(&mut self) -> std::io::Result<u16> {
        let mut bin = &self.buffer[self.position..self.position + 2];
        self.position += 2;
        match self.endian {
            Endian::Big => { bin.read_u16::<BigEndian>() },
            Endian::Little => { bin.read_u16::<LittleEndian>() }
        }
    }

    pub fn read_u32(&mut self) -> std::io::Result<u32> {
        let mut bin = &self.buffer[self.position..self.position + 4];
        self.position += 4;
        match self.endian {
            Endian::Big => { bin.read_u32::<BigEndian>() },
            Endian::Little => { bin.read_u32::<LittleEndian>() }
        }
    }

    pub fn read_u64(&mut self) -> std::io::Result<u64> {
        let mut bin = &self.buffer[self.position..self.position + 8];
        self.position += 8;
        match self.endian {
            Endian::Big => { bin.read_u64::<BigEndian>() },
            Endian::Little => { bin.read_u64::<LittleEndian>() }
        }
    }

    pub fn read_f32(&mut self) -> std::io::Result<f32> {
        let mut bin = &self.buffer[self.position..self.position + 4];
        self.position += 4;
        match self.endian {
            Endian::Big => { bin.read_f32::<BigEndian>() },
            Endian::Little => { bin.read_f32::<LittleEndian>() }
        }
    }

    pub fn read_f64(&mut self) -> std::io::Result<f64> {
        let mut bin = &self.buffer[self.position..self.position + 8];
        self.position += 8;
        match self.endian {
            Endian::Big => { bin.read_f64::<BigEndian>() },
            Endian::Little => { bin.read_f64::<LittleEndian>() }
        }
    }
}

#[test]
fn test_basic_binary_parse() {
    let string_binary = [0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64]; //hello world
    let mut parse = BinaryParser::from_u8_slice(&string_binary);
    let result = parse.read_string();

    assert_eq!("hello world", result);

    let float_binary = [0x3f, 0x8c, 0xcc, 0xcd]; //1.1
    let mut parse = BinaryParser::from_u8_slice(&float_binary);
    let result = parse.read_f32().unwrap();

    assert_eq!(1.1, result);
}
