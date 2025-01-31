use super::{
    absolute_read, absolute_x_read, absolute_y_read, handle_invalid_addressing_mode,
    immediate_read, indirect_x_read, indirect_y_read, zeropage_read, zeropage_x_read,
};
use super::{AddressingMode, Cpu};
use crate::Interrupts;
use crate::Mapper;

impl<M: Mapper, I: Interrupts> Cpu<M, I> {
    pub(crate) fn instruction_adc(
        &mut self,

        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte);
                self.adc_intermediate(value);

                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, low_byte);
                self.adc_intermediate(value);

                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, low_byte);
                self.adc_intermediate(value);

                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, low_byte, high_byte);
                self.adc_intermediate(value);

                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (value, boundary_crossed) = absolute_x_read(self, low_byte, high_byte);
                self.adc_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (value, boundary_crossed) = absolute_y_read(self, low_byte, high_byte);
                self.adc_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let value = indirect_x_read(self, low_byte);
                self.adc_intermediate(value);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let (value, boundary_crossed) = indirect_y_read(self, low_byte);
                self.adc_intermediate(value);

                match boundary_crossed {
                    true => 6,
                    false => 5,
                }
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_sbc(
        &mut self,

        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte);
                self.sbc_intermediate(value);

                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, low_byte);
                self.sbc_intermediate(value);

                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, low_byte);
                self.sbc_intermediate(value);

                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, low_byte, high_byte);
                self.sbc_intermediate(value);

                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (value, boundary_crossed) = absolute_x_read(self, low_byte, high_byte);
                self.sbc_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (value, boundary_crossed) = absolute_y_read(self, low_byte, high_byte);
                self.sbc_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let value = indirect_x_read(self, low_byte);
                self.sbc_intermediate(value);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let (value, boundary_crossed) = indirect_y_read(self, low_byte);
                self.sbc_intermediate(value);

                match boundary_crossed {
                    true => 6,
                    false => 5,
                }
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_cmp(
        &mut self,

        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte);
                self.cmp_intermediate(value);

                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, low_byte);
                self.cmp_intermediate(value);

                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, low_byte);
                self.cmp_intermediate(value);

                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, low_byte, high_byte);
                self.cmp_intermediate(value);

                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (value, boundary_crossed) = absolute_x_read(self, low_byte, high_byte);
                self.cmp_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (value, boundary_crossed) = absolute_y_read(self, low_byte, high_byte);
                self.cmp_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let value = indirect_x_read(self, low_byte);
                self.cmp_intermediate(value);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let (value, boundary_crossed) = indirect_y_read(self, low_byte);
                self.cmp_intermediate(value);

                match boundary_crossed {
                    true => 6,
                    false => 5,
                }
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_cpx(
        &mut self,

        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte);
                self.cpx_intermediate(value);

                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, low_byte);
                self.cpx_intermediate(value);

                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, low_byte);
                self.cpx_intermediate(value);

                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, low_byte, high_byte);
                self.cpx_intermediate(value);

                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (value, boundary_crossed) = absolute_x_read(self, low_byte, high_byte);
                self.cpx_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (value, boundary_crossed) = absolute_y_read(self, low_byte, high_byte);
                self.cpx_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let value = indirect_x_read(self, low_byte);
                self.cpx_intermediate(value);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let (value, boundary_crossed) = indirect_y_read(self, low_byte);
                self.cpx_intermediate(value);

                match boundary_crossed {
                    true => 6,
                    false => 5,
                }
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_cpy(
        &mut self,

        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Immediate => {
                let value = immediate_read(low_byte);
                self.cpy_intermediate(value);

                2
            }
            AddressingMode::Zeropage => {
                let value = zeropage_read(self, low_byte);
                self.cpy_intermediate(value);

                3
            }
            AddressingMode::ZeropageXIndexed => {
                let value = zeropage_x_read(self, low_byte);
                self.cpy_intermediate(value);

                4
            }
            AddressingMode::Absolute => {
                let value = absolute_read(self, low_byte, high_byte);
                self.cpy_intermediate(value);

                4
            }
            AddressingMode::AbsoluteXIndexed => {
                let (value, boundary_crossed) = absolute_x_read(self, low_byte, high_byte);
                self.cpy_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::AbsoluteYIndexed => {
                let (value, boundary_crossed) = absolute_y_read(self, low_byte, high_byte);
                self.cpy_intermediate(value);

                match boundary_crossed {
                    true => 5,
                    false => 4,
                }
            }
            AddressingMode::IndirectXIndexed => {
                let value = indirect_x_read(self, low_byte);
                self.cpy_intermediate(value);

                6
            }
            AddressingMode::IndirectYIndexed => {
                let (value, boundary_crossed) = indirect_y_read(self, low_byte);
                self.cpy_intermediate(value);

                match boundary_crossed {
                    true => 6,
                    false => 5,
                }
            }
            _ => handle_invalid_addressing_mode(),
        }
    }
}

impl<M: Mapper, I: Interrupts> Cpu<M, I> {
    /// The intermediate code for ADC. Modifies the accumulator inside this method.
    fn adc_intermediate(&mut self, value: u8) {
        // If the sign bits are the same, then we need to check if they
        // are different later because that is an overflow.
        // If the sign bits are the same, we keep the sign in Some(), otherwise
        // it is None and we don't need to check for overflow
        let shared_sign = match (self.accumulator >> 7) == (value >> 7) {
            true => Some(self.accumulator >> 7),
            false => None,
        };

        // store whether we need a carry before modifying the values
        let carry_needed =
            (value as u16 + self.accumulator as u16 + self.processor_status.carry_flag() as u16)
                > 255;

        self.accumulator = self.accumulator.wrapping_add(value);
        self.accumulator = self
            .accumulator
            .wrapping_add(self.processor_status.carry_flag() as u8);

        // Modify the carry flag
        match carry_needed {
            true => self.processor_status.set_carry_flag(),
            false => self.processor_status.clear_carry_flag(),
        };

        // Modify the overflow flag
        // If the signs were the same before the operation, they need to
        // have the same sign as the result
        match shared_sign {
            Some(x) => match (self.accumulator >> 7) == (x) {
                true => self.processor_status.clear_overflow_flag(),
                false => self.processor_status.set_overflow_flag(),
            },
            None => self.processor_status.clear_overflow_flag(),
        }

        // Modify zero and negative flag
        self.modify_zero_flag(self.accumulator);
        self.modify_negative_flag(self.accumulator);
    }

    /// The intermediate code for SBC. Modifies the accumulator inside this method.
    fn sbc_intermediate(&mut self, value: u8) {
        // We can do a bit of twos comp math and simplify the operation to ADC(value ^ 0xFF).
        // The forum post on this is here: https://forums.nesdev.org/viewtopic.php?t=8703
        self.adc_intermediate(value ^ 0xFF);
    }

    fn cmp_intermediate(&mut self, value: u8) {
        let compared_value = self.accumulator.wrapping_sub(value);

        match self.accumulator >= value {
            true => self.processor_status.set_carry_flag(),
            false => self.processor_status.clear_carry_flag(),
        }

        self.modify_zero_flag(compared_value);
        self.modify_negative_flag(compared_value);
    }

    fn cpx_intermediate(&mut self, value: u8) {
        let compared_value = self.x.wrapping_sub(value);

        match self.x >= value {
            true => self.processor_status.set_carry_flag(),
            false => self.processor_status.clear_carry_flag(),
        }

        self.modify_zero_flag(compared_value);
        self.modify_negative_flag(compared_value);
    }

    fn cpy_intermediate(&mut self, value: u8) {
        let compared_value = self.y.wrapping_sub(value);

        match self.y >= value {
            true => self.processor_status.set_carry_flag(),
            false => self.processor_status.clear_carry_flag(),
        }

        self.modify_zero_flag(compared_value);
        self.modify_negative_flag(compared_value);
    }
}
