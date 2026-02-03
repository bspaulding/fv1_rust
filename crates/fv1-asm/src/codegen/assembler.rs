//! FV-1 Program Assembler
//!
//! Assembles parsed programs into FV-1 binary format

use crate::{
    ast::Program, codegen::encoder::encode_instruction, constants::MAX_INSTRUCTIONS,
    error::CodegenError,
};

/// FV-1 program assembler
pub struct Assembler {
    optimize: bool,
}

impl Assembler {
    /// Create a new assembler
    pub fn new() -> Self {
        Self { optimize: false }
    }

    /// Enable or disable optimization
    pub fn with_optimization(mut self, enable: bool) -> Self {
        self.optimize = enable;
        self
    }

    /// Assemble a program into FV-1 binary
    pub fn assemble(&self, program: &Program) -> Result<Binary, CodegenError> {
        let instructions = program.instructions();

        // Check program size
        if instructions.len() > MAX_INSTRUCTIONS {
            return Err(CodegenError::ProgramTooLarge {
                size: instructions.len(),
                max: MAX_INSTRUCTIONS,
            });
        }

        let mut binary = Binary::new();

        // Encode each instruction
        for inst in instructions {
            let encoded = encode_instruction(inst)?;
            binary.push(encoded);
        }

        // Pad to 128 instructions with NOPs
        while binary.len() < MAX_INSTRUCTIONS {
            binary.push(0x00000000); // NOP
        }

        // Apply optimizations if enabled
        if self.optimize {
            binary = self.optimize_binary(binary)?;
        }

        Ok(binary)
    }

    /// Apply peephole optimizations to the binary
    fn optimize_binary(&self, binary: Binary) -> Result<Binary, CodegenError> {
        // TODO: Implement optimizations:
        // - Remove redundant CLR instructions
        // - Combine consecutive SOF operations
        // - Dead code elimination after unconditional skips
        Ok(binary)
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

/// Compiled FV-1 binary program (128 x 32-bit instructions)
#[derive(Debug, Clone)]
pub struct Binary {
    instructions: Vec<u32>,
}

impl Binary {
    /// Create a new empty binary
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    /// Add an instruction to the binary
    pub fn push(&mut self, instruction: u32) {
        self.instructions.push(instruction);
    }

    /// Get the number of instructions
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    /// Check if the binary is empty
    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    /// Get instructions as a slice
    pub fn instructions(&self) -> &[u32] {
        &self.instructions
    }

    /// Create a Binary from raw bytes (512 bytes, big-endian)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CodegenError> {
        if bytes.len() != 512 {
            return Err(CodegenError::InvalidBinarySize {
                size: bytes.len(),
                expected: 512,
            });
        }

        let mut instructions = Vec::with_capacity(MAX_INSTRUCTIONS);
        for chunk in bytes.chunks_exact(4) {
            let word = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            instructions.push(word);
        }

        Ok(Self { instructions })
    }

    /// Export as raw binary bytes (512 bytes, big-endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(512);
        for &inst in &self.instructions {
            bytes.extend_from_slice(&inst.to_be_bytes());
        }
        bytes
    }

    /// Export as Intel HEX format
    ///
    /// Intel HEX format is commonly used for programming microcontrollers
    /// and FV-1 chips. Each line contains:
    /// - Record mark (`:`)
    /// - Byte count (2 hex digits)
    /// - Address (4 hex digits)
    /// - Record type (2 hex digits, `00` = data)
    /// - Data bytes (2 hex digits each)
    /// - Checksum (2 hex digits)
    pub fn to_hex(&self) -> String {
        let mut hex = String::new();
        let bytes = self.to_bytes();

        // Generate data records (16 bytes per line)
        for (i, chunk) in bytes.chunks(16).enumerate() {
            let addr = i * 16;
            let len = chunk.len();

            // Record header: :LLAAAATT
            hex.push_str(&format!(":{:02X}{:04X}00", len, addr));

            // Data bytes and calculate checksum
            let mut checksum = len + (addr >> 8) + (addr & 0xFF);
            for &byte in chunk {
                hex.push_str(&format!("{:02X}", byte));
                checksum += byte as usize;
            }

            // Two's complement checksum
            checksum = (256 - (checksum & 0xFF)) & 0xFF;
            hex.push_str(&format!("{:02X}\n", checksum));
        }

        // End of file record
        hex.push_str(":00000001FF\n");
        hex
    }

    /// Export as C array for embedding in firmware
    pub fn to_c_array(&self, name: &str) -> String {
        let mut c_code = String::new();

        c_code.push_str(&format!(
            "// FV-1 program: {} ({} instructions)\n",
            name,
            self.len()
        ));
        c_code.push_str(&format!("const uint32_t {}[{}] = {{\n", name, self.len()));

        for (i, &inst) in self.instructions.iter().enumerate() {
            if i % 4 == 0 {
                c_code.push_str("    ");
            }
            c_code.push_str(&format!("0x{:08X}", inst));
            if i < self.instructions.len() - 1 {
                c_code.push(',');
            }
            if (i + 1) % 4 == 0 {
                c_code.push('\n');
            } else if i < self.instructions.len() - 1 {
                c_code.push(' ');
            }
        }

        if !self.len().is_multiple_of(4) {
            c_code.push('\n');
        }
        c_code.push_str("};\n");
        c_code
    }
}

impl Default for Binary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::Statement, instruction::Instruction, register::Register};

    #[test]
    fn test_assembler_creation() {
        let assembler = Assembler::new();
        assert!(!assembler.optimize);
    }

    #[test]
    fn test_assembler_with_optimization() {
        let assembler = Assembler::new().with_optimization(true);
        assert!(assembler.optimize);
    }

    #[test]
    fn test_assemble_simple_program() {
        let mut program = Program::new();
        program.add_statement(Statement::Instruction(Instruction::CLR));
        program.add_statement(Statement::Instruction(Instruction::RDAX {
            reg: Register::ADCL,
            coeff: 1.0,
        }));
        program.add_statement(Statement::Instruction(Instruction::WRAX {
            reg: Register::DACL,
            coeff: 0.0,
        }));

        let assembler = Assembler::new();
        let binary = assembler.assemble(&program).unwrap();

        assert_eq!(binary.len(), MAX_INSTRUCTIONS);
        assert_eq!(binary.instructions()[0] >> 27, 0b01110); // CLR
        assert_eq!(binary.instructions()[1] >> 27, 0b00000); // RDAX
        assert_eq!(binary.instructions()[2] >> 27, 0b00110); // WRAX
    }

    #[test]
    fn test_assemble_program_too_large() {
        let mut program = Program::new();
        for _ in 0..129 {
            program.add_statement(Statement::Instruction(Instruction::NOP));
        }

        let assembler = Assembler::new();
        let result = assembler.assemble(&program);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CodegenError::ProgramTooLarge {
                size: 129,
                max: 128
            }
        ));
    }

    #[test]
    fn test_binary_creation() {
        let binary = Binary::new();
        assert_eq!(binary.len(), 0);
        assert!(binary.is_empty());
    }

    #[test]
    fn test_binary_push() {
        let mut binary = Binary::new();
        binary.push(0x12345678);
        binary.push(0xABCDEF00);

        assert_eq!(binary.len(), 2);
        assert!(!binary.is_empty());
        assert_eq!(binary.instructions()[0], 0x12345678);
        assert_eq!(binary.instructions()[1], 0xABCDEF00);
    }

    #[test]
    fn test_binary_to_bytes() {
        let mut binary = Binary::new();
        binary.push(0x12345678);

        let bytes = binary.to_bytes();
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_binary_to_hex() {
        let mut binary = Binary::new();
        for _ in 0..MAX_INSTRUCTIONS {
            binary.push(0x00000000);
        }

        let hex = binary.to_hex();
        assert!(hex.starts_with(':'));
        assert!(hex.ends_with(":00000001FF\n"));
        assert!(hex.contains("00000000"));
    }

    #[test]
    fn test_binary_to_c_array() {
        let mut binary = Binary::new();
        binary.push(0x12345678);
        binary.push(0xABCDEF00);

        let c_code = binary.to_c_array("test_program");
        assert!(c_code.contains("const uint32_t test_program"));
        assert!(c_code.contains("0x12345678"));
        assert!(c_code.contains("0xABCDEF00"));
    }

    #[test]
    fn test_assemble_with_labels() {
        let mut program = Program::new();
        program.add_statement(Statement::Label("start".to_string()));
        program.add_statement(Statement::Instruction(Instruction::CLR));
        program.add_statement(Statement::LabeledInstruction {
            label: "loop".to_string(),
            instruction: Instruction::NOP,
        });

        let assembler = Assembler::new();
        let binary = assembler.assemble(&program).unwrap();

        assert_eq!(binary.len(), MAX_INSTRUCTIONS);
        assert_eq!(program.resolve_label("start"), Some(0));
        assert_eq!(program.resolve_label("loop"), Some(1));
    }
}
