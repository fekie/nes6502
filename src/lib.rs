use instruction::{FullOpcode, Instruction, Opcode};
use processor_status::ProcessorStatus;
use serde::{Deserialize, Serialize};

const STACK_POINTER_STARTING_VALUE: u8 = 0xFF;
pub const NMI_VECTOR_ADDRESS: u16 = 0xFFFA;
pub const RESET_VECTOR_ADDRESS: u16 = 0xFFFC;
pub const IRQ_BRK_VECTOR_ADDRESS: u16 = 0xFFFE;

mod instruction;
mod processor_status;

pub trait Mapper {
    fn read(&self, address: u16) -> u8;

    fn write(&mut self, address: u16, byte: u8);
}

#[allow(clippy::upper_case_acronyms)]
pub struct Cpu {
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub registers: [u8; 6],
    pub processor_status: ProcessorStatus,
    pub memory_mapper: CpuMemoryMapper,
    pub initialized: bool,
}

/// The state of the CPU. The `ram` field is the non-zero memory
/// locations in [address, value]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CpuState {
    pub pc: u16,
    pub s: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    // The non-zero memory
    /// locations in [address, value] form. The value will be
    /// within a u16
    pub ram: Vec<Vec<u16>>,
}

impl Cpu {
    /// Creates a new Cpu but does not initialize it as it needs to be connected
    /// to the bus to initialize. You can initialize it with [`Self::initialize`].
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            x: 0,
            y: 0,
            stack_pointer: STACK_POINTER_STARTING_VALUE,
            program_counter: 0,
            registers: [0; 6],
            processor_status: ProcessorStatus::new(),
            memory_mapper: CpuMemoryMapper::new(),
            initialized: false,
        }
    }

    pub fn from_state(cpu_state: &CpuState) -> Self {
        Self {
            accumulator: cpu_state.a,
            x: cpu_state.x,
            y: cpu_state.y,
            stack_pointer: cpu_state.s,
            program_counter: cpu_state.pc,
            registers: [0; 6],
            processor_status: ProcessorStatus(cpu_state.p),
            memory_mapper: CpuMemoryMapper::new(),
            initialized: true,
        }
    }

    pub fn state(&self) -> CpuState {
        let ram = self
            .memory_mapper
            .work_ram
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, value)| match *value != 0 {
                true => Some(vec![i as u16, *value as u16]),
                false => None,
            })
            .collect::<Vec<Vec<u16>>>();

        CpuState {
            pc: self.program_counter,
            s: self.stack_pointer,
            a: self.accumulator,
            x: self.x,
            y: self.y,
            p: self.processor_status.0,
            ram,
        }
    }

    /// Initializes the Cpu to a state ready to run instructions.
    pub fn initialize(&mut self) {
        self.processor_status.clear_carry_flag();
        self.processor_status.clear_zero_flag();
        self.processor_status.set_interrupt_disable_flag();
        self.processor_status.clear_decimal_flag();
        self.processor_status.clear_overflow_flag();
        self.processor_status.clear_negative_flag();
        self.processor_status.clear_break_flag();

        self.stack_pointer = STACK_POINTER_STARTING_VALUE;

        self.program_counter = {
            let low_byte = self.memory_mapper.read(RESET_VECTOR_ADDRESS) as u16;
            let high_byte = self.memory_mapper.read(RESET_VECTOR_ADDRESS + 1) as u16;
            (high_byte << 8) + low_byte
        };

        self.initialized = true;
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }

    /// Runs a full instruction cycle. Returns the amount of
    /// machine cycles taken.
    pub fn cycle(&mut self) -> u8 {
        // check for interrupts
        /* if *bus.interrupts.borrow().interrupt.borrow() == Request::Active || *bus.interrupts.borrow().non_maskable_interrupt.borrow() == Request::Active {
            // if we get an interrupt, then set the previous pc back

        } */
        // fetch
        let instruction = self.fetch();
        self.pretty_print_cpu_state(instruction);

        // execute
        self.execute(instruction)
    }

    /// Fetches the next instruction and updates the program counter.
    pub fn fetch(&mut self) -> Instruction {
        let full_opcode = FullOpcode::new(self.memory_mapper.read(self.program_counter));

        let bytes_required = full_opcode.addressing_mode.bytes_required();

        // Low byte comes first as words are in little-endian
        let (low_byte, high_byte) = match bytes_required {
            1 => (None, None),
            2 => (
                Some(self.memory_mapper.read(self.program_counter + 1)),
                None,
            ),
            3 => (
                Some(self.memory_mapper.read(self.program_counter + 1)),
                Some(self.memory_mapper.read(self.program_counter + 2)),
            ),
            _ => unreachable!(),
        };

        // Decide how much we need to increment the PC
        self.program_counter += bytes_required;

        Instruction {
            opcode: full_opcode.opcode,
            addressing_mode: full_opcode.addressing_mode,
            low_byte,
            high_byte,
        }
    }

    /// Executes the instruction and returns the amount of machine cycles that it took.
    pub fn execute(&mut self, instruction: Instruction) -> u8 {
        match instruction.opcode {
            Opcode::ADC => self.instruction_adc(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::AND => self.instruction_and(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::ASL => self.instruction_asl(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::BCC => self.instruction_bcc(instruction.low_byte),
            Opcode::BCS => self.instruction_bcs(instruction.low_byte),
            Opcode::BEQ => self.instruction_beq(instruction.low_byte),
            Opcode::BIT => self.instruction_bit(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::BMI => self.instruction_bmi(instruction.low_byte),
            Opcode::BNE => self.instruction_bne(instruction.low_byte),
            Opcode::BPL => self.instruction_bpl(instruction.low_byte),
            Opcode::BRK => self.instruction_brk(),
            Opcode::BVC => self.instruction_bvc(instruction.low_byte),
            Opcode::BVS => self.instruction_bvs(instruction.low_byte),
            Opcode::CLC => self.instruction_clc(),
            Opcode::CLD => self.instruction_cld(),
            Opcode::CLI => self.instruction_cli(),
            Opcode::CLV => self.instruction_clv(),
            Opcode::CMP => self.instruction_cmp(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::CPX => self.instruction_cpx(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::CPY => self.instruction_cpy(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::DEC => self.instruction_dec(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::DEX => self.instruction_dex(),
            Opcode::DEY => self.instruction_dey(),
            Opcode::EOR => self.instruction_eor(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::INC => self.instruction_inc(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::INX => self.instruction_inx(),
            Opcode::INY => self.instruction_iny(),
            Opcode::JMP => self.instruction_jmp(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::JSR => self.instruction_jsr(instruction.low_byte, instruction.high_byte),
            Opcode::LDA => self.instruction_lda(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::LDX => self.instruction_ldx(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::LDY => self.instruction_ldy(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::LSR => self.instruction_lsr(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::NOP => self.instruction_nop(),
            Opcode::ORA => self.instruction_ora(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::PHA => self.instruction_pha(),
            Opcode::PHP => self.instruction_php(),
            Opcode::PLA => self.instruction_pla(),
            Opcode::PLP => self.instruction_plp(),
            Opcode::ROL => self.instruction_rol(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::ROR => self.instruction_ror(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::RTI => self.instruction_rti(),
            Opcode::RTS => self.instruction_rts(),
            Opcode::SBC => self.instruction_sbc(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::SEC => self.instruction_sec(),
            Opcode::SED => self.instruction_sed(),
            Opcode::SEI => self.instruction_sei(),
            Opcode::STA => self.instruction_sta(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::STX => self.instruction_stx(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::STY => self.instruction_sty(
                instruction.addressing_mode,
                instruction.low_byte,
                instruction.high_byte,
            ),
            Opcode::TAX => self.instruction_tax(),
            Opcode::TAY => self.instruction_tay(),
            Opcode::TSX => self.instruction_tsx(),
            Opcode::TXA => self.instruction_txa(),
            Opcode::TXS => self.instruction_txs(),
            Opcode::TYA => self.instruction_tya(),
        }
    }

    // Shortcuts to read a byte from the memory mapper because
    // we use this a lot.
    pub fn read(&self, address: u16) -> u8 {
        self.memory_mapper.read(address)
    }

    pub(crate) fn write(&mut self, address: u16, value: u8) {
        self.memory_mapper.write(address, value);
    }

    #[allow(dead_code)]
    /// Pretty prints the full state of the Cpu. Meant to be used after fetch but
    /// before execution to work correctly.
    pub fn pretty_print_cpu_state(&self, instruction: Instruction) {
        println!("------------------------------------");
        println!("New PC: ${:02X}", self.program_counter);
        println!("Instruction (not yet executed): {:#?}", instruction);
        println!(
            "Accumulator: {} | X: {} | Y: {}",
            self.accumulator, self.x, self.y
        );
        println!(
            "Stack Pointer: ${:02X} -> ${:04X}",
            self.stack_pointer,
            self.stack_pointer as u16 + 0x0100
        );
        println!("Registers: {:?}", self.registers);
        println!(
            "Carry: {} | Zero: {} | Interrupt Disable: {} | Decimal: {} | Break: {} | Overflow: {} | Negative: {}",
            self.processor_status.carry_flag(), self.processor_status.zero_flag(), self.processor_status.interrupt_disable_flag(), 
            self.processor_status.decimal_flag(), self.processor_status.break_flag(), self.processor_status.overflow_flag(), 
            self.processor_status.negative_flag()
        );
        println!("------------------------------------");
    }
}

// We use 2KB of work ram.
#[derive(Debug)]
pub struct WorkRAM([u8; 0x10000]);

/// Memory Map:
///
/// | Address range |  Size  |                                  Device                                  |   |   |
/// |:-------------:|:------:|:------------------------------------------------------------------------:|---|---|
/// | $0000–$07FF   | $0800  | 2 KB internal RAM                                                        |   |   |
/// | $0800–$0FFF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $1000–$17FF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $1800–$1FFF   | $0800  | Mirror  of $0000–$07FF                                                   |   |   |
/// | $2000–$2007   | $0008  | NES PPU registers                                                        |   |   |
/// | $2008–$3FFF   | $1FF8  | Mirrors of $2000–$2007 (repeats every 8 bytes)                           |   |   |
/// | $4000–$4017   | $0018  | NES APU and I/O registers                                                |   |   |
/// | $4018–$401F   | $0008  | APU and I/O functionality that is normally disabled. See Cpu Test Mode.  |   |   |
/// | $4020–$FFFF   | $BFE0  | Unmapped. Available for cartridge use.                                   |   |   |
/// | *$6000-$7FFF  | $2000  | Usually cartridge RAM, when present.                                     |   |   |
/// | *$8000-$FFFF  | $8000  | Usually cartridge ROM and mapper registers.                              |   |   |
#[derive(Debug)]
pub struct CpuMemoryMapper {
    work_ram: WorkRAM,
}

impl CpuMemoryMapper {
    pub fn new() -> Self {
        Self {
            work_ram: WorkRAM([0; 65536]),
        }
    }
}

impl Mapper for CpuMemoryMapper {
    fn read(&self, address: u16) -> u8 {
        // harte's tests (https://github.com/SingleStepTests/ProcessorTests/tree/main/nes6502)
        // require the entire address space mapped to RAM
        self.work_ram.0[address as usize]
    }

    fn write(&mut self, address: u16, byte: u8) {
        self.work_ram.0[address as usize] = byte;
    }
}
