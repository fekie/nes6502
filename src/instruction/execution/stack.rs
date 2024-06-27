use super::{AddressingMode, Cpu};
use crate::ProcessorStatus;

impl Cpu {
    pub(crate) fn instruction_tsx(&mut self) -> u8 {
        self.x = self.stack_pointer;
        2
    }

    pub(crate) fn instruction_txs(&mut self) -> u8 {
        self.stack_pointer = self.x;
        2
    }

    pub(crate) fn instruction_pha(&mut self) -> u8 {
        self.push(self.accumulator);
        3
    }

    pub(crate) fn instruction_php(&mut self) -> u8 {
        self.processor_status.set_break_flag();
        self.push(self.processor_status.0);
        self.processor_status.clear_break_flag();

        3
    }

    pub(crate) fn instruction_pla(&mut self) -> u8 {
        self.accumulator = self.pop();

        4
    }

    pub(crate) fn instruction_plp(&mut self) -> u8 {
        self.processor_status = ProcessorStatus(self.pop());

        4
    }
}
