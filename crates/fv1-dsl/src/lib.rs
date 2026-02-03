pub mod ops;

pub use fv1_asm::{Instruction, Register, Program, Statement, Control, Lfo, SkipCondition, ChoMode, ChoFlags};
pub use fv1_dsl_macro::fv1_program;

use std::collections::HashMap;

/// Builder for FV-1 programs using Rust API
///
/// This provides a fluent interface for constructing FV-1 programs programmatically.
///
/// # Example
///
/// ```
/// use fv1_dsl::ProgramBuilder;
/// use fv1_asm::{Instruction, Register};
///
/// // Using builder pattern (consuming self)
/// let program = ProgramBuilder::new()
///     .inst(Instruction::RDAX { reg: Register::ADCL, coeff: 1.0 })
///     .inst(Instruction::WRAX { reg: Register::DACL, coeff: 0.0 })
///     .build();
/// ```
pub struct ProgramBuilder {
    instructions: Vec<Instruction>,
    labels: HashMap<String, usize>,
}

impl ProgramBuilder {
    /// Create a new empty program builder
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }
    
    /// Add an instruction to the program (builder pattern - consumes self)
    pub fn inst(mut self, inst: Instruction) -> Self {
        self.instructions.push(inst);
        self
    }
    
    /// Add an instruction to the program (mutable reference - for use in macros)
    pub fn add_inst(&mut self, inst: Instruction) -> &mut Self {
        self.instructions.push(inst);
        self
    }
    
    /// Add a label at the current instruction position (builder pattern - consumes self)
    pub fn label(mut self, name: impl Into<String>) -> Self {
        self.labels.insert(name.into(), self.instructions.len());
        self
    }
    
    /// Add a label at the current instruction position (mutable reference)
    pub fn add_label(&mut self, name: impl Into<String>) -> &mut Self {
        self.labels.insert(name.into(), self.instructions.len());
        self
    }
    
    /// Build the final program
    pub fn build(self) -> Program {
        let mut program = Program::new();
        
        // Add all instructions
        for inst in self.instructions {
            program.add_statement(Statement::Instruction(inst));
        }
        
        // Add labels (they will point to the instruction indices)
        for (name, idx) in self.labels {
            program.labels.insert(name, idx);
        }
        
        program
    }
}

impl Default for ProgramBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::ops::*;
    pub use crate::{ProgramBuilder, Register, Instruction, Control, Lfo, SkipCondition, ChoMode, ChoFlags};
    pub use fv1_dsl_macro::fv1_program;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builder_creation() {
        let builder = ProgramBuilder::new();
        assert_eq!(builder.instructions.len(), 0);
        assert_eq!(builder.labels.len(), 0);
    }
    
    #[test]
    fn test_builder_add_instruction() {
        let mut builder = ProgramBuilder::new();
        builder.add_inst(Instruction::CLR);
        assert_eq!(builder.instructions.len(), 1);
    }
    
    #[test]
    fn test_builder_add_label() {
        let mut builder = ProgramBuilder::new();
        builder.add_inst(Instruction::CLR)
               .add_label("start")
               .add_inst(Instruction::NOP);
        
        assert_eq!(builder.instructions.len(), 2);
        assert_eq!(builder.labels.len(), 1);
        assert_eq!(builder.labels.get("start"), Some(&1));
    }
    
    #[test]
    fn test_builder_build() {
        let mut builder = ProgramBuilder::new();
        builder.add_inst(Instruction::RDAX { reg: Register::ADCL, coeff: 1.0 })
               .add_inst(Instruction::WRAX { reg: Register::DACL, coeff: 0.0 });
        
        let program = builder.build();
        assert_eq!(program.instructions().len(), 2);
    }
    
    #[test]
    fn test_builder_fluent_api() {
        let program = ProgramBuilder::new()
            .inst(Instruction::CLR)
            .inst(Instruction::RDAX { reg: Register::ADCL, coeff: 1.0 })
            .inst(Instruction::WRAX { reg: Register::DACL, coeff: 0.0 })
            .build();
        
        assert_eq!(program.instructions().len(), 3);
    }
}
