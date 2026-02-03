//! FV-1 Code Generation Module
//!
//! This module handles the conversion from parsed AST to FV-1 machine code.

pub mod assembler;
pub mod encoder;

// Re-export main types for convenience
pub use assembler::{Assembler, Binary};
pub use encoder::encode_instruction;
