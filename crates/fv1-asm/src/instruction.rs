use crate::register::{Register, Lfo};

/// FV-1 Instruction Set
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Accumulator operations
    /// Read register and add to ACC: ACC = ACC * C + [REG] * D
    RDAX { reg: Register, coeff: f32 },
    
    /// Read delay RAM: ACC = ACC * C + [ADDR] * D
    RDA { addr: u16, coeff: f32 },
    
    /// Read delay RAM with LFO: ACC = ACC * C + [ADDR + LFO] * D
    RMPA { coeff: f32 },
    
    /// Write ACC to register: [REG] = ACC * C, ACC = ACC * D
    WRAX { reg: Register, coeff: f32 },
    
    /// Write ACC to delay RAM: [ADDR] = ACC * C, ACC = ACC * D
    WRA { addr: u16, coeff: f32 },
    
    /// Write ACC with crossfade: [ADDR] = ACC * C + [ADDR] * D
    WRAP { addr: u16, coeff: f32 },
    
    // Mathematical operations
    /// Multiply ACC by register: ACC = ACC * [REG]
    MULX { reg: Register },
    
    /// Reverse multiply: ACC = [REG] - ACC * [REG]
    RDFX { reg: Register, coeff: f32 },
    
    /// Absolute value: ACC = |ACC| * C
    ABSA,
    
    /// Load immediate: ACC = C
    LDAX { reg: Register },
    
    // Filtering
    /// Single-pole lowpass: ACC = C * ACC + (1-C) * [REG]
    RDFX2 { reg: Register, coeff: f32 },
    
    // Logic and control
    /// Set accumulator to S
    SOF { coeff: f32, offset: f32 },  // ACC = ACC * C + D
    
    /// AND with mask
    AND { mask: u32 },
    
    /// OR with mask
    OR { mask: u32 },
    
    /// XOR with mask
    XOR { mask: u32 },
    
    /// Shift left
    SHL,
    
    /// Shift right
    SHR,
    
    /// Clear ACC
    CLR,
    
    /// No operation
    NOP,
    
    /// Exponential conversion
    EXP { coeff: f32, offset: f32 },
    
    /// Logarithmic conversion
    LOG { coeff: f32, offset: f32 },
    
    // Conditional skipping
    /// Skip next instruction if condition is met
    SKP { condition: SkipCondition, offset: i8 },
    
    // LFO control
    /// Write LFO frequency
    WLDS { lfo: Lfo, freq: u16, amplitude: u16 },
    
    // Jump/Call (if supported in variant)
    JAM { lfo: Lfo },
    
    // Delay RAM addressing
    CHO { mode: ChoMode, lfo: Lfo, flags: ChoFlags, addr: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipCondition {
    GEZ,  // Greater or equal to zero
    NEG,  // Negative
    ZRC,  // Zero crossing
    ZRO,  // Zero
    RUN,  // Always run
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoMode {
    RDA,  // Read delay with LFO
    SOF,  // Scale/offset with LFO
    RDAL, // Read delay and load LFO value
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChoFlags {
    pub rptr2: bool,      // Use second read pointer
    pub na: bool,         // No add (crossfade control)
    pub compc: bool,      // Complement coefficient
    pub compa: bool,      // Complement address
    pub rptr2_select: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::Register;

    #[test]
    fn test_rdax_instruction() {
        let inst = Instruction::RDAX {
            reg: Register::ADCL,
            coeff: 1.0,
        };
        match inst {
            Instruction::RDAX { reg, coeff } => {
                assert_eq!(reg, Register::ADCL);
                assert_eq!(coeff, 1.0);
            }
            _ => panic!("Wrong instruction type"),
        }
    }

    #[test]
    fn test_sof_instruction() {
        let inst = Instruction::SOF {
            coeff: 0.5,
            offset: 0.0,
        };
        match inst {
            Instruction::SOF { coeff, offset } => {
                assert_eq!(coeff, 0.5);
                assert_eq!(offset, 0.0);
            }
            _ => panic!("Wrong instruction type"),
        }
    }

    #[test]
    fn test_skip_condition() {
        let cond = SkipCondition::GEZ;
        assert_eq!(cond, SkipCondition::GEZ);
    }
}
