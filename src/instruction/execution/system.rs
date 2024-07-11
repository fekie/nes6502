use super::Cpu;
use super::{pack_bytes, unpack_bytes};
use crate::processor_status::ProcessorStatus;
use crate::IRQ_BRK_VECTOR_ADDRESS;
use crate::{Interrupts, RESET_VECTOR_ADDRESS};
use crate::{Mapper, NMI_VECTOR_ADDRESS};

/// Describes the interrupt state that triggered a BRK to determine which reset vector to use.
/// Also includes Reset, on top of the normal interrupts.
#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum InterruptState {
    Inactive,
    Reset,
    MaskableInterrupt,
    NonMaskableInterrupt,
}

impl<M: Mapper, I: Interrupts> Cpu<M, I> {
    // more information on BRK https://www.nesdev.org/wiki/Visual6502wiki/6502_BRK_and_B_bit
    pub(crate) fn instruction_brk(&mut self, interrupt_state: InterruptState) -> u8 {
        // we skip ahead 1 byte because the byte after a BRK provides debugging information
        let (pc_low, pc_high) = unpack_bytes(self.program_counter);

        self.push(pc_high);
        self.push(pc_low);

        // break flag is only pushed to the stack, and we only
        // do this if it was not caused by an interrupt
        if interrupt_state == InterruptState::Inactive || interrupt_state == InterruptState::Reset {
            self.processor_status.set_break_flag();
        }

        self.push(self.processor_status.0);

        if interrupt_state == InterruptState::Inactive || interrupt_state == InterruptState::Reset {
            self.processor_status.clear_break_flag();
        }

        // interrupt disable is set after pushing flags to stack https://www.nesdev.org/wiki/Status_flags#I:_Interrupt_Disable
        self.processor_status.set_interrupt_disable_flag();

        self.program_counter = match interrupt_state {
            InterruptState::Inactive | InterruptState::MaskableInterrupt => pack_bytes(
                self.read(IRQ_BRK_VECTOR_ADDRESS),
                self.read(IRQ_BRK_VECTOR_ADDRESS + 1),
            ),
            InterruptState::Reset => pack_bytes(
                self.read(RESET_VECTOR_ADDRESS),
                self.read(RESET_VECTOR_ADDRESS + 1),
            ),
            InterruptState::NonMaskableInterrupt => pack_bytes(
                self.read(NMI_VECTOR_ADDRESS),
                self.read(NMI_VECTOR_ADDRESS + 1),
            ),
        };

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
