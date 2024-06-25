use super::{AddressingMode, Cpu};

// We organize the instructions using modules according to the
// categories used on https://www.nesdev.org/obelisk-6502-guide/instructions.html
mod arithmetic;
mod branches;
mod incr_decr;
mod jumps_calls;
mod load_store;
mod logical;
mod register_transfers;
mod shifts;
mod stack;
mod status_flags;
mod system;

impl Cpu {
    /// Sets the zero flag if the given byte is 0.
    fn modify_zero_flag(&mut self, byte: u8) {
        match byte == 0 {
            true => self.processor_status.set_zero_flag(),
            false => self.processor_status.clear_zero_flag(),
        }
    }

    /// Sets the negative flag given byte is negative (in two's compliment)
    fn modify_negative_flag(&mut self, byte: u8) {
        match byte >> 7 != 0 {
            true => self.processor_status.set_negative_flag(),
            false => self.processor_status.clear_negative_flag(),
        }
    }

    /// Sets the overflow flag if an overflow ocurred.
    fn modify_overflow_flag(&mut self, op1: u8, op2: u8) {
        let op1_sign = op1 >> 7;
        let op2_sign = op2 >> 7;

        let result = op1 + op2;

        // If the signs were the same and are different from the result,
        // we have an overflow.
        match op1_sign == op2_sign {
            true => match op1_sign == result >> 7 {
                true => self.processor_status.clear_overflow_flag(),
                false => self.processor_status.set_overflow_flag(),
            },
            false => self.processor_status.clear_overflow_flag(),
        }
    }

    /// Sets the carry flag if a carry out ocurred.
    fn modify_carry_flag(&mut self, op1: u8, op2: u8) {
        match op1.checked_add(op2).is_none() {
            true => self.processor_status.set_carry_flag(),
            false => self.processor_status.clear_carry_flag(),
        }
    }

    // Pushes a value from the stack
    fn push(&mut self, byte: u8) {
        self.write(0x0100 | self.stack_pointer as u16, byte);

        self.stack_pointer = match self.stack_pointer.checked_sub(1) {
            Some(x) => x,
            None => panic!("Cpu stack overflow"),
        };
    }

    // Pops a value from the stack
    fn pop(&mut self) -> u8 {
        self.stack_pointer = match self.stack_pointer.checked_add(1) {
            Some(x) => x,
            None => panic!("Cpu stack underflow"),
        };

        let byte = self.read(0x0100 | self.stack_pointer as u16);

        byte
    }
}

fn handle_invalid_addressing_mode() -> ! {
    panic!("Invalid addressing mode")
}

fn unpack_bytes(packed: u16) -> (u8, u8) {
    ((packed & 0xFF) as u8, ((packed >> 8) & 0xFF) as u8)
}

fn pack_bytes(low_byte: u8, high_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | low_byte as u16
}

fn pack_bytes_wrapped(low_byte: Option<u8>, high_byte: Option<u8>) -> u16 {
    ((high_byte.unwrap() as u16) << 8) | low_byte.unwrap() as u16
}

fn twos_compliment_to_signed(value: u8) -> i8 {
    match (value >> 7) != 0 {
        true => {
            let negative = (!value).wrapping_add(1);

            // we check for the case that we had -128, which wouldnt be converted
            match (negative == 0b1000_0000) {
                true => -128,
                false => -(negative as i8),
            }
        }
        false => value as i8,
    }
}

// rough and dirty addressing shortcuts
fn immediate_read(low_byte: Option<u8>) -> u8 {
    low_byte.unwrap()
}

fn zeropage_read(cpu: &Cpu, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap() as u16;
    cpu.read(address)
}

// value is the value written to memory
fn zeropage_write(cpu: &mut Cpu, low_byte: Option<u8>, value: u8) {
    let address = low_byte.unwrap() as u16;
    cpu.write(address, value);
}

fn zeropage_x_read(cpu: &Cpu, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap().wrapping_add(cpu.x) as u16;
    cpu.read(address)
}

fn zeropage_x_write(cpu: &mut Cpu, low_byte: Option<u8>, value: u8) {
    let address = low_byte.unwrap().wrapping_add(cpu.x) as u16;
    cpu.write(address, value);
}

fn zeropage_y_read(cpu: &Cpu, low_byte: Option<u8>) -> u8 {
    let address = low_byte.unwrap().wrapping_add(cpu.y) as u16;
    cpu.read(address)
}

fn zeropage_y_write(cpu: &mut Cpu, low_byte: Option<u8>, value: u8) {
    let address = low_byte.unwrap().wrapping_add(cpu.y) as u16;
    cpu.write(address, value);
}

fn absolute_read(cpu: &Cpu, low_byte: Option<u8>, high_byte: Option<u8>) -> u8 {
    let address = pack_bytes_wrapped(low_byte, high_byte);
    cpu.read(address)
}

fn absolute_write(cpu: &mut Cpu, low_byte: Option<u8>, high_byte: Option<u8>, value: u8) {
    let address = pack_bytes_wrapped(low_byte, high_byte);
    cpu.write(address, value);
}

/// Returns the value and whether a page boundary was crossed.
fn absolute_x_read(cpu: &Cpu, low_byte: Option<u8>, high_byte: Option<u8>) -> (u8, bool) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.x as u16);

    let page_changed = low_byte.unwrap().checked_add(cpu.x).is_none();

    (cpu.read(address), page_changed)
}

fn absolute_x_write(cpu: &mut Cpu, low_byte: Option<u8>, high_byte: Option<u8>, value: u8) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.x as u16);
    cpu.write(address, value);
}

/// Returns the value and whether a page boundary was crossed.
fn absolute_y_read(cpu: &Cpu, low_byte: Option<u8>, high_byte: Option<u8>) -> (u8, bool) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.y as u16);

    let page_changed = low_byte.unwrap().checked_add(cpu.y).is_none();

    (cpu.read(address), page_changed)
}

fn absolute_y_write(cpu: &mut Cpu, low_byte: Option<u8>, high_byte: Option<u8>, value: u8) {
    let pre_add_address = pack_bytes_wrapped(low_byte, high_byte);
    let address = pre_add_address.wrapping_add(cpu.y as u16);
    cpu.write(address, value);
}

fn indirect_x_read(cpu: &Cpu, low_byte: Option<u8>) -> u8 {
    let base_address = low_byte.unwrap().wrapping_add(cpu.x) as u16;

    let resolved_address = pack_bytes(cpu.read(base_address), cpu.read(base_address + 1));

    cpu.read(resolved_address)
}

fn indirect_x_write(cpu: &mut Cpu, low_byte: Option<u8>, value: u8) {
    let base_address = low_byte.unwrap().wrapping_add(cpu.x) as u16;

    let resolved_address = pack_bytes(cpu.read(base_address), cpu.read(base_address + 1));

    cpu.write(resolved_address, value);
}

fn indirect_y_read(cpu: &Cpu, low_byte: Option<u8>) -> (u8, bool) {
    let low_base_address = low_byte.unwrap() as u16;
    let high_base_address = low_byte.unwrap().wrapping_add(1) as u16;

    let page_changed = low_base_address > high_base_address;

    let resolved_address =
        pack_bytes(cpu.read(low_base_address), cpu.read(high_base_address)) + cpu.y as u16;

    (cpu.read(resolved_address), page_changed)
}

fn indirect_y_write(cpu: &mut Cpu, low_byte: Option<u8>, value: u8) {
    let low_base_address = low_byte.unwrap() as u16;
    let high_base_address = low_byte.unwrap().wrapping_add(1) as u16;

    let resolved_address =
        pack_bytes(cpu.read(low_base_address), cpu.read(high_base_address)) + cpu.y as u16;

    cpu.write(resolved_address, value);
}