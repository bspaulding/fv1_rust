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
    assert!(bytes.len() >= 12, "Binary should contain at least 12 bytes for 3 instructions");
}
