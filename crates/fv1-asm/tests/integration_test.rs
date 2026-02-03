//! Integration test for complete assembler workflow

use fv1_asm::{Assembler, Instruction, Parser, Register, Statement};

#[test]
fn test_complete_assembler_workflow() {
    // Parse a simple FV-1 program
    let source = r#"
        ; Simple passthrough with gain control
        rdax adcl, 1.0
        wrax dacl, 0.0
    "#;

    let mut parser = Parser::new(source);
    let program = parser.parse().expect("Failed to parse program");

    // Assemble to binary
    let assembler = Assembler::new();
    let binary = assembler.assemble(&program).expect("Failed to assemble");

    // Verify binary properties
    assert_eq!(binary.len(), 128); // FV-1 requires exactly 128 instructions
    assert!(!binary.is_empty());

    // Check that instructions are encoded
    let instructions = binary.instructions();
    assert_eq!(instructions[0] >> 27, 0b00000); // RDAX opcode
    assert_eq!(instructions[1] >> 27, 0b00110); // WRAX opcode

    // Test binary output formats
    let bytes = binary.to_bytes();
    assert_eq!(bytes.len(), 512); // 128 instructions * 4 bytes

    let hex = binary.to_hex();
    assert!(hex.starts_with(':')); // Intel HEX format
    assert!(hex.ends_with(":00000001FF\n")); // EOF record

    let c_array = binary.to_c_array("test_prog");
    assert!(c_array.contains("const uint32_t test_prog"));
}

#[test]
fn test_program_with_labels() {
    let source = r#"
        start: rdax adcl, 1.0
        loop:  sof 0.5, 0.0
               wrax dacl, 0.0
    "#;

    let mut parser = Parser::new(source);
    let program = parser.parse().expect("Failed to parse");

    // Verify label resolution
    assert_eq!(program.resolve_label("start"), Some(0));
    assert_eq!(program.resolve_label("loop"), Some(1));

    // Assemble successfully
    let assembler = Assembler::new();
    let binary = assembler.assemble(&program).expect("Failed to assemble");
    assert_eq!(binary.len(), 128);
}

#[test]
fn test_all_instruction_types_encode() {
    let mut program = fv1_asm::Program::new();

    // Add various instruction types to test encoding
    program.add_statement(Statement::Instruction(Instruction::CLR));
    program.add_statement(Statement::Instruction(Instruction::RDAX {
        reg: Register::ADCL,
        coeff: 1.0,
    }));
    program.add_statement(Statement::Instruction(Instruction::SOF {
        coeff: 0.5,
        offset: 0.0,
    }));
    program.add_statement(Statement::Instruction(Instruction::MULX {
        reg: Register::REG(0),
    }));
    program.add_statement(Statement::Instruction(Instruction::WRAX {
        reg: Register::DACL,
        coeff: 0.0,
    }));

    let assembler = Assembler::new();
    let result = assembler.assemble(&program);
    assert!(result.is_ok());

    let binary = result.unwrap();
    assert_eq!(binary.len(), 128);
}

#[test]
fn test_program_size_validation() {
    let mut program = fv1_asm::Program::new();

    // Add more than 128 instructions
    for _ in 0..129 {
        program.add_statement(Statement::Instruction(Instruction::NOP));
    }

    let assembler = Assembler::new();
    let result = assembler.assemble(&program);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        fv1_asm::CodegenError::ProgramTooLarge { size: 129, max: 128 }
    ));
}
