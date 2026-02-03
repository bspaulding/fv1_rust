use fv1_dsl::prelude::*;

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
