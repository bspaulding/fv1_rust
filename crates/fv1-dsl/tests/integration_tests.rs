use fv1_dsl::prelude::*;

/// Tests for the type-safe TypedBuilder
mod typed_builder_tests {
    use super::*;

    #[test]
    fn test_typed_builder_basic_program() {
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 2);
    }

    #[test]
    fn test_typed_builder_gain_control() {
        // Same gain control as before, but with type safety
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0) // Transitions to Audio state
            .mulx(Register::REG(16)) // POT0, stays in Audio state
            .wrax(Register::DACL, 0.0) // Stays in Audio state
            .build();

        assert_eq!(program.instructions().len(), 3);

        // Verify the instructions are correct
        let instructions = program.instructions();
        match instructions[0] {
            fv1_asm::Instruction::RDAX { reg, coeff } => {
                assert_eq!(*reg, Register::ADCL);
                assert_eq!(*coeff, 1.0);
            }
            _ => panic!("Expected RDAX instruction"),
        }
    }

    #[test]
    fn test_typed_builder_clr_start() {
        // Start with CLR which transitions to Audio state
        let program = TypedBuilder::new()
            .clr() // Transitions to Audio state
            .rdax(Register::ADCL, 1.0) // Stays in Audio state
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 3);
    }

    #[test]
    fn test_typed_builder_delay_echo() {
        // Delay echo using TypedBuilder
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .wrax(Register::REG(0), 0.0)
            .rda(4000, 0.5)
            .mulx(Register::REG(17)) // POT1
            .rdax(Register::REG(0), 1.0)
            .wra(0, 0.0)
            .mulx(Register::REG(18)) // POT2
            .rdax(Register::REG(0), 1.0)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 9);
    }

    #[test]
    fn test_typed_builder_can_be_assembled() {
        use fv1_asm::Assembler;

        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .sof(0.5, 0.0)
            .wrax(Register::DACL, 0.0)
            .build();

        // Verify we can assemble it
        let assembler = Assembler::new();
        let result = assembler.assemble(&program);

        assert!(result.is_ok());
        let binary = result.unwrap();

        // Binary should contain our 3 instructions
        let bytes = binary.to_bytes();
        assert!(
            bytes.len() >= 12,
            "Binary should contain at least 12 bytes for 3 instructions"
        );
    }

    #[test]
    fn test_typed_builder_complex_effects_chain() {
        // Complex audio processing chain demonstrating type safety
        let program = TypedBuilder::new()
            .clr() // Start clean
            .rdax(Register::ADCL, 1.0) // Read input
            .sof(0.9, 0.0) // Scale down slightly
            .wrax(Register::REG(0), 0.5) // Store and keep half in ACC
            .rda(8000, 0.6) // Read delayed signal
            .mulx(Register::REG(16)) // Modulate with POT0
            .rdax(Register::REG(0), 1.0) // Add dry signal
            .sof(0.8, 0.0) // Scale output
            .wrax(Register::DACL, 0.0) // Output
            .build();

        assert_eq!(program.instructions().len(), 9);

        // Verify it can be assembled
        use fv1_asm::Assembler;
        let assembler = Assembler::new();
        let result = assembler.assemble(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_typed_builder_bitwise_operations() {
        // Test bitwise operations in the type-safe builder
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .and(0xFFFF0000)
            .or(0x0000FFFF)
            .xor(0x0000FF00)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 5);
    }

    #[test]
    fn test_typed_builder_math_operations() {
        // Test mathematical operations
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .absa() // Absolute value
            .exp(1.0, 0.0) // Exponential
            .log(1.0, 0.0) // Logarithm
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 5);
    }

    #[test]
    fn test_typed_builder_nop_insertion() {
        // Test NOP instructions can be inserted anywhere
        let program = TypedBuilder::new()
            .nop()
            .rdax(Register::ADCL, 1.0)
            .nop()
            .nop()
            .wrax(Register::DACL, 0.0)
            .nop()
            .build();

        assert_eq!(program.instructions().len(), 6);
    }
}

#[test]
fn test_macro_basic_program() {
    let program = fv1_program! {
        rdax(Register::ADCL, 1.0);
        wrax(Register::DACL, 0.0);
    };

    assert_eq!(program.instructions().len(), 2);
}

#[test]
fn test_macro_with_multiple_instructions() {
    let program = fv1_program! {
        clr();
        rdax(Register::ADCL, 1.0);
        sof(0.5, 0.0);
        wrax(Register::DACL, 0.0);
    };

    assert_eq!(program.instructions().len(), 4);
}

#[test]
fn test_macro_gain_control() {
    // Simple gain control using POT0
    let program = fv1_program! {
        rdax(Register::ADCL, 1.0);
        mulx(Register::REG(0));
        wrax(Register::DACL, 0.0);
    };

    assert_eq!(program.instructions().len(), 3);

    // Verify the instructions are correct
    let instructions = program.instructions();
    match instructions[0] {
        fv1_asm::Instruction::RDAX { reg, coeff } => {
            assert_eq!(*reg, Register::ADCL);
            assert_eq!(*coeff, 1.0);
        }
        _ => panic!("Expected RDAX instruction"),
    }

    match instructions[1] {
        fv1_asm::Instruction::MULX { reg } => {
            assert_eq!(*reg, Register::REG(0));
        }
        _ => panic!("Expected MULX instruction"),
    }
}

#[test]
fn test_builder_api_direct() {
    // Test the builder API without the macro
    let program = ProgramBuilder::new()
        .inst(rdax(Register::ADCL, 1.0))
        .inst(sof(0.5, 0.0))
        .inst(wrax(Register::DACL, 0.0))
        .build();

    assert_eq!(program.instructions().len(), 3);
}

#[test]
fn test_ops_module() {
    // Test various ops functions
    let inst1 = rdax(Register::ADCL, 1.0);
    let inst2 = wrax(Register::DACL, 0.5);
    let _inst3 = sof(0.5, 0.25);
    let _inst4 = mulx(Register::REG(0));
    let inst5 = clr();
    let inst6 = nop();

    match inst1 {
        fv1_asm::Instruction::RDAX { reg, coeff } => {
            assert_eq!(reg, Register::ADCL);
            assert_eq!(coeff, 1.0);
        }
        _ => panic!("Wrong instruction type"),
    }

    match inst2 {
        fv1_asm::Instruction::WRAX { reg, coeff } => {
            assert_eq!(reg, Register::DACL);
            assert_eq!(coeff, 0.5);
        }
        _ => panic!("Wrong instruction type"),
    }

    assert_eq!(inst5, fv1_asm::Instruction::CLR);
    assert_eq!(inst6, fv1_asm::Instruction::NOP);
}

#[test]
fn test_program_can_be_assembled() {
    use fv1_asm::Assembler;

    let program = fv1_program! {
        rdax(Register::ADCL, 1.0);
        sof(0.5, 0.0);
        wrax(Register::DACL, 0.0);
    };

    // Verify we can assemble it
    let assembler = Assembler::new();
    let result = assembler.assemble(&program);

    assert!(result.is_ok());
    let binary = result.unwrap();

    // Binary should contain our 3 instructions
    // (The assembler may pad to full program size of 512 bytes)
    let bytes = binary.to_bytes();
    assert!(
        bytes.len() >= 12,
        "Binary should contain at least 12 bytes for 3 instructions"
    );
}

/// Tests that compare DSL-built programs with parsed assembly examples
mod example_equivalence_tests {
    use super::*;
    use fv1_asm::{Assembler, Parser};

    /// Helper function to parse an assembly file and assemble it
    fn assemble_from_file(asm_source: &str) -> Vec<u8> {
        let mut parser = Parser::new(asm_source);
        let program = parser.parse().expect("Failed to parse assembly");
        let assembler = Assembler::new();
        let binary = assembler.assemble(&program).expect("Failed to assemble");
        binary.to_bytes()
    }

    /// Helper function to assemble a DSL-built program
    fn assemble_from_dsl(program: fv1_asm::Program) -> Vec<u8> {
        let assembler = Assembler::new();
        let binary = assembler
            .assemble(&program)
            .expect("Failed to assemble DSL program");
        binary.to_bytes()
    }

    #[test]
    fn test_passthrough_example_equivalence() {
        // Assembly source
        let asm_source = r#"
; Simple pass-through program
; Copies left input directly to left output

; Read left ADC input with unity gain
RDAX ADCL, 1.0

; Write to left DAC output
WRAX DACL, 0.0
"#;

        // DSL version using macro
        let dsl_program_macro = fv1_program! {
            rdax(Register::ADCL, 1.0);
            wrax(Register::DACL, 0.0);
        };

        // DSL version using builder
        let dsl_program_builder = ProgramBuilder::new()
            .inst(rdax(Register::ADCL, 1.0))
            .inst(wrax(Register::DACL, 0.0))
            .build();

        // Assemble all versions
        let asm_binary = assemble_from_file(asm_source);
        let dsl_macro_binary = assemble_from_dsl(dsl_program_macro);
        let dsl_builder_binary = assemble_from_dsl(dsl_program_builder);

        // All should produce identical binaries
        assert_eq!(
            asm_binary, dsl_macro_binary,
            "Assembly and DSL macro versions should produce identical binaries"
        );
        assert_eq!(
            asm_binary, dsl_builder_binary,
            "Assembly and DSL builder versions should produce identical binaries"
        );
    }

    #[test]
    fn test_gain_control_example_equivalence() {
        // Assembly source (POT0 maps to REG16)
        let asm_source = r#"
; Gain control using POT0
; POT0 controls the volume from 0 to 100%

; Read left ADC input
RDAX ADCL, 1.0

; Multiply by POT0 for volume control
MULX POT0

; Write to left DAC output
WRAX DACL, 0.0
"#;

        // DSL version using macro (POT0 = REG16)
        let dsl_program_macro = fv1_program! {
            rdax(Register::ADCL, 1.0);
            mulx(Register::REG(16));
            wrax(Register::DACL, 0.0);
        };

        // DSL version using builder
        let dsl_program_builder = ProgramBuilder::new()
            .inst(rdax(Register::ADCL, 1.0))
            .inst(mulx(Register::REG(16)))
            .inst(wrax(Register::DACL, 0.0))
            .build();

        // Assemble all versions
        let asm_binary = assemble_from_file(asm_source);
        let dsl_macro_binary = assemble_from_dsl(dsl_program_macro);
        let dsl_builder_binary = assemble_from_dsl(dsl_program_builder);

        // All should produce identical binaries
        assert_eq!(
            asm_binary, dsl_macro_binary,
            "Assembly and DSL macro versions should produce identical binaries for gain_control"
        );
        assert_eq!(
            asm_binary, dsl_builder_binary,
            "Assembly and DSL builder versions should produce identical binaries for gain_control"
        );
    }

    #[test]
    fn test_delay_echo_example_equivalence() {
        // Assembly source (POT1 = REG17, POT2 = REG18)
        let asm_source = r#"
; Simple delay/echo effect
; A basic echo with fixed delay time
; POT1 controls feedback amount
; POT2 controls wet/dry mix

; Read input
RDAX ADCL, 1.0
WRAX REG0, 0.0          ; Save input to REG0

; Read from delay line at address 4000
RDA 4000, 0.5           ; Read delayed signal

; Add feedback
MULX POT1               ; Scale by feedback amount
RDAX REG0, 1.0          ; Add input
WRA 0, 0.0              ; Write to delay line at address 0

; Mix wet/dry
; ACC now has delayed signal
MULX POT2               ; Scale by wet amount
RDAX REG0, 1.0          ; Add dry signal

; Output
WRAX DACL, 0.0
"#;

        // DSL version using macro (POT1 = REG17, POT2 = REG18)
        let dsl_program_macro = fv1_program! {
            rdax(Register::ADCL, 1.0);
            wrax(Register::REG(0), 0.0);
            rda(4000, 0.5);
            mulx(Register::REG(17));
            rdax(Register::REG(0), 1.0);
            wra(0, 0.0);
            mulx(Register::REG(18));
            rdax(Register::REG(0), 1.0);
            wrax(Register::DACL, 0.0);
        };

        // DSL version using builder
        let dsl_program_builder = ProgramBuilder::new()
            .inst(rdax(Register::ADCL, 1.0))
            .inst(wrax(Register::REG(0), 0.0))
            .inst(rda(4000, 0.5))
            .inst(mulx(Register::REG(17)))
            .inst(rdax(Register::REG(0), 1.0))
            .inst(wra(0, 0.0))
            .inst(mulx(Register::REG(18)))
            .inst(rdax(Register::REG(0), 1.0))
            .inst(wrax(Register::DACL, 0.0))
            .build();

        // Assemble all versions
        let asm_binary = assemble_from_file(asm_source);
        let dsl_macro_binary = assemble_from_dsl(dsl_program_macro);
        let dsl_builder_binary = assemble_from_dsl(dsl_program_builder);

        // All should produce identical binaries
        assert_eq!(
            asm_binary, dsl_macro_binary,
            "Assembly and DSL macro versions should produce identical binaries for delay_echo"
        );
        assert_eq!(
            asm_binary, dsl_builder_binary,
            "Assembly and DSL builder versions should produce identical binaries for delay_echo"
        );
    }
}

/// Tests demonstrating high-level abstractions from blocks module
mod high_level_abstraction_tests {
    use super::*;
    use fv1_asm::{Assembler, Parser};

    /// Helper function to parse an assembly file and assemble it
    fn assemble_from_file(asm_source: &str) -> Vec<u8> {
        let mut parser = Parser::new(asm_source);
        let program = parser.parse().expect("Failed to parse assembly");
        let assembler = Assembler::new();
        let binary = assembler.assemble(&program).expect("Failed to assemble");
        binary.to_bytes()
    }

    /// Helper function to assemble a DSL-built program
    fn assemble_from_dsl(program: fv1_asm::Program) -> Vec<u8> {
        let assembler = Assembler::new();
        let binary = assembler
            .assemble(&program)
            .expect("Failed to assemble DSL program");
        binary.to_bytes()
    }

    #[test]
    fn test_passthrough_with_blocks() {
        // Original assembly
        let asm_source = r#"
RDAX ADCL, 1.0
WRAX DACL, 0.0
"#;

        // Using high-level blocks - passthrough is just read + write
        // The blocks::gain function reads the input
        let mut builder = ProgramBuilder::new();
        builder.add_inst(blocks::gain(Register::ADCL, Register::REG(16)));
        builder.add_inst(wrax(Register::DACL, 0.0));
        let dsl_program = builder.build();

        let asm_binary = assemble_from_file(asm_source);
        let dsl_binary = assemble_from_dsl(dsl_program);

        // Should produce identical binaries (gain without mulx is just rdax)
        assert_eq!(
            asm_binary, dsl_binary,
            "Passthrough using blocks should produce identical binary"
        );
    }

    #[test]
    fn test_gain_control_with_blocks() {
        // Original assembly (POT0 = REG16)
        let asm_source = r#"
RDAX ADCL, 1.0
MULX POT0
WRAX DACL, 0.0
"#;

        // Using high-level blocks - gain reads input, then we multiply
        let mut builder = ProgramBuilder::new();
        builder.add_inst(blocks::gain(Register::ADCL, Register::REG(16)));
        builder.add_inst(mulx(Register::REG(16))); // POT0
        builder.add_inst(wrax(Register::DACL, 0.0));
        let dsl_program = builder.build();

        let asm_binary = assemble_from_file(asm_source);
        let dsl_binary = assemble_from_dsl(dsl_program);

        assert_eq!(
            asm_binary, dsl_binary,
            "Gain control using blocks should produce identical binary"
        );
    }

    #[test]
    fn test_delay_echo_with_blocks() {
        // Original assembly (POT1 = REG17, POT2 = REG18)
        let asm_source = r#"
RDAX ADCL, 1.0
WRAX REG0, 0.0
RDA 4000, 0.5
MULX POT1
RDAX REG0, 1.0
WRA 0, 0.0
MULX POT2
RDAX REG0, 1.0
WRAX DACL, 0.0
"#;

        // Using high-level blocks with Delay abstraction
        let delay = blocks::Delay::new(0, 4000);

        let mut builder = ProgramBuilder::new();
        // Read input and save
        builder.add_inst(rdax(Register::ADCL, 1.0));
        builder.add_inst(wrax(Register::REG(0), 0.0));

        // Read delayed signal using Delay block
        for inst in delay.read(4000) {
            builder.add_inst(inst);
        }

        // Note: We need to use specific coefficient for RDA
        // The Delay::read uses coefficient 1.0, but original uses 0.5
        // So we need to scale it
        builder.add_inst(sof(0.5, 0.0)); // Scale to match original 0.5 coefficient

        // Add feedback
        builder.add_inst(mulx(Register::REG(17))); // POT1
        builder.add_inst(rdax(Register::REG(0), 1.0));

        // Write to delay line using Delay block
        for inst in delay.write(0.0) {
            builder.add_inst(inst);
        }

        // Mix wet/dry
        builder.add_inst(mulx(Register::REG(18))); // POT2
        builder.add_inst(rdax(Register::REG(0), 1.0));

        // Output
        builder.add_inst(wrax(Register::DACL, 0.0));

        let dsl_program = builder.build();

        let _asm_binary = assemble_from_file(asm_source);
        
        // The instruction count should be different (we have one extra SOF)
        assert_eq!(
            dsl_program.instructions().len(),
            10,
            "Block-based version has 10 instructions (1 extra SOF)"
        );
        
        let dsl_binary = assemble_from_dsl(dsl_program);

        // Note: This won't be identical because we added an extra SOF instruction
        // But let's verify it assembles correctly
        assert!(
            !dsl_binary.is_empty(),
            "Delay echo using blocks should assemble successfully"
        );
    }

    #[test]
    fn test_delay_echo_with_blocks_exact_equivalence() {
        // Let's create a version that matches exactly by using RDA directly with coefficient
        let asm_source = r#"
RDAX ADCL, 1.0
WRAX REG0, 0.0
RDA 4000, 0.5
MULX POT1
RDAX REG0, 1.0
WRA 0, 0.0
MULX POT2
RDAX REG0, 1.0
WRAX DACL, 0.0
"#;

        // Using Delay block but with manual coefficient control
        let delay = blocks::Delay::new(0, 4000);

        let mut builder = ProgramBuilder::new();
        // Read input and save
        builder.add_inst(rdax(Register::ADCL, 1.0));
        builder.add_inst(wrax(Register::REG(0), 0.0));

        // Read delayed signal - use RDA directly with correct coefficient
        builder.add_inst(rda(4000, 0.5));

        // Add feedback
        builder.add_inst(mulx(Register::REG(17))); // POT1
        builder.add_inst(rdax(Register::REG(0), 1.0));

        // Write to delay line using Delay block
        for inst in delay.write(0.0) {
            builder.add_inst(inst);
        }

        // Mix wet/dry
        builder.add_inst(mulx(Register::REG(18))); // POT2
        builder.add_inst(rdax(Register::REG(0), 1.0));

        // Output
        builder.add_inst(wrax(Register::DACL, 0.0));

        let dsl_program = builder.build();

        let asm_binary = assemble_from_file(asm_source);
        let dsl_binary = assemble_from_dsl(dsl_program);

        assert_eq!(
            asm_binary, dsl_binary,
            "Delay echo using blocks should produce identical binary when using exact coefficients"
        );
    }

    #[test]
    fn test_lowpass_filter_block() {
        // Test the lowpass filter block
        let mut builder = ProgramBuilder::new();
        
        // Read input
        builder.add_inst(rdax(Register::ADCL, 1.0));
        
        // Apply lowpass filter
        for inst in blocks::lowpass(Register::ACC, Register::REG(16), Register::REG(1)) {
            builder.add_inst(inst);
        }
        
        // Output
        builder.add_inst(wrax(Register::DACL, 0.0));
        
        let program = builder.build();
        
        // Verify it assembles correctly
        let assembler = Assembler::new();
        let result = assembler.assemble(&program);
        assert!(result.is_ok(), "Lowpass filter program should assemble");
        
        // Verify instruction count: RDAX + 4 lowpass instructions + WRAX = 6
        assert_eq!(program.instructions().len(), 6);
    }

    #[test]
    fn test_soft_clip_block() {
        // Test the soft_clip block
        let mut builder = ProgramBuilder::new();
        
        // Read input
        builder.add_inst(rdax(Register::ADCL, 1.0));
        
        // Apply soft clipping
        for inst in blocks::soft_clip(0.8) {
            builder.add_inst(inst);
        }
        
        // Output
        builder.add_inst(wrax(Register::DACL, 0.0));
        
        let program = builder.build();
        
        // Verify it assembles correctly
        let assembler = Assembler::new();
        let result = assembler.assemble(&program);
        assert!(result.is_ok(), "Soft clip program should assemble");
        
        // Verify instruction count: RDAX + 3 soft_clip instructions + WRAX = 5
        assert_eq!(program.instructions().len(), 5);
    }

    #[test]
    fn test_complex_effect_with_multiple_blocks() {
        // Test combining multiple blocks: gain + lowpass + soft clip
        let mut builder = ProgramBuilder::new();
        
        // Gain control
        builder.add_inst(blocks::gain(Register::ADCL, Register::REG(16))); // POT0
        builder.add_inst(mulx(Register::REG(16)));
        
        // Lowpass filter
        for inst in blocks::lowpass(Register::ACC, Register::REG(17), Register::REG(1)) {
            builder.add_inst(inst);
        }
        
        // Soft clipping
        for inst in blocks::soft_clip(0.9) {
            builder.add_inst(inst);
        }
        
        // Output
        builder.add_inst(wrax(Register::DACL, 0.0));
        
        let program = builder.build();
        
        // Verify it assembles correctly
        let assembler = Assembler::new();
        let result = assembler.assemble(&program);
        assert!(result.is_ok(), "Complex multi-block effect should assemble");
        
        // Verify instruction count: 2 (gain) + 4 (lowpass) + 3 (soft_clip) + 1 (output) = 10
        assert_eq!(program.instructions().len(), 10);
    }
}
