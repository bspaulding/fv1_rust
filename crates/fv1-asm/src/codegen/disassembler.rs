//! FV-1 Program Disassembler
//!
//! Disassembles FV-1 binary format into assembly source code

use crate::{
    ast::{Program, Statement},
    codegen::{decoder::decode_instruction, Binary},
    error::CodegenError,
    instruction::{ChoMode, Instruction, SkipCondition},
    register::{Lfo, Register},
};

/// FV-1 program disassembler
pub struct Disassembler {
    strip_nops: bool,
}

impl Disassembler {
    /// Create a new disassembler
    pub fn new() -> Self {
        Self { strip_nops: true }
    }

    /// Control whether to strip trailing NOPs
    pub fn with_strip_nops(mut self, strip: bool) -> Self {
        self.strip_nops = strip;
        self
    }

    /// Disassemble a binary into a Program
    pub fn disassemble(&self, binary: &Binary) -> Result<Program, CodegenError> {
        let mut program = Program::new();

        for (idx, &word) in binary.instructions().iter().enumerate() {
            let inst = decode_instruction(word)?;

            // Skip trailing NOPs if enabled
            if self.strip_nops && matches!(inst, Instruction::NOP) {
                // Check if all remaining instructions are also NOPs
                let all_nops = binary.instructions()[idx..]
                    .iter()
                    .all(|&w| w == 0x00000000);
                if all_nops {
                    break;
                }
            }

            program.add_statement(Statement::Instruction(inst));
        }

        Ok(program)
    }

    /// Disassemble to assembly source code string
    pub fn disassemble_to_source(&self, binary: &Binary) -> Result<String, CodegenError> {
        let program = self.disassemble(binary)?;
        Ok(format_program(&program))
    }
}

impl Default for Disassembler {
    fn default() -> Self {
        Self::new()
    }
}

/// Format a program as assembly source code
fn format_program(program: &Program) -> String {
    let mut source = String::new();

    for statement in &program.statements {
        match statement {
            Statement::Instruction(inst) => {
                source.push_str(&format_instruction(inst));
                source.push('\n');
            }
            Statement::Label(label) => {
                source.push_str(label);
                source.push_str(":\n");
            }
            Statement::LabeledInstruction { label, instruction } => {
                source.push_str(label);
                source.push_str(": ");
                source.push_str(&format_instruction(instruction));
                source.push('\n');
            }
        }
    }

    source
}

/// Format a single instruction as assembly text
fn format_instruction(inst: &Instruction) -> String {
    match inst {
        Instruction::RDAX { reg, coeff } => format!("RDAX {}, {}", format_register(reg), coeff),
        Instruction::RDA { addr, coeff } => format!("RDA {}, {}", addr, coeff),
        Instruction::RMPA { coeff } => format!("RMPA {}", coeff),
        Instruction::WRAX { reg, coeff } => format!("WRAX {}, {}", format_register(reg), coeff),
        Instruction::WRA { addr, coeff } => format!("WRA {}, {}", addr, coeff),
        Instruction::WRAP { addr, coeff } => format!("WRAP {}, {}", addr, coeff),
        Instruction::MULX { reg } => format!("MULX {}", format_register(reg)),
        Instruction::RDFX { reg, coeff } => format!("RDFX {}, {}", format_register(reg), coeff),
        Instruction::RDFX2 { reg, coeff } => format!("RDFX2 {}, {}", format_register(reg), coeff),
        Instruction::LDAX { reg } => format!("LDAX {}", format_register(reg)),
        Instruction::ABSA => "ABSA".to_string(),
        Instruction::SOF { coeff, offset } => format!("SOF {}, {}", coeff, offset),
        Instruction::AND { mask } => format!("AND 0x{:06X}", mask),
        Instruction::OR { mask } => format!("OR 0x{:06X}", mask),
        Instruction::XOR { mask } => format!("XOR 0x{:06X}", mask),
        Instruction::SHL => "SHL".to_string(),
        Instruction::SHR => "SHR".to_string(),
        Instruction::CLR => "CLR".to_string(),
        Instruction::NOP => "NOP".to_string(),
        Instruction::EXP { coeff, offset } => format!("EXP {}, {}", coeff, offset),
        Instruction::LOG { coeff, offset } => format!("LOG {}, {}", coeff, offset),
        Instruction::SKP { condition, offset } => {
            format!("SKP {}, {}", format_skip_condition(condition), offset)
        }
        Instruction::WLDS {
            lfo,
            freq,
            amplitude,
        } => format!("WLDS {}, {}, {}", format_lfo(lfo), freq, amplitude),
        Instruction::JAM { lfo } => format!("JAM {}", format_lfo(lfo)),
        Instruction::CHO {
            mode,
            lfo,
            flags,
            addr,
        } => {
            let mut parts: Vec<String> = vec![
                format_cho_mode(mode).to_string(),
                format_lfo(lfo).to_string(),
            ];
            if flags.rptr2 {
                parts.push("RPTR2".to_string());
            }
            if flags.na {
                parts.push("NA".to_string());
            }
            if flags.compc {
                parts.push("COMPC".to_string());
            }
            if flags.compa {
                parts.push("COMPA".to_string());
            }
            if flags.rptr2_select {
                parts.push("RPTR2_SEL".to_string());
            }
            parts.push(addr.to_string());
            format!("CHO {}", parts.join(", "))
        }
    }
}

fn format_register(reg: &Register) -> String {
    match reg {
        Register::ACC => "ACC".to_string(),
        Register::ADCL => "ADCL".to_string(),
        Register::ADCR => "ADCR".to_string(),
        Register::DACL => "DACL".to_string(),
        Register::DACR => "DACR".to_string(),
        Register::REG(n) => format!("REG{}", n),
        Register::ADDR_PTR => "ADDR_PTR".to_string(),
        Register::LR => "LR".to_string(),
        Register::SIN0_RATE => "SIN0_RATE".to_string(),
        Register::SIN0_RANGE => "SIN0_RANGE".to_string(),
        Register::SIN1_RATE => "SIN1_RATE".to_string(),
        Register::SIN1_RANGE => "SIN1_RANGE".to_string(),
        Register::RMP0_RATE => "RMP0_RATE".to_string(),
        Register::RMP0_RANGE => "RMP0_RANGE".to_string(),
        Register::RMP1_RATE => "RMP1_RATE".to_string(),
        Register::RMP1_RANGE => "RMP1_RANGE".to_string(),
    }
}

fn format_skip_condition(cond: &SkipCondition) -> &str {
    match cond {
        SkipCondition::RUN => "RUN",
        SkipCondition::NEG => "NEG",
        SkipCondition::GEZ => "GEZ",
        SkipCondition::ZRO => "ZRO",
        SkipCondition::ZRC => "ZRC",
    }
}

fn format_lfo(lfo: &Lfo) -> &str {
    match lfo {
        Lfo::SIN0 => "SIN0",
        Lfo::SIN1 => "SIN1",
        Lfo::RMP0 => "RMP0",
        Lfo::RMP1 => "RMP1",
    }
}

fn format_cho_mode(mode: &ChoMode) -> &str {
    match mode {
        ChoMode::RDA => "RDA",
        ChoMode::SOF => "SOF",
        ChoMode::RDAL => "RDAL",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{codegen::Assembler, parser::Parser};

    #[test]
    fn test_disassemble_simple() {
        let source = "RDAX ADCL, 1.0\nWRAX DACL, 0.0\n";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        let assembler = Assembler::new();
        let binary = assembler.assemble(&program).unwrap();

        let disassembler = Disassembler::new();
        let disassembled = disassembler.disassemble(&binary).unwrap();

        assert_eq!(disassembled.instructions().len(), 2);
    }

    #[test]
    fn test_roundtrip() {
        let source = "RDAX ADCL, 0.5\nMULX REG0\nWRAX DACL, 0.0\n";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        let assembler = Assembler::new();
        let binary = assembler.assemble(&program).unwrap();

        let disassembler = Disassembler::new();
        let disassembled = disassembler.disassemble(&binary).unwrap();

        assert_eq!(
            program.instructions().len(),
            disassembled.instructions().len()
        );
    }
}
