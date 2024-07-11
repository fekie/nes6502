#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use super::Cpu;

pub(crate) mod execution;

// https://emudev.de/nes-emulator/opcodes-and-addressing-modes-the-6502/   <-- good stuff
// https://blogs.oregonstate.edu/ericmorgan/2022/01/21/6502-addressing-modes/  <--- also this too
// https://www.masswerk.at/6502/6502_instruction_set.html#LDY <-- and here!
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Copy)]
pub enum AddressingMode {
    Accumulator,
    Absolute,
    AbsoluteXIndexed,
    AbsoluteYIndexed,
    Immediate,
    #[default]
    Implied,
    Indirect,
    IndirectXIndexed,
    IndirectYIndexed,
    Relative,
    Zeropage,
    ZeropageXIndexed,
    ZeropageYIndexed,
}

impl AddressingMode {
    /// Each instruction will require 1-3 bytes in total.
    /// This includes the opcode byte.
    pub fn bytes_required(&self) -> u16 {
        match self {
            AddressingMode::Accumulator | AddressingMode::Implied => 1,
            //
            AddressingMode::Immediate
            | AddressingMode::IndirectXIndexed
            | AddressingMode::IndirectYIndexed
            | AddressingMode::Relative
            | AddressingMode::Zeropage
            | AddressingMode::ZeropageXIndexed
            | AddressingMode::ZeropageYIndexed => 2,
            //
            AddressingMode::Absolute
            | AddressingMode::AbsoluteXIndexed
            | AddressingMode::AbsoluteYIndexed
            | AddressingMode::Indirect => 3,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Copy)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    #[default]
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

/// Includes both the opcode and the addressing mode from
/// the opcode byte.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct FullOpcode {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub low_byte: Option<u8>,
    pub high_byte: Option<u8>,
}

impl FullOpcode {
    // Returning None means that we tried to parse an illegal instruction
    pub fn try_new(byte: u8) -> Option<FullOpcode> {
        let low_nibble = byte & 0b0000_1111;
        let high_nibble = byte >> 4;

        match low_nibble {
            0x0 => low_nibble_0(high_nibble),
            0x1 => low_nibble_1(high_nibble),
            0x2 => low_nibble_2(high_nibble),
            0x3 => None,
            0x4 => low_nibble_4(high_nibble),
            0x5 => low_nibble_5(high_nibble),
            0x6 => low_nibble_6(high_nibble),
            0x7 => None,
            0x8 => low_nibble_8(high_nibble),
            0x9 => low_nibble_9(high_nibble),
            0xA => low_nibble_a(high_nibble),
            0xB => None,
            0xC => low_nibble_c(high_nibble),
            0xD => low_nibble_d(high_nibble),
            0xE => low_nibble_e(high_nibble),
            0xF => None,
            _ => unreachable!(),
        }
    }
}

fn low_nibble_0(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::BRK,
            addressing_mode: AddressingMode::Implied,
        },
        0x1 => FullOpcode {
            opcode: Opcode::BPL,
            addressing_mode: AddressingMode::Relative,
        },
        0x2 => FullOpcode {
            opcode: Opcode::JSR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3 => FullOpcode {
            opcode: Opcode::BMI,
            addressing_mode: AddressingMode::Relative,
        },
        0x4 => FullOpcode {
            opcode: Opcode::RTI,
            addressing_mode: AddressingMode::Implied,
        },
        0x5 => FullOpcode {
            opcode: Opcode::BVC,
            addressing_mode: AddressingMode::Relative,
        },
        0x6 => FullOpcode {
            opcode: Opcode::RTS,
            addressing_mode: AddressingMode::Implied,
        },
        0x7 => FullOpcode {
            opcode: Opcode::BVS,
            addressing_mode: AddressingMode::Relative,
        },
        0x8 => return None,
        0x9 => FullOpcode {
            opcode: Opcode::BCC,
            addressing_mode: AddressingMode::Relative,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::Immediate,
        },
        0xB => FullOpcode {
            opcode: Opcode::BCS,
            addressing_mode: AddressingMode::Relative,
        },
        0xC => FullOpcode {
            opcode: Opcode::CPY,
            addressing_mode: AddressingMode::Immediate,
        },
        0xD => FullOpcode {
            opcode: Opcode::BNE,
            addressing_mode: AddressingMode::Relative,
        },
        0xE => FullOpcode {
            opcode: Opcode::CPX,
            addressing_mode: AddressingMode::Immediate,
        },
        0xF => FullOpcode {
            opcode: Opcode::BEQ,
            addressing_mode: AddressingMode::Relative,
        },
        _ => unreachable!(),
    })
}

fn low_nibble_1(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x3 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x5 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0xD => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::IndirectXIndexed,
        },
        0xF => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::IndirectYIndexed,
        },
        _ => unreachable!(),
    })
}

fn low_nibble_2(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0..=0x9 => return None,
        0xA => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::Immediate,
        },
        0xB..=0xF => return None,
        _ => unreachable!(),
    })
}

fn low_nibble_4(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0..=0x1 => return None,
        0x2 => FullOpcode {
            opcode: Opcode::BIT,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x3 => return None,
        0x4 => return None,
        0x5 => return None,
        0x6 => return None,
        0x7 => return None,
        0x8 => FullOpcode {
            opcode: Opcode::STY,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STY,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CPY,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xD => return None,
        0xE => FullOpcode {
            opcode: Opcode::CPX,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xF => return None,
        _ => unreachable!(),
    })
}

fn low_nibble_5(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x3 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x5 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xD => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xF => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        _ => unreachable!(),
    })
}

fn low_nibble_6(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x3 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x5 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STX,
            addressing_mode: AddressingMode::Zeropage,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STX,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::DEC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xD => FullOpcode {
            opcode: Opcode::DEC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::INC,
            addressing_mode: AddressingMode::Zeropage,
        },
        0xF => FullOpcode {
            opcode: Opcode::INC,
            addressing_mode: AddressingMode::ZeropageXIndexed,
        },
        _ => unreachable!(),
    })
}

fn low_nibble_8(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::PHP,
            addressing_mode: AddressingMode::Implied,
        },
        0x1 => FullOpcode {
            opcode: Opcode::CLC,
            addressing_mode: AddressingMode::Implied,
        },
        0x2 => FullOpcode {
            opcode: Opcode::PLP,
            addressing_mode: AddressingMode::Implied,
        },
        0x3 => FullOpcode {
            opcode: Opcode::SEC,
            addressing_mode: AddressingMode::Implied,
        },
        0x4 => FullOpcode {
            opcode: Opcode::PHA,
            addressing_mode: AddressingMode::Implied,
        },
        0x5 => FullOpcode {
            opcode: Opcode::CLI,
            addressing_mode: AddressingMode::Implied,
        },
        0x6 => FullOpcode {
            opcode: Opcode::PLA,
            addressing_mode: AddressingMode::Implied,
        },
        0x7 => FullOpcode {
            opcode: Opcode::SEI,
            addressing_mode: AddressingMode::Implied,
        },
        0x8 => FullOpcode {
            opcode: Opcode::DEY,
            addressing_mode: AddressingMode::Implied,
        },
        0x9 => FullOpcode {
            opcode: Opcode::TYA,
            addressing_mode: AddressingMode::Implied,
        },
        0xA => FullOpcode {
            opcode: Opcode::TAY,
            addressing_mode: AddressingMode::Implied,
        },
        0xB => FullOpcode {
            opcode: Opcode::CLV,
            addressing_mode: AddressingMode::Implied,
        },
        0xC => FullOpcode {
            opcode: Opcode::INY,
            addressing_mode: AddressingMode::Implied,
        },
        0xD => FullOpcode {
            opcode: Opcode::CLD,
            addressing_mode: AddressingMode::Implied,
        },
        0xE => FullOpcode {
            opcode: Opcode::INX,
            addressing_mode: AddressingMode::Implied,
        },
        0xF => FullOpcode {
            opcode: Opcode::SED,
            addressing_mode: AddressingMode::Implied,
        },
        _ => unreachable!(),
    })
}

fn low_nibble_9(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::Immediate,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::Immediate,
        },
        0x3 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::Immediate,
        },
        0x5 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::Immediate,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0x8 => return None,
        0x9 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::Immediate,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::Immediate,
        },
        0xD => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::Immediate,
        },
        0xF => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        _ => unreachable!(),
    })
}

fn low_nibble_a(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x1 => return None,
        0x2 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x3 => return None,
        0x4 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x5 => return None,
        0x6 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x7 => return None,
        0x8 => FullOpcode {
            opcode: Opcode::TXA,
            addressing_mode: AddressingMode::Implied,
        },
        0x9 => FullOpcode {
            opcode: Opcode::TXS,
            addressing_mode: AddressingMode::Implied,
        },
        0xA => FullOpcode {
            opcode: Opcode::TAX,
            addressing_mode: AddressingMode::Implied,
        },
        0xB => FullOpcode {
            opcode: Opcode::TSX,
            addressing_mode: AddressingMode::Implied,
        },
        0xC => FullOpcode {
            opcode: Opcode::DEX,
            addressing_mode: AddressingMode::Implied,
        },
        0xD => return None,
        0xE => FullOpcode {
            opcode: Opcode::NOP,
            addressing_mode: AddressingMode::Implied,
        },
        0xF => return None,
        _ => unreachable!(),
    })
}

fn low_nibble_c(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => return None,
        0x1 => return None,
        0x2 => FullOpcode {
            opcode: Opcode::BIT,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3 => return None,
        0x4 => FullOpcode {
            opcode: Opcode::JMP,
            addressing_mode: AddressingMode::Absolute,
        },
        0x5 => return None,
        0x6 => FullOpcode {
            opcode: Opcode::JMP,
            addressing_mode: AddressingMode::Indirect,
        },
        0x7 => return None,
        0x8 => FullOpcode {
            opcode: Opcode::STY,
            addressing_mode: AddressingMode::Absolute,
        },
        0x9 => return None,
        0xA => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::Absolute,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDY,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CPY,
            addressing_mode: AddressingMode::Absolute,
        },
        0xD => return None,
        0xE => FullOpcode {
            opcode: Opcode::CPX,
            addressing_mode: AddressingMode::Absolute,
        },
        0xF => return None,
        _ => unreachable!(),
    })
}

fn low_nibble_d(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::Absolute,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ORA,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3 => FullOpcode {
            opcode: Opcode::AND,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x5 => FullOpcode {
            opcode: Opcode::EOR,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::Absolute,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ADC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::Absolute,
        },
        0x9 => FullOpcode {
            opcode: Opcode::STA,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xA => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::Absolute,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDA,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::Absolute,
        },
        0xD => FullOpcode {
            opcode: Opcode::CMP,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::Absolute,
        },
        0xF => FullOpcode {
            opcode: Opcode::SBC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        _ => unreachable!(),
    })
}
fn low_nibble_e(high_nibble: u8) -> Option<FullOpcode> {
    Some(match high_nibble {
        0x0 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::Absolute,
        },
        0x1 => FullOpcode {
            opcode: Opcode::ASL,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x2 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::Absolute,
        },
        0x3 => FullOpcode {
            opcode: Opcode::ROL,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x4 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x5 => FullOpcode {
            opcode: Opcode::LSR,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x6 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::Absolute,
        },
        0x7 => FullOpcode {
            opcode: Opcode::ROR,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0x8 => FullOpcode {
            opcode: Opcode::STX,
            addressing_mode: AddressingMode::Absolute,
        },
        0x9 => return None,
        0xA => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::Absolute,
        },
        0xB => FullOpcode {
            opcode: Opcode::LDX,
            addressing_mode: AddressingMode::AbsoluteYIndexed,
        },
        0xC => FullOpcode {
            opcode: Opcode::DEC,
            addressing_mode: AddressingMode::Absolute,
        },
        0xD => FullOpcode {
            opcode: Opcode::DEC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        0xE => FullOpcode {
            opcode: Opcode::INC,
            addressing_mode: AddressingMode::Absolute,
        },
        0xF => FullOpcode {
            opcode: Opcode::INC,
            addressing_mode: AddressingMode::AbsoluteXIndexed,
        },
        _ => unreachable!(),
    })
}
