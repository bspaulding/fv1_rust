//! FV-1 Instruction Decoder
//!
//! Converts 32-bit FV-1 machine code back to AST instructions

use crate::{
    error::CodegenError,
    instruction::{ChoFlags, ChoMode, Instruction, SkipCondition},
    register::{Lfo, Register},
};

/// Decode a 32-bit FV-1 machine code instruction
pub fn decode_instruction(word: u32) -> Result<Instruction, CodegenError> {
    // Special case for NOP (all zeros)
    if word == 0x00000000 {
        return Ok(Instruction::NOP);
    }

    let opcode = (word >> 27) & 0x1F;

    match opcode {
        // Accumulator operations
        0b00000 => {
            // RDAX
            let reg = decode_register((word >> 21) & 0x3F)?;
            let coeff = decode_s114((word >> 6) & 0x7FFF)?;
            Ok(Instruction::RDAX { reg, coeff })
        }

        0b00001 => {
            // RDA
            let addr = ((word >> 11) & 0xFFFF) as u16;
            let coeff = decode_s114(word & 0x7FF)?;
            Ok(Instruction::RDA { addr, coeff })
        }

        0b00010 => {
            // RMPA
            let coeff = decode_s114(word & 0x7FFFFF)?;
            Ok(Instruction::RMPA { coeff })
        }

        0b00110 => {
            // WRAX
            let reg = decode_register((word >> 21) & 0x3F)?;
            let coeff = decode_s114((word >> 6) & 0x7FFF)?;
            Ok(Instruction::WRAX { reg, coeff })
        }

        0b00111 => {
            // WRA
            let addr = ((word >> 11) & 0xFFFF) as u16;
            let coeff = decode_s114(word & 0x7FF)?;
            Ok(Instruction::WRA { addr, coeff })
        }

        0b01000 => {
            // WRAP
            let addr = ((word >> 11) & 0xFFFF) as u16;
            let coeff = decode_s114(word & 0x7FF)?;
            Ok(Instruction::WRAP { addr, coeff })
        }

        // Mathematical operations
        0b01010 => {
            // MULX
            let reg = decode_register((word >> 21) & 0x3F)?;
            Ok(Instruction::MULX { reg })
        }

        0b01001 => {
            // RDFX
            let reg = decode_register((word >> 21) & 0x3F)?;
            let coeff = decode_s114((word >> 6) & 0x7FFF)?;
            Ok(Instruction::RDFX { reg, coeff })
        }

        0b01100 => {
            // RDFX2
            let reg = decode_register((word >> 21) & 0x3F)?;
            let coeff = decode_s114((word >> 6) & 0x7FFF)?;
            Ok(Instruction::RDFX2 { reg, coeff })
        }

        0b00101 => {
            // LDAX
            let reg = decode_register((word >> 21) & 0x3F)?;
            Ok(Instruction::LDAX { reg })
        }

        0b01011 => {
            // ABSA
            Ok(Instruction::ABSA)
        }

        // Logic and control
        0b01101 => {
            // SOF
            let coeff = decode_s114((word >> 11) & 0xFFFF)?;
            let offset = decode_s10(word & 0x7FF)?;
            Ok(Instruction::SOF { coeff, offset })
        }

        0b01111 => {
            // AND
            let mask = word & 0xFFFFFF;
            Ok(Instruction::AND { mask })
        }

        0b10000 => {
            // OR
            let mask = word & 0xFFFFFF;
            Ok(Instruction::OR { mask })
        }

        0b10001 => {
            // XOR
            let mask = word & 0xFFFFFF;
            Ok(Instruction::XOR { mask })
        }

        0b10010 => {
            // SHL
            Ok(Instruction::SHL)
        }

        0b10011 => {
            // SHR
            Ok(Instruction::SHR)
        }

        0b01110 => {
            // CLR
            Ok(Instruction::CLR)
        }

        // Conversion operations
        0b10100 => {
            // EXP
            let coeff = decode_s114((word >> 11) & 0xFFFF)?;
            let offset = decode_s10(word & 0x7FF)?;
            Ok(Instruction::EXP { coeff, offset })
        }

        0b10101 => {
            // LOG
            let coeff = decode_s114((word >> 11) & 0xFFFF)?;
            let offset = decode_s10(word & 0x7FF)?;
            Ok(Instruction::LOG { coeff, offset })
        }

        // Conditional skipping
        0b10110 => {
            // SKP
            let condition = decode_skip_condition((word >> 24) & 0x07)?;
            let offset = ((word >> 18) & 0x3F) as i8;
            Ok(Instruction::SKP { condition, offset })
        }

        // LFO control
        0b10111 => {
            // WLDS
            let lfo = decode_lfo((word >> 25) & 0x03)?;
            let freq = ((word >> 9) & 0x1FF) as u16;
            let amplitude = (word & 0x1FF) as u16;
            Ok(Instruction::WLDS {
                lfo,
                freq,
                amplitude,
            })
        }

        0b11000 => {
            // JAM
            let lfo = decode_lfo((word >> 25) & 0x03)?;
            Ok(Instruction::JAM { lfo })
        }

        0b11001 => {
            // CHO
            let mode = decode_cho_mode((word >> 24) & 0x03)?;
            let lfo = decode_lfo((word >> 22) & 0x03)?;
            let flags = decode_cho_flags((word >> 16) & 0x3F);
            let addr = (word & 0xFFFF) as u16;
            Ok(Instruction::CHO {
                mode,
                lfo,
                flags,
                addr,
            })
        }

        _ => Err(CodegenError::InvalidOpcode { opcode: opcode as u8 }),
    }
}

/// Decode register from 5-bit or 6-bit field
fn decode_register(bits: u32) -> Result<Register, CodegenError> {
    match bits {
        0b00000 => Ok(Register::ADCL),
        0b00001 => Ok(Register::ADCR),
        0b00010 => Ok(Register::DACL),
        0b00011 => Ok(Register::DACR),
        0b00100 => Ok(Register::ADDR_PTR),
        0b00101 => Ok(Register::LR),
        n if n >= 16 && n < 48 => Ok(Register::REG((n - 16) as u8)),
        _ => Err(CodegenError::InvalidRegister { bits: bits as u8 }),
    }
}

/// Decode S1.14 fixed-point coefficient
fn decode_s114(bits: u32) -> Result<f32, CodegenError> {
    // Convert from S1.14 format: 15-bit signed value (1 sign + 14 fractional)
    // The sign bit is bit 14 (0x4000)
    let value = if bits & 0x4000 != 0 {
        // Negative: sign extend from 15 bits to 32 bits
        ((bits | 0xFFFF8000) as i32) as f32 / 16384.0
    } else {
        // Positive
        (bits as i32) as f32 / 16384.0
    };
    Ok(value)
}

/// Decode S.10 fixed-point coefficient
fn decode_s10(bits: u32) -> Result<f32, CodegenError> {
    // Convert from S.10 format: 11-bit signed value (1 sign + 10 fractional)
    // The sign bit is bit 10 (0x400)
    let value = if bits & 0x400 != 0 {
        // Negative: sign extend from 11 bits to 32 bits
        ((bits | 0xFFFFF800) as i32) as f32 / 512.0
    } else {
        // Positive
        (bits as i32) as f32 / 512.0
    };
    Ok(value)
}

/// Decode skip condition from 3-bit field
fn decode_skip_condition(bits: u32) -> Result<SkipCondition, CodegenError> {
    match bits {
        0b000 => Ok(SkipCondition::RUN),
        0b001 => Ok(SkipCondition::NEG),
        0b010 => Ok(SkipCondition::GEZ),
        0b011 => Ok(SkipCondition::ZRO),
        0b100 => Ok(SkipCondition::ZRC),
        _ => Err(CodegenError::InvalidSkipCondition { bits: bits as u8 }),
    }
}

/// Decode LFO from 2-bit field
fn decode_lfo(bits: u32) -> Result<Lfo, CodegenError> {
    match bits {
        0b00 => Ok(Lfo::SIN0),
        0b01 => Ok(Lfo::SIN1),
        0b10 => Ok(Lfo::RMP0),
        0b11 => Ok(Lfo::RMP1),
        _ => Err(CodegenError::InvalidLfo { bits: bits as u8 }),
    }
}

/// Decode CHO mode from 2-bit field
fn decode_cho_mode(bits: u32) -> Result<ChoMode, CodegenError> {
    match bits {
        0b00 => Ok(ChoMode::RDA),
        0b10 => Ok(ChoMode::SOF),
        0b11 => Ok(ChoMode::RDAL),
        _ => Err(CodegenError::InvalidChoMode { bits: bits as u8 }),
    }
}

/// Decode CHO flags from 6-bit field
fn decode_cho_flags(bits: u32) -> ChoFlags {
    ChoFlags {
        rptr2: (bits & 0b100000) != 0,
        na: (bits & 0b010000) != 0,
        compc: (bits & 0b001000) != 0,
        compa: (bits & 0b000100) != 0,
        rptr2_select: (bits & 0b000010) != 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::encoder::encode_instruction;

    #[test]
    fn test_decode_rdax() {
        let inst = Instruction::RDAX {
            reg: Register::ADCL,
            coeff: 0.5,
        };
        let encoded = encode_instruction(&inst).unwrap();
        let decoded = decode_instruction(encoded).unwrap();
        assert_eq!(decoded, inst);
    }

    #[test]
    fn test_decode_wrax() {
        let inst = Instruction::WRAX {
            reg: Register::DACL,
            coeff: 0.5,
        };
        let encoded = encode_instruction(&inst).unwrap();
        let decoded = decode_instruction(encoded).unwrap();
        assert_eq!(decoded, inst);
    }

    #[test]
    fn test_decode_sof() {
        let inst = Instruction::SOF {
            coeff: 0.75,
            offset: 0.0,
        };
        let encoded = encode_instruction(&inst).unwrap();
        let decoded = decode_instruction(encoded).unwrap();
        assert_eq!(decoded, inst);
    }

    #[test]
    fn test_decode_clr() {
        let inst = Instruction::CLR;
        let encoded = encode_instruction(&inst).unwrap();
        let decoded = decode_instruction(encoded).unwrap();
        assert_eq!(decoded, inst);
    }

    #[test]
    fn test_decode_nop() {
        let inst = Instruction::NOP;
        let encoded = encode_instruction(&inst).unwrap();
        let decoded = decode_instruction(encoded).unwrap();
        assert_eq!(decoded, inst);
    }

    #[test]
    fn test_roundtrip_all_instructions() {
        let instructions = vec![
            Instruction::RDAX {
                reg: Register::ADCL,
                coeff: 0.5,
            },
            Instruction::WRAX {
                reg: Register::DACL,
                coeff: 0.75,
            },
            Instruction::SOF {
                coeff: 0.75,
                offset: -0.25,
            },
            Instruction::CLR,
            Instruction::NOP,
            Instruction::MULX {
                reg: Register::REG(0),
            },
        ];

        for inst in instructions {
            let encoded = encode_instruction(&inst).unwrap();
            let decoded = decode_instruction(encoded).unwrap();
            assert_eq!(decoded, inst);
        }
    }
}
