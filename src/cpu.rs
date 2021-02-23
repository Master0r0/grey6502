use std::sync::Arc;

use crate::{instructions::{Instruction, init_instructions, Mode}};

pub struct StatRegister {
    negative: bool,
    overflow: bool,
    ignored: bool,
    sbreak: bool,
    decimal: bool,
    interrupt: bool,
    zero: bool,
    carry: bool
}

impl From<u8> for StatRegister {
    fn from(byte: u8) -> Self {
        Self {
            negative: byte & 0x1 == 1,
            overflow: byte & 0x2 == 1,
            ignored: byte & 0x4 == 1,
            sbreak: byte & 0x8 == 1,
            decimal: byte & 0x10 == 1,
            interrupt: byte & 0x20 == 1,
            zero: byte & 0x40 == 1,
            carry: byte & 0x80 == 1,
        }
    }
}

pub struct Registers {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: StatRegister,
    pub sp: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: StatRegister::from(0),
            sp: 0
        }
    }
}

pub struct CPU {
    pub memory: [u8; 0xFFFF],
    pub registers: Registers,
    pub instructions: Arc<Vec<Box<dyn Instruction>>>,
}

impl CPU {
    pub fn new() -> Self {
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFF44] = 0x18;
        Self {
            memory: mem,
            registers: Registers::new(),
            instructions: Arc::new(init_instructions()),
        }
    }

    pub fn get_memory_at_address(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn execute_instruction(&mut self, opcode: &u8, values: Vec<u8>) {
        let instructions = self.instructions.clone();
        let instruction = match instructions.iter().find(|i| i.get_opcodes().contains(opcode)) {
            Some(i) => i,
            None => {
                panic!("An unknown instruction was called");
            }
        };
        instruction.execute(opcode, values, Mode::Immediate, self);
    }
}
