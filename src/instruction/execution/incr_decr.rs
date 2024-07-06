use super::{
    absolute_read, absolute_write, absolute_x_read, absolute_x_write,
    handle_invalid_addressing_mode, zeropage_read, zeropage_write, zeropage_x_read,
    zeropage_x_write,
};
use super::{AddressingMode, Cpu};
use crate::Mapper;

impl<M: Mapper> Cpu<M> {
    pub(crate) fn instruction_inc(
        &mut self,

        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Zeropage => {
                let mut value = zeropage_read(self, low_byte);

                value = value.wrapping_add(1);

                self.modify_zero_flag(value);
                self.modify_negative_flag(value);

                zeropage_write(self, low_byte, value);

                5
            }
            AddressingMode::ZeropageXIndexed => {
                let mut value = zeropage_x_read(self, low_byte);

                value = value.wrapping_add(1);

                self.modify_zero_flag(value);
                self.modify_negative_flag(value);

                zeropage_x_write(self, low_byte, value);

                6
            }
            AddressingMode::Absolute => {
                let mut value = absolute_read(self, low_byte, high_byte);

                value = value.wrapping_add(1);

                self.modify_zero_flag(value);
                self.modify_negative_flag(value);

                absolute_write(self, low_byte, high_byte, value);

                6
            }
            AddressingMode::AbsoluteXIndexed => {
                let (mut value, _) = absolute_x_read(self, low_byte, high_byte);

                value = value.wrapping_add(1);

                self.modify_zero_flag(value);
                self.modify_negative_flag(value);

                absolute_x_write(self, low_byte, high_byte, value);

                7
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_inx(&mut self) -> u8 {
        self.x = self.x.wrapping_add(1);

        self.modify_zero_flag(self.x);
        self.modify_negative_flag(self.x);

        2
    }

    pub(crate) fn instruction_iny(&mut self) -> u8 {
        self.y = self.y.wrapping_add(1);

        self.modify_zero_flag(self.y);
        self.modify_negative_flag(self.y);

        2
    }

    pub(crate) fn instruction_dec(
        &mut self,

        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Zeropage => {
                let mut value = zeropage_read(self, low_byte);

                value = value.wrapping_sub(1);

                self.modify_zero_flag(value);
                self.modify_negative_flag(value);

                zeropage_write(self, low_byte, value);

                5
            }
            AddressingMode::ZeropageXIndexed => {
                let mut value = zeropage_x_read(self, low_byte);

                value = value.wrapping_sub(1);

                self.modify_zero_flag(value);
                self.modify_negative_flag(value);

                zeropage_x_write(self, low_byte, value);

                6
            }
            AddressingMode::Absolute => {
                let mut value = absolute_read(self, low_byte, high_byte);

                value = value.wrapping_sub(1);

                self.modify_zero_flag(value);
                self.modify_negative_flag(value);

                absolute_write(self, low_byte, high_byte, value);

                6
            }
            AddressingMode::AbsoluteXIndexed => {
                let (mut value, _) = absolute_x_read(self, low_byte, high_byte);

                value = value.wrapping_sub(1);

                self.modify_zero_flag(value);
                self.modify_negative_flag(value);

                absolute_x_write(self, low_byte, high_byte, value);

                7
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_dex(&mut self) -> u8 {
        self.x = self.x.wrapping_sub(1);

        self.modify_zero_flag(self.x);
        self.modify_negative_flag(self.x);

        2
    }

    pub(crate) fn instruction_dey(&mut self) -> u8 {
        self.y = self.y.wrapping_sub(1);

        self.modify_zero_flag(self.y);
        self.modify_negative_flag(self.y);

        2
    }
}
