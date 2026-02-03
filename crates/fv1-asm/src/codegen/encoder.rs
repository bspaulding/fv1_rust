//! FV-1 Instruction Encoder
//!
//! Converts AST instructions to 32-bit FV-1 machine code

use crate::{
    constants::DELAY_RAM_SIZE,
    error::CodegenError,
    instruction::{ChoFlags, ChoMode, Instruction, SkipCondition},
    register::{Lfo, Register},
};

/// Encode a single instruction to 32-bit FV-1 machine code
pub fn encode_instruction(inst: &Instruction) -> Result<u32, CodegenError> {
    match inst {
        // Accumulator operations
        Instruction::RDAX { reg, coeff } => {
            let opcode = 0b00000_u32 << 27;
            let reg_bits = encode_register(reg)? << 21;
            let coeff_bits = (encode_s114(*coeff)? & 0x7FFF) << 6;
            Ok(opcode | reg_bits | coeff_bits)
        }

        Instruction::RDA { addr, coeff } => {
            let opcode = 0b00001_u32 << 27;
            let addr_bits = encode_address(*addr)? << 11;
            let coeff_bits = encode_s114(*coeff)? & 0x7FF;
            Ok(opcode | addr_bits | coeff_bits)
        }

        Instruction::RMPA { coeff } => {
            let opcode = 0b00010_u32 << 27;
            let coeff_bits = encode_s114(*coeff)? & 0x7FFFFF;
            Ok(opcode | coeff_bits)
        }

        Instruction::WRAX { reg, coeff } => {
            let opcode = 0b00110_u32 << 27;
            let reg_bits = encode_register(reg)? << 21;
            let coeff_bits = (encode_s114(*coeff)? & 0x7FFF) << 6;
            Ok(opcode | reg_bits | coeff_bits)
        }

        Instruction::WRA { addr, coeff } => {
            let opcode = 0b00111_u32 << 27;
            let addr_bits = encode_address(*addr)? << 11;
            let coeff_bits = encode_s114(*coeff)? & 0x7FF;
            Ok(opcode | addr_bits | coeff_bits)
        }

        Instruction::WRAP { addr, coeff } => {
            let opcode = 0b01000_u32 << 27;
            let addr_bits = encode_address(*addr)? << 11;
            let coeff_bits = encode_s114(*coeff)? & 0x7FF;
            Ok(opcode | addr_bits | coeff_bits)
        }

        // Mathematical operations
        Instruction::MULX { reg } => {
            let opcode = 0b01010_u32 << 27;
            let reg_bits = encode_register(reg)? << 21;
            Ok(opcode | reg_bits)
        }

        Instruction::RDFX { reg, coeff } => {
            let opcode = 0b01001_u32 << 27;
            let reg_bits = encode_register(reg)? << 21;
            let coeff_bits = (encode_s114(*coeff)? & 0x7FFF) << 6;
            Ok(opcode | reg_bits | coeff_bits)
        }

        Instruction::RDFX2 { reg, coeff } => {
            let opcode = 0b01100_u32 << 27;
            let reg_bits = encode_register(reg)? << 21;
            let coeff_bits = (encode_s114(*coeff)? & 0x7FFF) << 6;
            Ok(opcode | reg_bits | coeff_bits)
        }

        Instruction::LDAX { reg } => {
            let opcode = 0b00101_u32 << 27;
            let reg_bits = encode_register(reg)? << 21;
            Ok(opcode | reg_bits)
        }

        Instruction::ABSA => {
            let opcode = 0b01011_u32 << 27;
            Ok(opcode)
        }

        // Logic and control
        Instruction::SOF { coeff, offset } => {
            let opcode = 0b01101_u32 << 27;
            let coeff_bits = (encode_s114(*coeff)? & 0xFFFF) << 11;
            let offset_bits = encode_s10(*offset)? & 0x7FF;
            Ok(opcode | coeff_bits | offset_bits)
        }

        Instruction::AND { mask } => {
            let opcode = 0b01111_u32 << 27;
            let mask_bits = mask & 0xFFFFFF;
            Ok(opcode | mask_bits)
        }

        Instruction::OR { mask } => {
            let opcode = 0b10000_u32 << 27;
            let mask_bits = mask & 0xFFFFFF;
            Ok(opcode | mask_bits)
        }

        Instruction::XOR { mask } => {
            let opcode = 0b10001_u32 << 27;
            let mask_bits = mask & 0xFFFFFF;
            Ok(opcode | mask_bits)
        }

        Instruction::SHL => {
            let opcode = 0b10010_u32 << 27;
            Ok(opcode)
        }

        Instruction::SHR => {
            let opcode = 0b10011_u32 << 27;
            Ok(opcode)
        }

        Instruction::CLR => {
            let opcode = 0b01110_u32 << 27;
            Ok(opcode)
        }

        Instruction::NOP => {
            // NOP is encoded as a no-op instruction
            Ok(0x00000000)
        }

        // Conversion operations
        Instruction::EXP { coeff, offset } => {
            let opcode = 0b10100_u32 << 27;
            let coeff_bits = (encode_s114(*coeff)? & 0xFFFF) << 11;
            let offset_bits = encode_s10(*offset)? & 0x7FF;
            Ok(opcode | coeff_bits | offset_bits)
        }

        Instruction::LOG { coeff, offset } => {
            let opcode = 0b10101_u32 << 27;
            let coeff_bits = (encode_s114(*coeff)? & 0xFFFF) << 11;
            let offset_bits = encode_s10(*offset)? & 0x7FF;
            Ok(opcode | coeff_bits | offset_bits)
        }

        // Conditional skipping
        Instruction::SKP { condition, offset } => {
            let opcode = 0b10110_u32 << 27;
            let cond_bits = encode_skip_condition(*condition) << 24;
            let offset_bits = (*offset as u32 & 0x3F) << 18;
            Ok(opcode | cond_bits | offset_bits)
        }

        // LFO control
        Instruction::WLDS {
            lfo,
            freq,
            amplitude,
        } => {
            let opcode = 0b10111_u32 << 27;
            let lfo_bits = encode_lfo(*lfo) << 25;
            let freq_bits = (*freq as u32 & 0x1FF) << 9;
            let amp_bits = *amplitude as u32 & 0x1FF;
            Ok(opcode | lfo_bits | freq_bits | amp_bits)
        }

        Instruction::JAM { lfo } => {
            let opcode = 0b11000_u32 << 27;
            let lfo_bits = encode_lfo(*lfo) << 25;
            Ok(opcode | lfo_bits)
        }

        Instruction::CHO {
            mode,
            lfo,
            flags,
            addr,
        } => {
            let opcode = 0b11001_u32 << 27;
            let mode_bits = encode_cho_mode(*mode) << 24;
            let lfo_bits = encode_lfo(*lfo) << 22;
            let flags_bits = encode_cho_flags(flags) << 16;
            let addr_bits = encode_address(*addr)? & 0xFFFF;
            Ok(opcode | mode_bits | lfo_bits | flags_bits | addr_bits)
        }
    }
}

/// Encode register to 5-bit or 6-bit field depending on register type
fn encode_register(reg: &Register) -> Result<u32, CodegenError> {
    match reg {
        Register::ADCL => Ok(0b00000),
        Register::ADCR => Ok(0b00001),
        Register::DACL => Ok(0b00010),
        Register::DACR => Ok(0b00011),
        Register::ADDR_PTR => Ok(0b00100),
        Register::LR => Ok(0b00101),
        Register::REG(n) if *n < 32 => Ok(*n as u32 + 16), // REG0-31 are offset by 16
        Register::ACC => Ok(0),                            // ACC is implicit in most operations
        _ => Ok(0),                                        // Default for special registers
    }
}

/// Encode S1.14 fixed-point coefficient (-2.0 to ~2.0)
fn encode_s114(value: f32) -> Result<u32, CodegenError> {
    if !value.is_finite() || !(-2.0..2.0).contains(&value) {
        return Err(CodegenError::CoefficientOutOfRange { value });
    }

    // Convert to S1.14: sign bit + 14 fractional bits
    let scaled = (value * 16384.0).round() as i32;
    Ok((scaled & 0x7FFF) as u32)
}

/// Encode S.10 fixed-point coefficient (-1.0 to ~1.0)
fn encode_s10(value: f32) -> Result<u32, CodegenError> {
    if !value.is_finite() || !(-1.0..1.0).contains(&value) {
        return Err(CodegenError::CoefficientOutOfRange { value });
    }

    let scaled = (value * 512.0).round() as i32;
    Ok((scaled & 0x7FF) as u32)
}

/// Encode 16-bit delay address
fn encode_address(addr: u16) -> Result<u32, CodegenError> {
    let max = (DELAY_RAM_SIZE - 1) as u16;
    if addr > max {
        return Err(CodegenError::AddressOutOfRange { addr, max });
    }
    Ok(addr as u32)
}

/// Encode skip condition to 3-bit field
fn encode_skip_condition(condition: SkipCondition) -> u32 {
    match condition {
        SkipCondition::RUN => 0b000,
        SkipCondition::NEG => 0b001,
        SkipCondition::GEZ => 0b010,
        SkipCondition::ZRO => 0b011,
        SkipCondition::ZRC => 0b100,
    }
}

/// Encode LFO to 2-bit field
fn encode_lfo(lfo: Lfo) -> u32 {
    match lfo {
        Lfo::SIN0 => 0b00,
        Lfo::SIN1 => 0b01,
        Lfo::RMP0 => 0b10,
        Lfo::RMP1 => 0b11,
    }
}

/// Encode CHO mode to 2-bit field
fn encode_cho_mode(mode: ChoMode) -> u32 {
    match mode {
        ChoMode::RDA => 0b00,
        ChoMode::SOF => 0b10,
        ChoMode::RDAL => 0b11,
    }
}

/// Encode CHO flags to 6-bit field
fn encode_cho_flags(flags: &ChoFlags) -> u32 {
    let mut bits = 0u32;
    if flags.rptr2 {
        bits |= 0b100000;
    }
    if flags.na {
        bits |= 0b010000;
    }
    if flags.compc {
        bits |= 0b001000;
    }
    if flags.compa {
        bits |= 0b000100;
    }
    if flags.rptr2_select {
        bits |= 0b000010;
    }
    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_rdax() {
        let inst = Instruction::RDAX {
            reg: Register::ADCL,
            coeff: 1.0,
        };
        let encoded = encode_instruction(&inst).unwrap();
        // RDAX opcode is 0b00000, should be in top 5 bits
        assert_eq!(encoded >> 27, 0b00000);
    }

    #[test]
    fn test_encode_wrax() {
        let inst = Instruction::WRAX {
            reg: Register::DACL,
            coeff: 0.5,
        };
        let encoded = encode_instruction(&inst).unwrap();
        // WRAX opcode is 0b00110
        assert_eq!(encoded >> 27, 0b00110);
    }

    #[test]
    fn test_encode_sof() {
        let inst = Instruction::SOF {
            coeff: 1.0,
            offset: 0.0,
        };
        let encoded = encode_instruction(&inst).unwrap();
        // SOF opcode is 0b01101
        assert_eq!(encoded >> 27, 0b01101);
    }

    #[test]
    fn test_encode_clr() {
        let inst = Instruction::CLR;
        let encoded = encode_instruction(&inst).unwrap();
        // CLR opcode is 0b01110
        assert_eq!(encoded >> 27, 0b01110);
    }

    #[test]
    fn test_encode_mulx() {
        let inst = Instruction::MULX {
            reg: Register::REG(0),
        };
        let encoded = encode_instruction(&inst).unwrap();
        // MULX opcode is 0b01010
        assert_eq!(encoded >> 27, 0b01010);
    }

    #[test]
    fn test_encode_s114_positive() {
        let result = encode_s114(1.0).unwrap();
        assert_eq!(result, 16384); // 1.0 * 16384
    }

    #[test]
    fn test_encode_s114_negative() {
        let result = encode_s114(-1.0).unwrap();
        // -1.0 * 16384 = -16384, masked to 15 bits
        assert_eq!(result & 0x7FFF, 0x7FFF - 16384 + 1);
    }

    #[test]
    fn test_encode_s114_out_of_range() {
        let result = encode_s114(3.0);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CodegenError::CoefficientOutOfRange { value: 3.0 }
        ));
    }

    #[test]
    fn test_encode_s10() {
        let result = encode_s10(0.5).unwrap();
        assert_eq!(result, 256); // 0.5 * 512
    }

    #[test]
    fn test_encode_register() {
        assert_eq!(encode_register(&Register::ADCL).unwrap(), 0);
        assert_eq!(encode_register(&Register::ADCR).unwrap(), 1);
        assert_eq!(encode_register(&Register::DACL).unwrap(), 2);
        assert_eq!(encode_register(&Register::DACR).unwrap(), 3);
        assert_eq!(encode_register(&Register::ADDR_PTR).unwrap(), 4);
    }

    #[test]
    fn test_encode_skip_condition() {
        assert_eq!(encode_skip_condition(SkipCondition::RUN), 0b000);
        assert_eq!(encode_skip_condition(SkipCondition::NEG), 0b001);
        assert_eq!(encode_skip_condition(SkipCondition::GEZ), 0b010);
        assert_eq!(encode_skip_condition(SkipCondition::ZRO), 0b011);
        assert_eq!(encode_skip_condition(SkipCondition::ZRC), 0b100);
    }

    #[test]
    fn test_encode_lfo() {
        assert_eq!(encode_lfo(Lfo::SIN0), 0b00);
        assert_eq!(encode_lfo(Lfo::SIN1), 0b01);
        assert_eq!(encode_lfo(Lfo::RMP0), 0b10);
        assert_eq!(encode_lfo(Lfo::RMP1), 0b11);
    }
}
