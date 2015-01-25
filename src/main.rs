//TODO(remove)
#![allow(dead_code)]

use std::io::{File, Reader};
use error::Ch8Error;

const RAM_SIZE: usize = 4096;
const PROGRAM_START: u16 = 0x200;

mod error;

struct Vm {
    reg: [u8; 16],
    index: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 256],
    ram: [u8; RAM_SIZE],

    timer: u8,
    tone: u8,

    screen: [u8; 64 * 32],
    keys: [u8; 16],
}

impl Vm {
    fn new() -> Vm {
        Vm {
            reg: [0; 16],
            index: 0,
            pc: PROGRAM_START, 
            sp: 0,
            stack: [0; 256],
            ram: [0; RAM_SIZE],

            timer: 0,
            tone: 0,

            screen: [0; 64 * 32],
            keys: [0; 16],
        }
    }

    fn load_rom(&mut self, reader: &mut Reader) -> Result<usize, Ch8Error> {
        let rom = try!(reader.read_to_end());
        if rom.len() > (RAM_SIZE - PROGRAM_START as usize) {
           return Err(Ch8Error::Io("Rom was larger than available RAM", None))
        }
        return Ok(rom.len());
    }
}

struct Op {
    raw: u16
}

impl Op {
    fn addr(&self) -> u16 {
        self.raw & 0x0FFF
    }

    fn x(&self) -> u8 {
        ((self.raw & 0x0F00) >> 8) as u8
    }

    fn y(&self) -> u8 {
        ((self.raw & 0x00F0) >> 4) as u8
    }

    fn n(&self) -> u8 {
        ((self.raw & 0xF000) >> 12) as u8
    }

    fn kk(&self) -> u8 {
        (self.raw & 0x00FF) as u8
    }
}

fn main() {
    let mut vm = Vm::new();

    let mut rom_file = File::open(&Path::new("/Users/jakerr/Downloads/IBM Logo.ch8")).unwrap();
    match vm.load_rom(&mut rom_file) {
        Ok(size) => println!("Loaded rom size: {}", size),
        Err(e) => println!("Error loading rom: {}", e)
    }

    let op = Op{raw: 0x8cd1};
    println!("addr: 0x{:X}", op.addr());
    println!("x: 0x{:X}", op.x());
    println!("y: 0x{:X}", op.y());
    println!("n: 0x{:X}", op.n());
    println!("kk: 0x{:X}", op.kk());
}
