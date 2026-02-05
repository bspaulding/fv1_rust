//! FV-1 Example Programs
//!
//! This crate contains example FV-1 programs demonstrating various effects and techniques.
//!
//! The examples are organized into two categories:
//! - Low-level instruction examples (using `fv1_asm` directly)
//! - DSL examples (using the `fv1_dsl` high-level API)

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

/// DSL examples demonstrating high-level programming with the FV-1 DSL
pub mod dsl_examples {
    use fv1_dsl::prelude::*;

    /// Simple pass-through example using the fv1_program! macro
    ///
    /// This is the simplest FV-1 program - it reads the left ADC input
    /// and writes it directly to the left DAC output.
    ///
    /// # Example
    /// ```
    /// use fv1_examples::dsl_examples;
    /// use fv1_asm::Assembler;
    ///
    /// let program = dsl_examples::passthrough_macro();
    /// let assembler = Assembler::new();
    /// let binary = assembler.assemble(&program).unwrap();
    /// assert!(!binary.to_bytes().is_empty());
    /// ```
    pub fn passthrough_macro() -> fv1_asm::Program {
        fv1_program! {
            rdax(Register::ADCL, 1.0);
            wrax(Register::DACL, 0.0);
        }
    }

    /// Simple pass-through example using the untyped ProgramBuilder
    ///
    /// This demonstrates the fluent builder API for constructing programs.
    pub fn passthrough_builder() -> fv1_asm::Program {
        ProgramBuilder::new()
            .inst(rdax(Register::ADCL, 1.0))
            .inst(wrax(Register::DACL, 0.0))
            .build()
    }

    /// Simple pass-through example using the type-safe TypedBuilder
    ///
    /// This demonstrates compile-time type safety with phantom types.
    pub fn passthrough_typed() -> fv1_asm::Program {
        TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .wrax(Register::DACL, 0.0)
            .build()
    }

    /// Gain control example using POT0
    ///
    /// This program reads the left ADC input, multiplies it by POT0 for volume
    /// control, and outputs to the left DAC.
    ///
    /// Note: POT0 maps to REG16 in the FV-1 architecture.
    ///
    /// # Example
    /// ```
    /// use fv1_examples::dsl_examples;
    /// use fv1_asm::Assembler;
    ///
    /// let program = dsl_examples::gain_control();
    /// let assembler = Assembler::new();
    /// let binary = assembler.assemble(&program).unwrap();
    /// assert_eq!(program.instructions().len(), 3);
    /// ```
    pub fn gain_control() -> fv1_asm::Program {
        fv1_program! {
            rdax(Register::ADCL, 1.0);
            mulx(Register::REG(16)); // POT0
            wrax(Register::DACL, 0.0);
        }
    }

    /// Gain control using the type-safe TypedBuilder
    pub fn gain_control_typed() -> fv1_asm::Program {
        TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .mulx(Register::REG(16)) // POT0
            .wrax(Register::DACL, 0.0)
            .build()
    }

    /// Delay echo effect with feedback and mix controls
    ///
    /// This program creates a simple delay/echo effect:
    /// - POT1 (REG17) controls feedback amount
    /// - POT2 (REG18) controls wet/dry mix
    /// - Fixed delay time at address 4000
    ///
    /// # Example
    /// ```
    /// use fv1_examples::dsl_examples;
    /// use fv1_asm::Assembler;
    ///
    /// let program = dsl_examples::delay_echo();
    /// let assembler = Assembler::new();
    /// let binary = assembler.assemble(&program).unwrap();
    /// assert_eq!(program.instructions().len(), 9);
    /// ```
    pub fn delay_echo() -> fv1_asm::Program {
        TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .wrax(Register::REG(0), 0.0) // Save input
            .rda(4000, 0.5) // Read delayed signal
            .mulx(Register::REG(17)) // POT1 - feedback
            .rdax(Register::REG(0), 1.0) // Add input
            .wra(0, 0.0) // Write to delay line
            .mulx(Register::REG(18)) // POT2 - wet amount
            .rdax(Register::REG(0), 1.0) // Add dry signal
            .wrax(Register::DACL, 0.0) // Output
            .build()
    }

    /// Advanced effect using high-level building blocks
    ///
    /// This demonstrates combining multiple DSP blocks:
    /// - Input gain control with POT0
    /// - One-pole lowpass filter with POT1 controlling cutoff
    /// - Soft clipping for saturation
    /// - Output
    ///
    /// # Example
    /// ```
    /// use fv1_examples::dsl_examples;
    /// use fv1_asm::Assembler;
    ///
    /// let program = dsl_examples::advanced_effect();
    /// let assembler = Assembler::new();
    /// let binary = assembler.assemble(&program).unwrap();
    /// assert!(!binary.to_bytes().is_empty());
    /// ```
    pub fn advanced_effect() -> fv1_asm::Program {
        let mut builder = ProgramBuilder::new();

        // Input gain control
        builder.add_inst(blocks::gain(Register::ADCL, Register::REG(16)));
        builder.add_inst(mulx(Register::REG(16))); // POT0 controls input gain

        // One-pole lowpass filter
        // Filter state stored in REG1, cutoff controlled by POT1 (REG17)
        for inst in blocks::lowpass(Register::ACC, Register::REG(17), Register::REG(1)) {
            builder.add_inst(inst);
        }

        // Soft clipping for saturation (threshold at 0.9)
        for inst in blocks::soft_clip(0.9) {
            builder.add_inst(inst);
        }

        // Output
        builder.add_inst(wrax(Register::DACL, 0.0));

        builder.build()
    }

    /// Multi-tap delay effect using Delay abstraction
    ///
    /// This creates a multi-tap delay with two tap points:
    /// - Tap 1 at 2000 samples
    /// - Tap 2 at 4000 samples
    /// - POT0 controls overall feedback
    /// - POT1 controls wet/dry mix
    ///
    /// # Example
    /// ```
    /// use fv1_examples::dsl_examples;
    /// use fv1_asm::Assembler;
    ///
    /// let program = dsl_examples::multi_tap_delay();
    /// let assembler = Assembler::new();
    /// let binary = assembler.assemble(&program).unwrap();
    /// assert!(!binary.to_bytes().is_empty());
    /// ```
    pub fn multi_tap_delay() -> fv1_asm::Program {
        let delay = blocks::Delay::new(0, 8000);

        let mut builder = ProgramBuilder::new();

        // Read and save input
        builder.add_inst(rdax(Register::ADCL, 1.0));
        builder.add_inst(wrax(Register::REG(0), 0.0));

        // Clear accumulator
        builder.add_inst(clr());

        // Add first tap at 2000 samples (coefficient 0.4)
        builder.add_inst(rda(2000, 0.4));

        // Add second tap at 4000 samples (coefficient 0.3)
        builder.add_inst(rda(4000, 0.3));

        // Scale by POT1 for wet amount
        builder.add_inst(mulx(Register::REG(17))); // POT1

        // Add dry signal
        builder.add_inst(rdax(Register::REG(0), 1.0));

        // Store output in temp register
        builder.add_inst(wrax(Register::REG(1), 1.0));

        // Write to delay line with feedback (POT0)
        builder.add_inst(rdax(Register::REG(0), 1.0));
        builder.add_inst(mulx(Register::REG(16))); // POT0 - feedback
        builder.add_inst(rdax(Register::REG(1), 1.0));
        for inst in delay.write(0.0) {
            builder.add_inst(inst);
        }

        // Output final mix
        builder.add_inst(rdax(Register::REG(1), 1.0));
        builder.add_inst(wrax(Register::DACL, 0.0));

        builder.build()
    }
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

    // DSL examples tests
    #[test]
    fn test_dsl_passthrough_macro() {
        let program = dsl_examples::passthrough_macro();
        assert_eq!(program.instructions().len(), 2);
    }

    #[test]
    fn test_dsl_passthrough_builder() {
        let program = dsl_examples::passthrough_builder();
        assert_eq!(program.instructions().len(), 2);
    }

    #[test]
    fn test_dsl_passthrough_typed() {
        let program = dsl_examples::passthrough_typed();
        assert_eq!(program.instructions().len(), 2);
    }

    #[test]
    fn test_dsl_gain_control() {
        let program = dsl_examples::gain_control();
        assert_eq!(program.instructions().len(), 3);
    }

    #[test]
    fn test_dsl_gain_control_typed() {
        let program = dsl_examples::gain_control_typed();
        assert_eq!(program.instructions().len(), 3);
    }

    #[test]
    fn test_dsl_delay_echo() {
        let program = dsl_examples::delay_echo();
        assert_eq!(program.instructions().len(), 9);
    }

    #[test]
    fn test_dsl_advanced_effect() {
        let program = dsl_examples::advanced_effect();
        // 2 (gain) + 4 (lowpass) + 3 (soft_clip) + 1 (output) = 10
        assert_eq!(program.instructions().len(), 10);
    }

    #[test]
    fn test_dsl_multi_tap_delay() {
        let program = dsl_examples::multi_tap_delay();
        // Verify it has instructions and can be assembled
        assert!(program.instructions().len() > 10);
    }

    #[test]
    fn test_dsl_examples_can_be_assembled() {
        use fv1_asm::Assembler;

        let examples = vec![
            dsl_examples::passthrough_macro(),
            dsl_examples::passthrough_builder(),
            dsl_examples::passthrough_typed(),
            dsl_examples::gain_control(),
            dsl_examples::gain_control_typed(),
            dsl_examples::delay_echo(),
            dsl_examples::advanced_effect(),
            dsl_examples::multi_tap_delay(),
        ];

        let assembler = Assembler::new();
        for program in examples {
            let result = assembler.assemble(&program);
            assert!(
                result.is_ok(),
                "Failed to assemble program: {:?}",
                result.err()
            );
        }
    }
}
