use super::Cpu;
use super::{pack_bytes, unpack_bytes};
use crate::processor_status::ProcessorStatus;
use crate::Interrupts;
use crate::Mapper;
use crate::IRQ_BRK_VECTOR_ADDRESS;

impl<M: Mapper, I: Interrupts> Cpu<M, I> {
    // more information on BRK https://www.nesdev.org/wiki/Visual6502wiki/6502_BRK_and_B_bit
    pub(crate) fn instruction_brk(&mut self, caused_by_interrupt: bool) -> u8 {
        // we skip ahead 1 byte because the byte after a BRK provides debugging information
        let (pc_low, pc_high) = unpack_bytes(self.program_counter);

        self.push(pc_high);
        self.push(pc_low);

        // break flag is only pushed to the stack, and we only
        // do this if it was not caused by an interrupt
        if !caused_by_interrupt {
            self.processor_status.set_break_flag();
        }

        self.push(self.processor_status.0);

        if !caused_by_interrupt {
            self.processor_status.clear_break_flag();
        }

        // interrupt disable is set after pushing flags to stack https://www.nesdev.org/wiki/Status_flags#I:_Interrupt_Disable
        self.processor_status.set_interrupt_disable_flag();

        self.program_counter = pack_bytes(
            self.read(IRQ_BRK_VECTOR_ADDRESS),
            self.read(IRQ_BRK_VECTOR_ADDRESS + 1),
        );

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
