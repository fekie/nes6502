use super::{pack_bytes, unpack_bytes};
use super::{AddressingMode, Cpu};
use crate::processor_status::ProcessorStatus;
use crate::IRQ_BRK_VECTOR_ADDRESS;

impl Cpu {
    pub(crate) fn instruction_brk(&mut self) -> u8 {
        let (pc_low, pc_high) = unpack_bytes(self.program_counter);

        self.push(pc_high);
        self.push(pc_low);

        self.processor_status.set_break_flag();

        self.push(self.processor_status.0);

        self.program_counter = IRQ_BRK_VECTOR_ADDRESS;

        7
    }

    pub(crate) fn instruction_nop(&mut self) -> u8 {
        2
    }

    pub(crate) fn instruction_rti(&mut self) -> u8 {
        // ignore the new break flag and bit 5
        self.processor_status =
            ProcessorStatus((self.pop() & 0b1100_1111) | (self.processor_status.0 & 0b0011_0000));

        let pc_low = self.pop();
        let pc_high: u8 = self.pop();

        self.program_counter = pack_bytes(pc_low, pc_high);

        6
    }
}
