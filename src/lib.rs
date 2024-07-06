use instruction::{FullOpcode, Instruction, Opcode};
use processor_status::ProcessorStatus;
use sonic_rs::{Deserialize, Serialize};

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

/// Emulates an NES version of the 6502.
/// 
/// # Examples
/// ### Creating a memory mapper mapped to 64KB of ram.
/// ```
/// use nes6502::{Cpu, Mapper};
/// 
/// struct Memory([u8; 0x10000]);
/// 
/// impl Memory {
///     pub fn new() -> Self {
///         Self([0; 0x10000])
///     }
/// }
/// 
/// impl Mapper for Memory {
///     fn read(&self, address: u16) -> u8 {
///         self.0[address as usize]
///     }
/// 
///     fn write(&mut self, address: u16, byte: u8) {
///         self.0[address as usize] = byte
///     }
/// }
/// 
///
/// let memory = Memory::new();
/// let mut cpu = Cpu::new(memory);
/// 
/// let machine_cycles_taken = cpu.cycle();
///
/// ```
#[allow(clippy::upper_case_acronyms)]
pub struct Cpu<M: Mapper> {
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub registers: [u8; 6],
    pub processor_status: ProcessorStatus,
    pub memory_mapper: M,
    pub initialized: bool,
}

impl<M:Mapper> Cpu<M> {
    
}



/// The state of the CPU. The `ram` field is the non-zero memory
/// locations in [address, value]
#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl PartialEq for CpuState {
    fn eq(&self, other: &Self) -> bool {
        let other_fields_match = (self.pc == other.pc) && (self.s == other.s) && (self.a == other.a) && (self.x == other.x) &&(self.y == other.y) && (self.p == other.p);

        // Sort the ram by index. This is its normalized state
        let self_normalized_ram = {
            let mut cloned = self.ram.clone();
            cloned.sort_by(|a, b| {
                a[0].cmp(&b[0])
            });
            cloned.into_iter().filter(|x| {
                x[1] != 0
            }).collect::<Vec<Vec<u16>>>()
        };

        let other_normalized_ram = {
            let mut cloned = other.ram.clone();
            cloned.sort_by(|a, b| {
                a[0].cmp(&b[0])
            });
            cloned.into_iter().filter(|x| {
                x[1] != 0
            }).collect::<Vec<Vec<u16>>>()
        };

        other_fields_match && (self_normalized_ram == other_normalized_ram)
    }
}

impl<M: Mapper> Cpu<M> {
    /// Creates a new Cpu but does not initialize it as it needs to be connected
    /// to the bus to initialize. You can initialize it with [`Self::initialize`].
    #[allow(clippy::new_without_default)]
    pub fn new(memory_mapper: M) -> Self {
        Self {
            accumulator: 0,
            x: 0,
            y: 0,
            stack_pointer: STACK_POINTER_STARTING_VALUE,
            program_counter: 0,
            registers: [0; 6],
            processor_status: ProcessorStatus::new(),
            memory_mapper,
            initialized: false,
        }
    }

    pub fn from_state(cpu_state: CpuState, mut memory_mapper: M) -> Self {
        for chunk in &cpu_state.ram {
            let address = chunk[0];
            let value = chunk[1];

            memory_mapper.write(address, value as u8);
        }

        let cpu = Self {
            accumulator: cpu_state.a,
            x: cpu_state.x,
            y: cpu_state.y,
            stack_pointer: cpu_state.s,
            program_counter: cpu_state.pc,
            registers: [0; 6],
            processor_status: ProcessorStatus(cpu_state.p),
            memory_mapper,
            initialized: true,
        };

        // sanity check
        assert_eq!(cpu.state(), cpu_state);

        cpu
    }

    pub fn state(&self) -> CpuState {
        let ram = {
            let mut ram = Vec::new();

            for i in 0..=65535 {
                let value = self.read(i);
                if value != 0 {
                    ram.push(vec![i, value as u16])
                }
            }

            ram
        };

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

    pub fn reset(&mut self) {
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
            let high_byte = self.memory_mapper.read( RESET_VECTOR_ADDRESS + 1) as u16;
            (high_byte << 8) + low_byte
        };
    }
    
    /// Initializes the CPU to a state ready to run instructions.
    pub fn initialize(&mut self) {
        self.reset();
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
        let instruction = self.fetch().unwrap();

        // execute
        self.execute(instruction)
    }

    // returns true on the second return value if instruction was executed successfully
    pub fn cycle_debug(&mut self) -> (u8, bool, Option<Instruction>) {
        let instruction = match self.fetch() {
            Some(x) => x,
            None => return (0, false, None),
        };
        //self.pretty_print_cpu_state(instruction);

        // execute
        (self.execute(instruction), true, Some(instruction))
    }

    /// Fetches the next instruction and updates the program counter.
    pub fn fetch(&mut self) -> Option<Instruction> {
        let full_opcode = match FullOpcode::try_new(self.memory_mapper.read(self.program_counter)) {
            Some(x) => x,
            None => return None
        };

        let bytes_required = full_opcode.addressing_mode.bytes_required();

        // Low byte comes first as words are in little-endian
        let (low_byte, high_byte) = match bytes_required {
            1 => (None, None),
            2 => (
                Some(self.memory_mapper.read(self.program_counter.wrapping_add(1))),
                None,
            ),
            3 => (
                Some(self.memory_mapper.read(self.program_counter.wrapping_add(1))),
                Some(self.memory_mapper.read(self.program_counter.wrapping_add(2))),
            ),
            _ => unreachable!(),
        };

        // Decide how much we need to increment the PC
        self.program_counter = self.program_counter.wrapping_add(bytes_required);

        Some(Instruction {
                    opcode: full_opcode.opcode,
                    addressing_mode: full_opcode.addressing_mode,
                    low_byte,
                    high_byte,
                })
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