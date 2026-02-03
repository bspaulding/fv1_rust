use crate::instruction::Instruction;
use std::collections::HashMap;

/// Complete FV-1 program
#[derive(Debug, Clone)]
pub struct Program {
    /// Assembly directives (EQU, MEM, etc.)
    pub directives: Vec<Directive>,
    /// Program statements (labels and instructions)
    pub statements: Vec<Statement>,
    /// Label name to instruction index mapping
    pub labels: HashMap<String, usize>,
}

/// Assembly directive
#[derive(Debug, Clone)]
pub enum Directive {
    /// EQU name, value - Define a symbolic constant
    Equate { name: String, value: Value },

    /// MEM name size - Allocate delay memory
    MemoryAllocation { name: String, size: u16 },

    /// SPINASM version - SpinASM compatibility directive
    SpinAsm { version: String },
}

/// Program statement (label or instruction)
#[derive(Debug, Clone)]
pub enum Statement {
    /// Label: - Defines a label at the current position
    Label(String),

    /// Instruction
    Instruction(Instruction),

    /// Label: Instruction - Label and instruction on the same line
    LabeledInstruction {
        label: String,
        instruction: Instruction,
    },
}

/// Value in an expression or directive
#[derive(Debug, Clone)]
pub enum Value {
    /// Floating-point literal
    Float(f32),
    /// Integer literal
    Integer(i64),
    /// Reference to a symbolic constant (equate)
    Identifier(String),
}

impl Program {
    /// Create a new empty program
    pub fn new() -> Self {
        Self {
            directives: Vec::new(),
            statements: Vec::new(),
            labels: HashMap::new(),
        }
    }

    /// Get all instructions in order, excluding standalone labels
    pub fn instructions(&self) -> Vec<&Instruction> {
        self.statements
            .iter()
            .filter_map(|s| match s {
                Statement::Instruction(i) => Some(i),
                Statement::LabeledInstruction { instruction, .. } => Some(instruction),
                Statement::Label(_) => None,
            })
            .collect()
    }

    /// Resolve a label to its instruction index
    pub fn resolve_label(&self, label: &str) -> Option<usize> {
        self.labels.get(label).copied()
    }

    /// Add a statement and update label mappings if needed
    pub fn add_statement(&mut self, statement: Statement) {
        match &statement {
            Statement::Label(name) => {
                // Label points to the next instruction
                self.labels.insert(name.clone(), self.instruction_count());
            }
            Statement::LabeledInstruction { label, .. } => {
                // Label points to this instruction
                self.labels.insert(label.clone(), self.instruction_count());
            }
            Statement::Instruction(_) => {}
        }
        self.statements.push(statement);
    }

    /// Get the current instruction count (for label resolution)
    fn instruction_count(&self) -> usize {
        self.statements
            .iter()
            .filter(|s| {
                matches!(
                    s,
                    Statement::Instruction(_) | Statement::LabeledInstruction { .. }
                )
            })
            .count()
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;
    use crate::register::Register;

    #[test]
    fn test_program_creation() {
        let program = Program::new();
        assert_eq!(program.directives.len(), 0);
        assert_eq!(program.statements.len(), 0);
        assert_eq!(program.labels.len(), 0);
    }

    #[test]
    fn test_program_add_instruction() {
        let mut program = Program::new();
        let inst = Instruction::CLR;
        program.add_statement(Statement::Instruction(inst));

        assert_eq!(program.statements.len(), 1);
        assert_eq!(program.instructions().len(), 1);
    }

    #[test]
    fn test_program_add_label() {
        let mut program = Program::new();
        program.add_statement(Statement::Label("start".to_string()));
        program.add_statement(Statement::Instruction(Instruction::CLR));

        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.labels.get("start"), Some(&0));
    }

    #[test]
    fn test_program_labeled_instruction() {
        let mut program = Program::new();
        let inst = Instruction::RDAX {
            reg: Register::ADCL,
            coeff: 1.0,
        };
        program.add_statement(Statement::LabeledInstruction {
            label: "read_input".to_string(),
            instruction: inst,
        });

        assert_eq!(program.statements.len(), 1);
        assert_eq!(program.instructions().len(), 1);
        assert_eq!(program.labels.get("read_input"), Some(&0));
    }

    #[test]
    fn test_program_resolve_label() {
        let mut program = Program::new();
        program.add_statement(Statement::Instruction(Instruction::CLR));
        program.add_statement(Statement::Label("loop".to_string()));
        program.add_statement(Statement::Instruction(Instruction::NOP));

        assert_eq!(program.resolve_label("loop"), Some(1));
    }

    #[test]
    fn test_value_types() {
        let float_val = Value::Float(1.5);
        let int_val = Value::Integer(42);
        let id_val = Value::Identifier("GAIN".to_string());

        match float_val {
            Value::Float(v) => assert_eq!(v, 1.5),
            _ => panic!("Wrong value type"),
        }

        match int_val {
            Value::Integer(v) => assert_eq!(v, 42),
            _ => panic!("Wrong value type"),
        }

        match id_val {
            Value::Identifier(s) => assert_eq!(s, "GAIN"),
            _ => panic!("Wrong value type"),
        }
    }
}
