use super::twos_compliment_to_signed;
use super::Cpu;
use crate::Mapper;

impl<M: Mapper> Cpu<M> {
    pub(crate) fn instruction_bcc(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = !self.processor_status.carry_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bcs(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = self.processor_status.carry_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_beq(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = self.processor_status.zero_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bmi(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = self.processor_status.negative_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bne(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = !self.processor_status.zero_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bpl(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = !self.processor_status.negative_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bvc(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = !self.processor_status.overflow_flag();
        branch(self, low_byte, needs_branch)
    }

    pub(crate) fn instruction_bvs(&mut self, low_byte: Option<u8>) -> u8 {
        let needs_branch = self.processor_status.overflow_flag();
        branch(self, low_byte, needs_branch)
    }
}

/// Executes a branch based on whether it needs a branch.
fn branch<M: Mapper>(cpu: &mut Cpu<M>, low_byte: Option<u8>, needs_branch: bool) -> u8 {
    let value = twos_compliment_to_signed(low_byte.unwrap());
    let original_page = cpu.program_counter >> 8;

    if needs_branch {
        match value.is_positive() {
            true => {
                cpu.program_counter = cpu.program_counter.wrapping_add(value as u16);
            }
            false => {
                cpu.program_counter = cpu.program_counter.wrapping_sub((-(value as i16)) as u16)
            }
        };
    }

    let new_page = cpu.program_counter >> 8;
    let page_crossed = original_page != new_page;

    match needs_branch {
        true => match page_crossed {
            true => 4,
            false => 3,
        },
        false => 2,
    }
}
