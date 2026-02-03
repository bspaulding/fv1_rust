#![allow(unused_assignments)]

use miette::Diagnostic;
use thiserror::Error;

/// Errors that can occur during parsing
#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("unexpected end of file")]
    #[diagnostic(code(parse::unexpected_eof))]
    UnexpectedEof,

    #[error("unexpected token: expected {expected}, found {found}")]
    #[diagnostic(code(parse::unexpected_token))]
    UnexpectedToken {
        expected: String,
        found: String,
        #[label("unexpected token here")]
        span: std::ops::Range<usize>,
    },

    #[error("expected register")]
    #[diagnostic(code(parse::expected_register))]
    ExpectedRegister {
        #[label("expected register here")]
        span: std::ops::Range<usize>,
    },

    #[error("expected number")]
    #[diagnostic(code(parse::expected_number))]
    ExpectedNumber {
        #[label("expected number here")]
        span: std::ops::Range<usize>,
    },

    #[error("too many instructions: {count} (max {max})")]
    #[diagnostic(code(parse::too_many_instructions))]
    TooManyInstructions {
        max: usize,
        count: usize,
        #[label("instruction limit exceeded here")]
        span: std::ops::Range<usize>,
    },

    #[error("undefined label: {name}")]
    #[diagnostic(code(parse::undefined_label))]
    UndefinedLabel {
        name: String,
        #[label("label used here")]
        span: std::ops::Range<usize>,
    },

    #[error("invalid token")]
    #[diagnostic(code(parse::invalid_token))]
    InvalidToken {
        #[label("invalid token here")]
        span: std::ops::Range<usize>,
    },
}

/// Errors that can occur during code generation
#[derive(Error, Debug, Diagnostic)]
pub enum CodegenError {
    #[error("coefficient {value} out of range (must fit in FV-1 fixed-point format)")]
    #[diagnostic(code(codegen::coefficient_out_of_range))]
    CoefficientOutOfRange { value: f32 },

    #[error("address {addr} out of range (max {max})")]
    #[diagnostic(code(codegen::address_out_of_range))]
    AddressOutOfRange { addr: u16, max: u16 },

    #[error("program too large: {size} instructions (max {max})")]
    #[diagnostic(code(codegen::program_too_large))]
    ProgramTooLarge { size: usize, max: usize },
}
