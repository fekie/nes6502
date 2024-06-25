use super::{handle_invalid_addressing_mode, pack_bytes, pack_bytes_wrapped, unpack_bytes};
use super::{AddressingMode, Cpu};

impl Cpu {
    pub(crate) fn instruction_jmp(
        &mut self,

        addressing_mode: AddressingMode,
        low_byte: Option<u8>,
        high_byte: Option<u8>,
    ) -> u8 {
        match addressing_mode {
            AddressingMode::Absolute => {
                self.program_counter = pack_bytes_wrapped(low_byte, high_byte);
                3
            }
            AddressingMode::Indirect => {
                // We do an indrect read here. We do not have a general function
                // as JMP is the only instruction that uses it
                let base_address = pack_bytes_wrapped(low_byte, high_byte);
                self.program_counter =
                    pack_bytes(self.read(base_address), self.read(base_address + 1));

                5
            }
            _ => handle_invalid_addressing_mode(),
        }
    }

    pub(crate) fn instruction_jsr(&mut self, low_byte: Option<u8>, high_byte: Option<u8>) -> u8 {
        let subroutine_address = pack_bytes_wrapped(low_byte, high_byte);

        let (pc_low, pc_high) = unpack_bytes(self.program_counter);

        self.push(pc_high);
        self.push(pc_low);

        self.program_counter = subroutine_address;

        6
    }

    pub(crate) fn instruction_rts(&mut self) -> u8 {
        let pc_low = self.pop();
        let pc_high: u8 = self.pop();

        self.program_counter = pack_bytes(pc_low, pc_high);

        6
    }
}
