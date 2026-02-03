// Property tests for FV-1 assembler/disassembler

#![allow(clippy::identity_op)] // Allow 0 << 27 for readability in opcode definitions

use fv1_asm::{Assembler, Binary, Disassembler, Parser};
use proptest::prelude::*;

// Test that disassemble -> assemble -> disassemble produces the same result
proptest! {
    #[test]
    fn test_binary_roundtrip(
        // Generate a binary with random instruction words
        // We'll generate valid FV-1 instruction words
        words in prop::collection::vec(valid_instruction_word(), 1..20)
    ) {
        // Create a binary from the generated words
        let mut binary1 = Binary::new();
        for word in &words {
            binary1.push(*word);
        }
        // Pad to 128 instructions
        while binary1.len() < 128 {
            binary1.push(0x00000000); // NOP
        }

        // Disassemble
        let disassembler = Disassembler::new().with_strip_nops(true);
        let source = disassembler.disassemble_to_source(&binary1).unwrap();

        // Assemble
        let mut parser = Parser::new(&source);
        let program = parser.parse().unwrap();
        let assembler = Assembler::new();
        let binary2 = assembler.assemble(&program).unwrap();

        // Disassemble again
        let source2 = disassembler.disassemble_to_source(&binary2).unwrap();

        // The two disassembled sources should be identical
        prop_assert_eq!(source, source2);
    }
}

// Generate a valid FV-1 instruction word
fn valid_instruction_word() -> impl Strategy<Value = u32> {
    prop_oneof![
        // RDAX: opcode 0b00000, reg (6 bits), coeff (15 bits)
        (valid_register(), valid_s114()).prop_map(|(reg, coeff)| {
            (0b00000_u32 << 27) | ((reg & 0x3F) << 21) | ((coeff & 0x7FFF) << 6)
        }),
        // WRAX: opcode 0b00110, reg (6 bits), coeff (15 bits)
        (valid_register(), valid_s114()).prop_map(|(reg, coeff)| {
            (0b00110_u32 << 27) | ((reg & 0x3F) << 21) | ((coeff & 0x7FFF) << 6)
        }),
        // SOF: opcode 0b01101, coeff (16 bits), offset (11 bits)
        (valid_s114(), valid_s10()).prop_map(|(coeff, offset)| {
            (0b01101_u32 << 27) | ((coeff & 0xFFFF) << 11) | (offset & 0x7FF)
        }),
        // MULX: opcode 0b01010, reg (6 bits)
        valid_register().prop_map(|reg| {
            (0b01010_u32 << 27) | ((reg & 0x3F) << 21)
        }),
        // CLR: opcode 0b01110
        Just(0b01110_u32 << 27),
        // NOP
        Just(0x00000000),
        // AND: opcode 0b01111, mask (24 bits)
        (0u32..0x1000000).prop_map(|mask| {
            (0b01111_u32 << 27) | (mask & 0xFFFFFF)
        }),
        // OR: opcode 0b10000, mask (24 bits)
        (0u32..0x1000000).prop_map(|mask| {
            (0b10000_u32 << 27) | (mask & 0xFFFFFF)
        }),
        // XOR: opcode 0b10001, mask (24 bits)
        (0u32..0x1000000).prop_map(|mask| {
            (0b10001_u32 << 27) | (mask & 0xFFFFFF)
        }),
    ]
}

// Generate a valid register code
fn valid_register() -> impl Strategy<Value = u32> {
    prop_oneof![
        // ADCL, ADCR, DACL, DACR, ADDR_PTR, LR
        Just(0u32),  // ADCL
        Just(1u32),  // ADCR
        Just(2u32),  // DACL
        Just(3u32),  // DACR
        Just(4u32),  // ADDR_PTR
        Just(5u32),  // LR
        // REG0-REG31 (encoded as 16-47)
        (16u32..48),
    ]
}

// Generate a valid S1.14 coefficient (15-bit signed value)
fn valid_s114() -> impl Strategy<Value = u32> {
    // S1.14 format: 15-bit signed, range [-16384, 16383]
    // We want values that can roundtrip, so we'll generate actual S1.14 encoded values
    (-16384i32..16384).prop_map(|val| (val & 0x7FFF) as u32)
}

// Generate a valid S.10 coefficient (11-bit signed value)
fn valid_s10() -> impl Strategy<Value = u32> {
    // S.10 format: 11-bit signed, range [-512, 511]
    (-512i32..512).prop_map(|val| (val & 0x7FF) as u32)
}

#[cfg(test)]
mod regular_tests {
    use super::*;

    #[test]
    fn test_specific_roundtrip() {
        // Test a specific known case
        let source1 = "RDAX ADCL, 0.5\nWRAX DACL, 0.0\n";
        let mut parser = Parser::new(source1);
        let program1 = parser.parse().unwrap();

        let assembler = Assembler::new();
        let binary = assembler.assemble(&program1).unwrap();

        let disassembler = Disassembler::new();
        let source2 = disassembler.disassemble_to_source(&binary).unwrap();

        let mut parser2 = Parser::new(&source2);
        let program2 = parser2.parse().unwrap();

        // Should have the same number of instructions
        assert_eq!(
            program1.instructions().len(),
            program2.instructions().len()
        );
    }

    #[test]
    fn test_roundtrip_with_multiple_instructions() {
        let source1 = "SOF 0.5, 0.0\nMULX REG0\nWRAX DACL, 0.0\n";
        let mut parser = Parser::new(source1);
        let program1 = parser.parse().unwrap();

        let assembler = Assembler::new();
        let binary = assembler.assemble(&program1).unwrap();

        let disassembler = Disassembler::new();
        let source2 = disassembler.disassemble_to_source(&binary).unwrap();

        let mut parser2 = Parser::new(&source2);
        let program2 = parser2.parse().unwrap();

        // The binary should be the same when re-assembled
        let binary2 = assembler.assemble(&program2).unwrap();

        assert_eq!(binary.instructions(), binary2.instructions());
    }
}
