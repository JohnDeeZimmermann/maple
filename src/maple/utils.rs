use crate::maple::cpu::MapleCPU;
use crate::maple::interrupt_codes::INTERRUPT_CODE_ILLEGAL_DIRECT_ARGUMENT;

pub struct ConditionalResult {
    pub parity: bool,
    pub negative: bool,
    pub zero: bool,
    pub overflow: bool,
}

pub enum DirectArgumentSignMode {
    Unsigned,
    Signed { bit_width: u8 },
}

pub fn extract_from_binary_left(value: u64, section_size: u32, section_left_offset: u64) -> u64 {
    let section_right_offset = 64_u64 - section_left_offset - section_size as u64;
    extract_from_binary_right(value, section_size, section_right_offset)
}

pub fn extract_from_binary_right(value: u64, section_size: u32, section_right_offset: u64) -> u64 {
    let mask = if section_size >= 64 {
        u64::MAX
    } else {
        (1_u64 << section_size) - 1
    };

    (value >> section_right_offset) & mask
}

pub fn resolve_argument_value(
    cpu: &MapleCPU,
    argument: u64,
    direct_sign_mode: DirectArgumentSignMode,
    forced_sign_bit: Option<u64>,
) -> u64 {
    let is_register = (argument & 1) == 1;
    let resolved_value = if is_register {
        let reg_num = extract_from_binary_right(argument, 4, 1) as u8;
        cpu.get_register(reg_num)
    } else {
        let direct_value = argument >> 1;
        match direct_sign_mode {
            DirectArgumentSignMode::Unsigned => direct_value,
            DirectArgumentSignMode::Signed { bit_width } => sign_extend(direct_value, bit_width),
        }
    };

    match forced_sign_bit {
        Some(sign_bit) => apply_sign_bit(resolved_value, sign_bit),
        None => resolved_value,
    }
}

pub fn resolve_potential_register_argument_value(cpu: &MapleCPU, argument: u64) -> u64 {
    resolve_argument_value(cpu, argument, DirectArgumentSignMode::Unsigned, None)
}

pub fn resolve_signed_potential_register_argument_value(cpu: &MapleCPU, argument: u64) -> i64 {
    resolve_argument_value(
        cpu,
        argument,
        DirectArgumentSignMode::Signed { bit_width: 23 },
        None,
    ) as i64
}

pub fn resolve_required_register_argument_value(cpu: &mut MapleCPU, argument: u64) -> u64 {
    let is_register = (argument & 1) == 1;
    if !is_register {
        cpu.raise_interrupt(INTERRUPT_CODE_ILLEGAL_DIRECT_ARGUMENT);
        return 0;
    }
    let reg_num = extract_from_binary_right(argument, 4, 1) as u8;
    cpu.get_register(reg_num)
}

pub fn place_value_in_binary_from_right(value: u64, position: u8, slice_size: u8) -> u64 {
    if position > 64 {
        panic!("Position provided may not be smaller than 0.")
    }

    place_value_in_binary_from_left(value, position, slice_size)
}

pub fn place_value_in_binary_from_left(value: u64, position: u8, slice_size: u8) -> u64 {
    (value & (2_u64.pow(slice_size as u32) - 1)) << position
}

pub fn apply_sign_bit(value: u64, sign: u64) -> u64 {
    (sign << 63) | value
}

pub fn sign_extend(value: u64, bit_width: u8) -> u64 {
    if bit_width == 0 || bit_width > 64 {
        panic!("Bit width must be between 1 and 64.")
    }

    if bit_width == 64 {
        return value;
    }

    let value_mask = (1_u64 << bit_width) - 1;
    let masked_value = value & value_mask;
    let sign_mask = 1_u64 << (bit_width - 1);

    if masked_value & sign_mask == 0 {
        masked_value
    } else {
        masked_value | !value_mask
    }
}

pub fn get_conditional_result(cpu: &MapleCPU) -> ConditionalResult {
    let register = cpu.get_result_register();

    return ConditionalResult {
        parity: (register >> 3) & 1 == 1,
        negative: (register >> 2) & 1 == 1,
        zero: (register >> 1) & 1 == 1,
        overflow: register & 1 == 1,
    };
}
