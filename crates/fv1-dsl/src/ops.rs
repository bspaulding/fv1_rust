/// Helper functions for creating FV-1 instructions
///
/// This module provides convenience functions for creating instructions
/// with a more ergonomic API than constructing the enums directly.

use crate::{Instruction, Register, Lfo, SkipCondition, ChoMode, ChoFlags};

// Accumulator operations

/// Read register and add to ACC: ACC = ACC * C + [REG] * D
pub fn rdax(reg: Register, coeff: f32) -> Instruction {
    Instruction::RDAX { reg, coeff }
}

/// Read delay RAM: ACC = ACC * C + [ADDR] * D
pub fn rda(addr: u16, coeff: f32) -> Instruction {
    Instruction::RDA { addr, coeff }
}

/// Read delay RAM with LFO: ACC = ACC * C + [ADDR + LFO] * D
pub fn rmpa(coeff: f32) -> Instruction {
    Instruction::RMPA { coeff }
}

/// Write ACC to register: [REG] = ACC * C, ACC = ACC * D
pub fn wrax(reg: Register, coeff: f32) -> Instruction {
    Instruction::WRAX { reg, coeff }
}

/// Write ACC to delay RAM: [ADDR] = ACC * C, ACC = ACC * D
pub fn wra(addr: u16, coeff: f32) -> Instruction {
    Instruction::WRA { addr, coeff }
}

/// Write ACC with crossfade: [ADDR] = ACC * C + [ADDR] * D
pub fn wrap(addr: u16, coeff: f32) -> Instruction {
    Instruction::WRAP { addr, coeff }
}

// Mathematical operations

/// Multiply ACC by register: ACC = ACC * [REG]
pub fn mulx(reg: Register) -> Instruction {
    Instruction::MULX { reg }
}

/// Reverse multiply: ACC = [REG] - ACC * [REG]
pub fn rdfx(reg: Register, coeff: f32) -> Instruction {
    Instruction::RDFX { reg, coeff }
}

/// Absolute value: ACC = |ACC| * C
pub fn absa() -> Instruction {
    Instruction::ABSA
}

/// Load immediate: ACC = C
pub fn ldax(reg: Register) -> Instruction {
    Instruction::LDAX { reg }
}

// Filtering

/// RDFX with double filtering: ACC = C * ACC + (1-C) * [REG]
pub fn rdfx2(reg: Register, coeff: f32) -> Instruction {
    Instruction::RDFX2 { reg, coeff }
}

// Logic and control

/// Set accumulator: ACC = ACC * C + D
pub fn sof(coeff: f32, offset: f32) -> Instruction {
    Instruction::SOF { coeff, offset }
}

/// AND with mask
pub fn and(mask: u32) -> Instruction {
    Instruction::AND { mask }
}

/// OR with mask
pub fn or(mask: u32) -> Instruction {
    Instruction::OR { mask }
}

/// XOR with mask
pub fn xor(mask: u32) -> Instruction {
    Instruction::XOR { mask }
}

/// Shift left
pub fn shl() -> Instruction {
    Instruction::SHL
}

/// Shift right
pub fn shr() -> Instruction {
    Instruction::SHR
}

/// Clear ACC
pub fn clr() -> Instruction {
    Instruction::CLR
}

/// No operation
pub fn nop() -> Instruction {
    Instruction::NOP
}

/// Exponential conversion
pub fn exp(coeff: f32, offset: f32) -> Instruction {
    Instruction::EXP { coeff, offset }
}

/// Logarithmic conversion
pub fn log(coeff: f32, offset: f32) -> Instruction {
    Instruction::LOG { coeff, offset }
}

// Conditional skipping

/// Skip next instruction if condition is met
pub fn skp(condition: SkipCondition, offset: i8) -> Instruction {
    Instruction::SKP { condition, offset }
}

// LFO control

/// Write LFO frequency
pub fn wlds(lfo: Lfo, freq: u16, amplitude: u16) -> Instruction {
    Instruction::WLDS { lfo, freq, amplitude }
}

/// JAM LFO
pub fn jam(lfo: Lfo) -> Instruction {
    Instruction::JAM { lfo }
}

// Delay RAM addressing

/// CHO - Complex LFO operation
pub fn cho(mode: ChoMode, lfo: Lfo, flags: ChoFlags, addr: u16) -> Instruction {
    Instruction::CHO { mode, lfo, flags, addr }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rdax() {
        let inst = rdax(Register::ADCL, 1.0);
        match inst {
            Instruction::RDAX { reg, coeff } => {
                assert_eq!(reg, Register::ADCL);
                assert_eq!(coeff, 1.0);
            }
            _ => panic!("Wrong instruction type"),
        }
    }
    
    #[test]
    fn test_wrax() {
        let inst = wrax(Register::DACL, 0.5);
        match inst {
            Instruction::WRAX { reg, coeff } => {
                assert_eq!(reg, Register::DACL);
                assert_eq!(coeff, 0.5);
            }
            _ => panic!("Wrong instruction type"),
        }
    }
    
    #[test]
    fn test_sof() {
        let inst = sof(0.5, 0.25);
        match inst {
            Instruction::SOF { coeff, offset } => {
                assert_eq!(coeff, 0.5);
                assert_eq!(offset, 0.25);
            }
            _ => panic!("Wrong instruction type"),
        }
    }
    
    #[test]
    fn test_mulx() {
        let inst = mulx(Register::REG(0));
        match inst {
            Instruction::MULX { reg } => {
                assert_eq!(reg, Register::REG(0));
            }
            _ => panic!("Wrong instruction type"),
        }
    }
    
    #[test]
    fn test_clr() {
        let inst = clr();
        assert_eq!(inst, Instruction::CLR);
    }
    
    #[test]
    fn test_nop() {
        let inst = nop();
        assert_eq!(inst, Instruction::NOP);
    }
}
