use crate::assembler::parser::errors::parse_errors::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum AstValue {
    IntegerValue(u64),
    SignedIntegerValue(i64),
    FloatValue(f64),
    Register(u8),
    LabelAddress(u32),
}

impl AstValue {
    pub fn to_register_index(&self) -> Result<u8, ParseError> {
        match self {
            AstValue::Register(reg) => Ok(*reg),
            AstValue::IntegerValue(int) => {
                if *int < 16 {
                    Ok(*int as u8)
                } else {
                    Err(ParseError::IllegalArgumentError(
                        format!("Integer value out of range: {}", int)
                    ))
                }
            }
            _ => Err(ParseError::IllegalArgumentError(
                format!("Expected a register value, got {:?}", self)
            )),
        }
    }

    pub fn to_potential_register_argument(
        &self,
        available_bytes: u32,
        allow_sign: bool,
    ) -> Result<u64, ParseError> {
        match self {
            AstValue::Register(reg) => Ok(((*reg as u64) << 1) | 1),
            AstValue::IntegerValue(int) => {
                let value = *int;
                // When signs are allowed, there is an empty 0 bit to the left
                let needed_extra_bits = if allow_sign { 2 } else { 1 };
                if value > 2_u64.pow(available_bytes - needed_extra_bits) {
                    Err(ParseError::IllegalArgumentError(format!(
                        "Integer value out of range: {} for length {}",
                        int, available_bytes
                    )))
                } else {
                    Ok(value << 1)
                }
            }
            AstValue::SignedIntegerValue(sigint) => {
                if !allow_sign {
                    return Err(ParseError::IllegalArgumentError(format!(
                        "Signed integer value {} not allowed.",
                        sigint
                    )));
                }

                let sigint = *sigint;
                let abs = sigint.abs() as u64;
                if abs > 2_u64.pow(available_bytes + 1) {
                    Err(ParseError::IllegalArgumentError(format!(
                        "Signed integer value out of range: {}",
                        sigint
                    )))
                } else {
                    let signbit: u64 = if sigint < 0 { 0 } else { 1 };
                    // Setting sign bit to the left of the available range
                    Ok((signbit << ((available_bytes as u64) - 1)) | (abs as u64) << 1)
                }
            }
            AstValue::LabelAddress(address) => {
                let address = *address;
                // When signs are allowed, there is an empty 0 bit to the left
                let needed_extra_bits = if allow_sign { 2 } else { 1 };
                if address > 2_u32.pow(available_bytes + needed_extra_bits) {
                    Err(ParseError::IllegalArgumentError(format!(
                        "Label address out of range: {}",
                        address
                    )))
                } else {
                    Ok((address as u64) << 1)
                }
            }

            _ => Err(ParseError::IllegalArgumentError(format!(
                "Unexpected value type: {:?}",
                self
            ))),
        }
    }
}
