//! FV-1 Example Programs
//!
//! This crate contains example FV-1 programs demonstrating various effects and techniques.

use fv1_asm::{Instruction, Register};

/// Simple pass-through example
pub fn passthrough() -> Vec<Instruction> {
    vec![
        Instruction::RDAX {
            reg: Register::ADCL,
            coeff: 1.0,
        },
        Instruction::WRAX {
            reg: Register::DACL,
            coeff: 0.0,
        },
    ]
}

/// Simple gain control example
pub fn gain_control() -> Vec<Instruction> {
    vec![
        Instruction::RDAX {
            reg: Register::ADCL,
            coeff: 1.0,
        },
        Instruction::MULX {
            reg: Register::REG(0), // Assume POT0 is loaded into REG0
        },
        Instruction::WRAX {
            reg: Register::DACL,
            coeff: 0.0,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passthrough() {
        let program = passthrough();
        assert_eq!(program.len(), 2);
    }

    #[test]
    fn test_gain_control() {
        let program = gain_control();
        assert_eq!(program.len(), 3);
    }
}
