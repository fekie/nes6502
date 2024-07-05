use super::Cpu;
use crate::ProcessorStatus;

impl Cpu {
    pub(crate) fn instruction_tsx(&mut self) -> u8 {
        self.x = self.stack_pointer;

        self.modify_zero_flag(self.x);
        self.modify_negative_flag(self.x);

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
        self.modify_zero_flag(self.accumulator);
        self.modify_negative_flag(self.accumulator);

        4
    }

    pub(crate) fn instruction_plp(&mut self) -> u8 {
        // ignore break flag and bit 5
        let original_flags = self.processor_status.0 & 0b0011_0000;
        self.processor_status = ProcessorStatus((self.pop() & 0b1100_1111) | original_flags);

        4
    }
}
