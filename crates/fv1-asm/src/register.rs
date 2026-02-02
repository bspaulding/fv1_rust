/// FV-1 Registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Register {
    // Accumulator (implied in most operations)
    ACC,
    
    // Audio I/O
    ADCL,  // Left ADC input
    ADCR,  // Right ADC input
    DACL,  // Left DAC output
    DACR,  // Right DAC output
    
    // General purpose registers (32 total)
    REG(u8),  // REG0-REG31
    
    // Special registers
    ADDR_PTR,  // Address pointer for RMPA
    LR,        // Low-pass/Ramp register (some variants)
    
    // Delay RAM address
    SIN0_RATE,
    SIN0_RANGE,
    SIN1_RATE,
    SIN1_RANGE,
    RMP0_RATE,
    RMP0_RANGE,
    RMP1_RATE,
    RMP1_RANGE,
}

/// Control inputs (POT0-POT2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    POT0,
    POT1,
    POT2,
}

/// LFO oscillators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lfo {
    SIN0,
    SIN1,
    RMP0,
    RMP1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_creation() {
        let reg = Register::REG(0);
        assert_eq!(reg, Register::REG(0));
    }

    #[test]
    fn test_control_creation() {
        let pot = Control::POT0;
        assert_eq!(pot, Control::POT0);
    }

    #[test]
    fn test_lfo_creation() {
        let lfo = Lfo::SIN0;
        assert_eq!(lfo, Lfo::SIN0);
    }
}
