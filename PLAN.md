# Spin FV-1 Rust Implementation Plan

A comprehensive roadmap for building a complete Rust ecosystem for FV-1 DSP programming.

**Strategy**: Build Option B (Assembler) → Option A (DSL) → Option C (Framework)

-----

## 📊 Progress Tracker

### ✅ Completed Milestones

#### Phase 1: Milestone 1.1 - Project Setup & Core Types (Week 1)
**Status**: ✅ COMPLETE  
**Completed**: 2026-02-02

**Deliverables**:
- ✅ Cargo workspace created with three crates: `fv1-asm`, `fv1-cli`, `fv1-examples`
- ✅ Core type definitions implemented:
  - `Register` enum with all FV-1 registers (ACC, ADCL, ADCR, DACL, DACR, REG(0-31), special registers)
  - `Control` enum for POT0-POT2 inputs
  - `Lfo` enum for oscillators (SIN0, SIN1, RMP0, RMP1)
  - Register validation with bounds checking via `Register::reg()` method
- ✅ Complete FV-1 instruction set implemented as Rust enums:
  - Accumulator operations (RDAX, RDA, RMPA, WRAX, WRA, WRAP)
  - Mathematical operations (MULX, RDFX, ABSA, LDAX)
  - Filtering operations (RDFX2)
  - Logic and control (SOF, AND, OR, XOR, SHL, SHR, CLR, NOP)
  - Conversion operations (EXP, LOG)
  - Conditional execution (SKP with conditions)
  - LFO control (WLDS, JAM, CHO)
- ✅ Hardware constants defined (sample rate, memory sizes, fixed-point formats)
- ✅ 11 passing unit tests covering all core types
- ✅ GitHub Actions CI/CD workflow configured:
  - Test job (cargo test --all)
  - Build job (cargo build --all)
  - Fmt job (code formatting checks)
  - Clippy job (linting)
  - Cargo caching for faster builds
  - Security permissions configured
- ✅ Code review completed, feedback addressed
- ✅ CodeQL security scan passed with 0 alerts
- ✅ Documentation: README.md created with project overview

**Commits**:
- `725b8b1` - Implement Milestone 1.1: Project setup and core types
- `a0c1f32` - Address code review feedback: improve documentation and add register validation
- `335e486` - Add GitHub CI workflow for tests and linting
- `d4dfd8b` - Add explicit permissions to CI workflow for security

#### Phase 1: Milestone 1.2 - Assembler Core (Week 2-3)
**Status**: ✅ COMPLETE  
**Completed**: 2026-02-03

**Deliverables**:
- ✅ Lexer implementation (`lexer.rs`):
  - Tokenizes FV-1 assembly using `logos` crate
  - All FV-1 instructions (RDAX, RDA, WRAX, SOF, CLR, etc.)
  - All registers (ADCL/ADCR/DACL/DACR, REG0-31, POT0-2)
  - LFO oscillators (SIN0/1, RMP0/1)
  - Numeric literals (float, hex, binary)
  - Case-insensitive token matching
  - Automatic comment stripping (`;` line comments)
- ✅ Parser implementation (`parser.rs`):
  - Recursive descent parser converting tokens to AST
  - All instruction types with proper operands
  - Label support (standalone and inline)
  - Directive support (EQU, MEM, SPINASM)
  - Expression evaluation for coefficients
- ✅ AST data structures (`ast.rs`):
  - `Program`: Complete program representation
  - `Directive`: EQU, MEM, SpinAsm directives
  - `Statement`: Instructions, labels, labeled instructions
  - `Value`: Float, integer, identifier values
  - Label-to-instruction-index resolution
- ✅ Error handling (`error.rs`):
  - `ParseError` with source span information
  - `CodegenError` for code generation
  - Beautiful error diagnostics using `miette`
- ✅ Dependencies added: logos, thiserror, miette
- ✅ 27 new unit tests (36 total), all passing
- ✅ Code review completed, feedback addressed
- ✅ CodeQL security scan passed with 0 alerts
- ✅ Clippy and rustfmt compliance

**Commits**:
- `3bb2c6e` - Implement Milestone 1.2 core: lexer, parser, AST, and error types
- `85fbef2` - Address code review feedback: fix return types and improve error handling

#### Phase 1: Milestone 1.3 - Code Generation (Week 3-4)
**Status**: ✅ COMPLETE  
**Completed**: 2026-02-03

**Deliverables**:
- ✅ Code generation module (`codegen/`):
  - `encoder.rs`: Instruction encoding to 32-bit FV-1 machine code
  - `assembler.rs`: Program assembly and binary generation
  - `mod.rs`: Module exports
- ✅ Instruction encoding implementation:
  - All accumulator operations (RDAX, WRAX, RDA, WRA, WRAP, RMPA)
  - All mathematical operations (MULX, RDFX, RDFX2, LDAX, ABSA)
  - All logic operations (AND, OR, XOR, SHL, SHR)
  - Control instructions (CLR, NOP, SOF)
  - Conversion operations (EXP, LOG)
  - Conditional execution (SKP with all conditions)
  - LFO control (WLDS, JAM, CHO)
- ✅ Fixed-point encoding functions:
  - S1.14 format for coefficients (-2.0 to ~2.0)
  - S.10 format for offsets (-1.0 to ~1.0)
  - 16-bit address encoding with validation
- ✅ Register encoding (5-bit fields)
- ✅ Binary output formats:
  - Raw bytes (512 bytes, big-endian)
  - Intel HEX format for programming
  - C array format for firmware embedding
- ✅ Program assembler:
  - Converts parsed AST to FV-1 binary
  - Validates program size (max 128 instructions)
  - Automatic NOP padding to 128 instructions
  - Optimization support (framework for future optimizations)
- ✅ 22 new unit tests (60 total), all passing
- ✅ Integrated into library exports
- ✅ Clippy and rustfmt compliance
- ✅ Code review completed, feedback addressed
- ✅ CodeQL security scan passed with 0 alerts

**Commits**:
- `2362130` - Implement Milestone 1.3: Code Generation module with encoder and assembler

#### Phase 1: Milestone 1.4 - CLI Tool Enhancement (Week 4-5)
**Status**: ✅ COMPLETE  
**Completed**: 2026-02-03

**Deliverables**:
- ✅ Enhanced CLI tool (`fv1-cli`):
  - Command-line argument parsing using `clap`
  - File input support for `.asm` files
  - Multiple output formats (binary, hex, C array)
  - Output file path configuration
  - Verbose mode for detailed progress output
  - Optimization flag support
- ✅ Error reporting with miette:
  - Beautiful diagnostic messages
  - Context lines for errors
  - Terminal hyperlinks support
  - Unicode symbols for better readability
- ✅ Example FV-1 assembly programs:
  - `passthrough.asm`: Simple audio pass-through
  - `gain_control.asm`: Volume control using POT0
  - `delay_echo.asm`: Basic delay/echo effect with feedback
  - All examples properly documented with comments
- ✅ Parser enhancement:
  - POT0-2 tokens now accepted as registers
  - Maps POT0-2 to appropriate register addresses
- ✅ Documentation updates:
  - README.md updated with CLI usage examples
  - Example program documentation
  - Feature list and current status
- ✅ All existing tests passing (64 total)
- ✅ CLI tested with all output formats
- ✅ Error reporting verified with invalid input

**Commits**:
- `5387e59` - Implement enhanced CLI tool with multiple output formats
- `5c3eb11` - Update documentation and mark milestone complete

#### Phase 1: Milestone 1.5 - CLI Tool with Subcommands (Week 4-5)
**Status**: ✅ COMPLETE  
**Completed**: 2026-02-03

**Deliverables**:
- ✅ CLI restructured with subcommand architecture:
  - `assemble` subcommand: Assembles .asm files to binary/hex/C array formats
  - `check` subcommand: Validates assembly files without generating output
  - `disassemble` subcommand: Stub implementation (marked as TODO)
- ✅ Assemble subcommand features:
  - Input/output file specification
  - Multiple output formats (bin, hex, C array)
  - Optimization flag (-O/--optimize)
  - Verbose output mode (-v/--verbose)
  - Custom C array naming (--name)
- ✅ Check subcommand:
  - Validates assembly syntax and structure
  - Reports instruction and label counts
  - Provides clear success/error messages
- ✅ Improved CLI structure:
  - Better command organization with subcommands
  - Consistent help messages for all commands
  - Proper argument parsing with clap
- ✅ All existing tests passing (64 total)
- ✅ Clippy and rustfmt compliance
- ✅ CLI tested with all three subcommands

**Commits**:
- `2a458e7` - Implement Milestone 1.5: CLI tool with subcommands
- `c55ed89` - Update PLAN.md: Mark Milestone 1.5 as complete

#### Phase 1: Milestone 1.6 - Disassembler Implementation (Week 5)
**Status**: ✅ COMPLETE  
**Completed**: 2026-02-03

**Deliverables**:
- ✅ Decoder module (`codegen/decoder.rs`):
  - Converts 32-bit FV-1 machine code back to AST instructions
  - Decodes all FV-1 instruction types (25+ opcodes)
  - Handles S1.14 and S.10 fixed-point formats with proper sign extension
  - Comprehensive error handling with detailed error types
- ✅ Disassembler module (`codegen/disassembler.rs`):
  - Converts binary programs to human-readable assembly source
  - Formats instructions with proper syntax
  - Optional NOP stripping
  - Register name formatting
- ✅ Binary loading (`Binary::from_bytes()`):
  - Loads FV-1 binary from 512-byte files
  - Validates file size
- ✅ CLI integration:
  - Full `disassemble` subcommand implementation
  - Reads binary files and writes assembly source
  - Error reporting with miette
- ✅ Fixed-point encoding improvements:
  - Fixed S1.14 edge case where 1.0 was encoding as -1.0
  - Added clamping to valid signed ranges before masking
  - S1.14: [-16384, 16383] representing [-2.0, ~2.0)
  - S.10: [-512, 511] representing [-1.0, ~1.0)
- ✅ Property-based testing:
  - Added `proptest` dependency for property testing
  - Implemented roundtrip property: `disassemble(assemble(disassemble(x))) == disassemble(x)`
  - Tests random valid instruction sequences
  - 100 test cases per property test run
- ✅ Comprehensive testing:
  - 66 unit tests (existing + new decoder tests)
  - 4 integration tests
  - 3 property tests
  - 2 example tests
  - **Total: 75 tests, all passing**
- ✅ Manual verification:
  - All three example programs roundtrip correctly
  - Binary → disassemble → assemble → binary produces identical output
- ✅ Code quality:
  - Clippy passes with `-D warnings`
  - rustfmt compliant
  - No unsafe code
  - Comprehensive documentation

**Commits**:
- `c96564f` - Implement disassembler and fix S1.14 encoding edge cases
- `fdd13b7` - Add property tests for disassembler roundtrip
- `427d50d` - Fix clippy warnings

#### Phase 2: Milestone 2.2 - Type-Safe DSL (Week 8-9)
**Status**: ✅ COMPLETE  
**Completed**: 2026-02-03

**Deliverables**:
- ✅ Type-safe builder with phantom types (`fv1-dsl/src/typed.rs`):
  - `TypedBuilder<State>` generic builder with state tracking
  - Phantom type markers: `Audio`, `Control`, `Lfo`
  - State transitions enforced at compile-time
- ✅ Comprehensive instruction set coverage:
  - Audio operations: rdax, wrax, mulx, sof, rda, wra, wrap, rmpa, ldax
  - Mathematical operations: absa, exp, log
  - Bitwise operations: and, or, xor
  - Control operations: clr, nop
  - All operations maintain or transition accumulator state appropriately
- ✅ Type system guarantees:
  - `rdax` and `clr` transition to `Audio` state
  - Audio state operations maintain state consistency
  - `nop` preserves current state
  - Cannot call audio-only operations from non-audio states
- ✅ Integration with existing DSL:
  - Works alongside `ProgramBuilder` (untyped builder)
  - Compatible with `fv1_program!` macro
  - Exports through `prelude` module
- ✅ Comprehensive testing:
  - 10 unit tests in `typed.rs`
  - 9 integration tests demonstrating type-safe usage
  - Tests for basic programs, gain control, delay echo, complex chains
  - All existing tests still passing (39 total tests)
- ✅ Documentation:
  - Detailed doc comments on `TypedBuilder` and all methods
  - Usage examples in doc tests
  - Integration test examples demonstrate real-world patterns
- ✅ Code quality:
  - Clippy compliant
  - rustfmt compliant
  - No unsafe code
  - Zero-cost abstraction (phantom types are compile-time only)

**Commits**:
- TBD - Implement Milestone 2.2: Type-Safe DSL with phantom types

### 🚧 In Progress

*No active work items*

### 📋 Next Up

*See detailed phase plans below for upcoming milestones*

-----

## PHASE 1: Foundation - Rust Assembler (Option B)

### Milestone 1.1: Project Setup & Core Types (Week 1)

**Goal**: Establish project structure and fundamental data types.

#### 1.1.1 Workspace Structure

```
fv1-rs/
├── Cargo.toml                 # Workspace manifest
├── crates/
│   ├── fv1-asm/              # Core assembler library
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── instruction.rs
│   │   │   ├── register.rs
│   │   │   └── constants.rs
│   │   └── tests/
│   ├── fv1-cli/              # Command-line tool
│   │   ├── Cargo.toml
│   │   └── src/
│   └── fv1-examples/         # Example programs
└── docs/
```

#### 1.1.2 Core Type Definitions

**File: `fv1-asm/src/register.rs`**

```rust
/// FV-1 Registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    // Accumulator (implied in most operations)
    ACC,
    
    // Audio I/O
    ADCL,  // Left ADC input
    ADCR,  // Right ADC input
    DACL,  // Left DAC output
    DACR,  // Right DAC output
    
    // General purpose registers (32 total)
    REG(u8),  // REG0-REG31
    
    // Special registers
    ADDR_PTR,  // Address pointer for RMPA
    LR,        // Low-pass/Ramp register (some variants)
    
    // Delay RAM address
    SIN0_RATE,
    SIN0_RANGE,
    SIN1_RATE,
    SIN1_RANGE,
    RMP0_RATE,
    RMP0_RANGE,
    RMP1_RATE,
    RMP1_RANGE,
}

/// Control inputs (POT0-POT2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    POT0,
    POT1,
    POT2,
}

/// LFO oscillators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lfo {
    SIN0,
    SIN1,
    RMP0,
    RMP1,
}
```

**File: `fv1-asm/src/instruction.rs`**

```rust
/// FV-1 Instruction Set
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Accumulator operations
    /// Read register and add to ACC: ACC = ACC * C + [REG] * D
    RDAX { reg: Register, coeff: f32 },
    
    /// Read delay RAM: ACC = ACC * C + [ADDR] * D
    RDA { addr: u16, coeff: f32 },
    
    /// Read delay RAM with LFO: ACC = ACC * C + [ADDR + LFO] * D
    RMPA { coeff: f32 },
    
    /// Write ACC to register: [REG] = ACC * C, ACC = ACC * D
    WRAX { reg: Register, coeff: f32 },
    
    /// Write ACC to delay RAM: [ADDR] = ACC * C, ACC = ACC * D
    WRA { addr: u16, coeff: f32 },
    
    /// Write ACC with crossfade: [ADDR] = ACC * C + [ADDR] * D
    WRAP { addr: u16, coeff: f32 },
    
    // Mathematical operations
    /// Multiply ACC by register: ACC = ACC * [REG]
    MULX { reg: Register },
    
    /// Reverse multiply: ACC = [REG] - ACC * [REG]
    RDFX { reg: Register, coeff: f32 },
    
    /// Absolute value: ACC = |ACC| * C
    ABSA,
    
    /// Load immediate: ACC = C
    LDAX { reg: Register },
    
    // Filtering
    /// Single-pole lowpass: ACC = C * ACC + (1-C) * [REG]
    RDFX2 { reg: Register, coeff: f32 },
    
    // Logic and control
    /// Set accumulator to S
    SOF { coeff: f32, offset: f32 },  // ACC = ACC * C + D
    
    /// AND with mask
    AND { mask: u32 },
    
    /// OR with mask
    OR { mask: u32 },
    
    /// XOR with mask
    XOR { mask: u32 },
    
    /// Shift left
    SHL,
    
    /// Shift right
    SHR,
    
    /// Clear ACC
    CLR,
    
    /// No operation
    NOP,
    
    /// Exponential conversion
    EXP { coeff: f32, offset: f32 },
    
    /// Logarithmic conversion
    LOG { coeff: f32, offset: f32 },
    
    // Conditional skipping
    /// Skip next instruction if ACC >= 0
    SKP { condition: SkipCondition, offset: i8 },
    
    // LFO control
    /// Write LFO frequency
    WLDS { lfo: Lfo, freq: u16, amplitude: u16 },
    
    // Jump/Call (if supported in variant)
    JAM { lfo: Lfo },
    
    // Delay RAM addressing
    CHO { mode: ChoMode, lfo: Lfo, flags: ChoFlags, addr: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipCondition {
    GEZ,  // Greater or equal to zero
    NEG,  // Negative
    ZRC,  // Zero crossing
    ZRO,  // Zero
    RUN,  // Always run
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoMode {
    RDA,  // Read delay with LFO
    SOF,  // Scale/offset with LFO
    RDAL, // Read delay and load LFO value
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChoFlags {
    pub rptr2: bool,      // Use second read pointer
    pub na: bool,         // No add (crossfade control)
    pub compc: bool,      // Complement coefficient
    pub compa: bool,      // Complement address
    pub rptr2_select: bool,
}

/// Binary encoding of an instruction (32 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodedInstruction(pub u32);

impl Instruction {
    /// Encode instruction to FV-1 binary format
    pub fn encode(&self) -> EncodedInstruction {
        // To be implemented
        todo!()
    }
    
    /// Decode from binary
    pub fn decode(encoded: EncodedInstruction) -> Result<Self, DecodeError> {
        // To be implemented
        todo!()
    }
}
```

**File: `fv1-asm/src/constants.rs`**

```rust
/// FV-1 Hardware Constants
pub const MAX_INSTRUCTIONS: usize = 128;
pub const DELAY_RAM_SIZE: usize = 32768;  // 32K samples
pub const NUM_REGISTERS: usize = 32;
pub const NUM_PROGRAMS: usize = 8;  // In EEPROM

/// Coefficient encoding (S.23 fixed point)
pub const COEFF_SCALE: f32 = 8388608.0;  // 2^23

/// Address range
pub const MAX_DELAY_ADDR: u16 = 32767;

/// Coefficient helper
pub fn encode_coeff(value: f32) -> i32 {
    (value * COEFF_SCALE).round() as i32
}

pub fn decode_coeff(encoded: i32) -> f32 {
    encoded as f32 / COEFF_SCALE
}
```

#### 1.1.3 Dependencies

```toml
# fv1-asm/Cargo.toml
[dependencies]
thiserror = "1.0"
logos = "0.13"           # Lexer generator
chumsky = "0.9"          # Parser combinator (alternative: nom)
miette = "5.0"           # Beautiful error reporting
```

-----

### Milestone 1.2: Lexer Implementation (Week 1-2)

**Goal**: Tokenize FV-1 assembly source files.

#### 1.2.1 Token Definitions

**File: `fv1-asm/src/lexer.rs`**

```rust
use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]  // Skip whitespace
#[logos(skip r";[^\n]*")]      // Skip comments
pub enum Token {
    // Instructions
    #[token("rdax", ignore(ascii_case))]
    RDAX,
    #[token("rda", ignore(ascii_case))]
    RDA,
    #[token("wrax", ignore(ascii_case))]
    WRAX,
    #[token("wra", ignore(ascii_case))]
    WRA,
    #[token("mulx", ignore(ascii_case))]
    MULX,
    #[token("sof", ignore(ascii_case))]
    SOF,
    #[token("clr", ignore(ascii_case))]
    CLR,
    // ... (add all instructions)
    
    // Registers
    #[token("adcl", ignore(ascii_case))]
    ADCL,
    #[token("adcr", ignore(ascii_case))]
    ADCR,
    #[token("dacl", ignore(ascii_case))]
    DACL,
    #[token("dacr", ignore(ascii_case))]
    DACR,
    #[regex(r"(?i)reg([0-9]|[12][0-9]|3[01])")]
    REG,
    #[regex(r"(?i)pot[0-2]")]
    POT,
    
    // LFOs
    #[token("sin0", ignore(ascii_case))]
    SIN0,
    #[token("sin1", ignore(ascii_case))]
    SIN1,
    #[token("rmp0", ignore(ascii_case))]
    RMP0,
    #[token("rmp1", ignore(ascii_case))]
    RMP1,
    
    // Literals
    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f32>().ok())]
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<f32>().ok())]
    Float(f32),
    
    #[regex(r"0x[0-9a-fA-F]+", |lex| i64::from_str_radix(&lex.slice()[2..], 16).ok())]
    #[regex(r"\$[0-9a-fA-F]+", |lex| i64::from_str_radix(&lex.slice()[1..], 16).ok())]
    #[regex(r"%[01]+", |lex| i64::from_str_radix(&lex.slice()[1..], 2).ok())]
    Integer(i64),
    
    // Identifiers (labels, equates)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    
    // Operators
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("=")]
    Equals,
    
    // Directives
    #[token("equ", ignore(ascii_case))]
    EQU,
    #[token("mem", ignore(ascii_case))]
    MEM,
    
    // Special
    #[token("#")]
    Hash,
}

pub struct Lexer<'source> {
    inner: logos::Lexer<'source, Token>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            inner: Token::lexer(source),
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = (Token, std::ops::Range<usize>);
    
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.inner.next()?;
        let span = self.inner.span();
        Some((token.ok()?, span))
    }
}
```

#### 1.2.2 Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let source = "rdax adcl, 0.5";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next().unwrap().0, Token::RDAX);
        assert_eq!(lexer.next().unwrap().0, Token::ADCL);
        assert_eq!(lexer.next().unwrap().0, Token::Comma);
        assert_eq!(lexer.next().unwrap().0, Token::Float(0.5));
    }
    
    #[test]
    fn test_comments() {
        let source = r#"
            rdax adcl, 0.5  ; read left input
            ; full line comment
            sof 0, 0        ; clear
        "#;
        
        let tokens: Vec<_> = Lexer::new(source).collect();
        // Should skip comments
        assert_eq!(tokens.len(), 8);
    }
}
```

-----

### Milestone 1.3: Parser & AST (Week 2-3)

**Goal**: Parse token stream into Abstract Syntax Tree.

#### 1.3.1 AST Definitions

**File: `fv1-asm/src/ast.rs`**

```rust
use crate::instruction::Instruction;
use std::collections::HashMap;

/// Complete FV-1 program
#[derive(Debug, Clone)]
pub struct Program {
    pub directives: Vec<Directive>,
    pub statements: Vec<Statement>,
    pub labels: HashMap<String, usize>,  // label -> instruction index
}

#[derive(Debug, Clone)]
pub enum Directive {
    /// EQU name, value
    Equate { name: String, value: Value },
    
    /// MEM name size
    MemoryAllocation { name: String, size: u16 },
    
    /// SPINASM compatibility
    SpinAsm { version: String },
}

#[derive(Debug, Clone)]
pub enum Statement {
    /// Label:
    Label(String),
    
    /// Instruction
    Instruction(Instruction),
    
    /// Labeled instruction
    LabeledInstruction {
        label: String,
        instruction: Instruction,
    },
}

#[derive(Debug, Clone)]
pub enum Value {
    Float(f32),
    Integer(i64),
    Identifier(String),  // Reference to equate
}

impl Program {
    pub fn new() -> Self {
        Self {
            directives: Vec::new(),
            statements: Vec::new(),
            labels: HashMap::new(),
        }
    }
    
    /// Get all instructions in order
    pub fn instructions(&self) -> Vec<&Instruction> {
        self.statements.iter()
            .filter_map(|s| match s {
                Statement::Instruction(i) => Some(i),
                Statement::LabeledInstruction { instruction, .. } => Some(instruction),
                _ => None,
            })
            .collect()
    }
    
    /// Resolve label to instruction index
    pub fn resolve_label(&self, label: &str) -> Option<usize> {
        self.labels.get(label).copied()
    }
}
```

#### 1.3.2 Parser Implementation

**File: `fv1-asm/src/parser.rs`**

```rust
use crate::{
    ast::*,
    instruction::*,
    lexer::{Token, Lexer},
    register::*,
    error::ParseError,
};

pub struct Parser<'source> {
    tokens: Vec<(Token, std::ops::Range<usize>)>,
    pos: usize,
    source: &'source str,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        let tokens: Vec<_> = Lexer::new(source).collect();
        Self {
            tokens,
            pos: 0,
            source,
        }
    }
    
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::new();
        
        while !self.is_at_end() {
            // Parse directive or statement
            if self.check_directive() {
                program.directives.push(self.parse_directive()?);
            } else {
                program.statements.push(self.parse_statement()?);
            }
        }
        
        // Second pass: resolve labels
        self.resolve_labels(&mut program)?;
        
        Ok(program)
    }
    
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        // Check for label
        if let Some((Token::Identifier(name), _)) = self.peek() {
            if matches!(self.peek_next(), Some((Token::Colon, _))) {
                let label = name.clone();
                self.advance(); // consume identifier
                self.advance(); // consume colon
                
                // Check if instruction follows
                if !self.is_at_end() && self.is_instruction() {
                    let inst = self.parse_instruction()?;
                    return Ok(Statement::LabeledInstruction {
                        label,
                        instruction: inst,
                    });
                } else {
                    return Ok(Statement::Label(label));
                }
            }
        }
        
        // Parse instruction
        let inst = self.parse_instruction()?;
        Ok(Statement::Instruction(inst))
    }
    
    fn parse_instruction(&mut self) -> Result<Instruction, ParseError> {
        let (token, span) = self.advance()
            .ok_or_else(|| ParseError::UnexpectedEof)?;
        
        match token {
            Token::RDAX => {
                let reg = self.parse_register()?;
                self.expect(Token::Comma)?;
                let coeff = self.parse_float()?;
                Ok(Instruction::RDAX { reg, coeff })
            }
            
            Token::WRAX => {
                let reg = self.parse_register()?;
                self.expect(Token::Comma)?;
                let coeff = self.parse_float()?;
                Ok(Instruction::WRAX { reg, coeff })
            }
            
            Token::SOF => {
                let coeff = self.parse_float()?;
                self.expect(Token::Comma)?;
                let offset = self.parse_float()?;
                Ok(Instruction::SOF { coeff, offset })
            }
            
            Token::MULX => {
                let reg = self.parse_register()?;
                Ok(Instruction::MULX { reg })
            }
            
            Token::CLR => Ok(Instruction::CLR),
            
            // ... implement all other instructions
            
            _ => Err(ParseError::UnexpectedToken {
                expected: "instruction".to_string(),
                found: format!("{:?}", token),
                span,
            }),
        }
    }
    
    fn parse_register(&mut self) -> Result<Register, ParseError> {
        let (token, span) = self.advance()
            .ok_or_else(|| ParseError::UnexpectedEof)?;
        
        match token {
            Token::ADCL => Ok(Register::ADCL),
            Token::ADCR => Ok(Register::ADCR),
            Token::DACL => Ok(Register::DACL),
            Token::DACR => Ok(Register::DACR),
            Token::REG => {
                // Extract register number from token
                // This requires looking back at the source
                todo!("Parse REG number")
            }
            _ => Err(ParseError::ExpectedRegister { span }),
        }
    }
    
    fn parse_float(&mut self) -> Result<f32, ParseError> {
        let (token, span) = self.advance()
            .ok_or_else(|| ParseError::UnexpectedEof)?;
        
        match token {
            Token::Float(f) => Ok(*f),
            Token::Integer(i) => Ok(*i as f32),
            _ => Err(ParseError::ExpectedNumber { span }),
        }
    }
    
    // Helper methods
    fn peek(&self) -> Option<&(Token, std::ops::Range<usize>)> {
        self.tokens.get(self.pos)
    }
    
    fn peek_next(&self) -> Option<&(Token, std::ops::Range<usize>)> {
        self.tokens.get(self.pos + 1)
    }
    
    fn advance(&mut self) -> Option<&(Token, std::ops::Range<usize>)> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }
    
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let (token, span) = self.advance()
            .ok_or_else(|| ParseError::UnexpectedEof)?;
        
        if std::mem::discriminant(token) == std::mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", token),
                span: span.clone(),
            })
        }
    }
    
    fn is_instruction(&self) -> bool {
        matches!(
            self.peek().map(|(t, _)| t),
            Some(Token::RDAX | Token::WRAX | Token::SOF | Token::CLR | /* ... */)
        )
    }
    
    fn resolve_labels(&self, program: &mut Program) -> Result<(), ParseError> {
        let mut index = 0;
        for stmt in &program.statements {
            match stmt {
                Statement::Label(name) |
                Statement::LabeledInstruction { label: name, .. } => {
                    program.labels.insert(name.clone(), index);
                }
                _ => {}
            }
            if matches!(stmt, Statement::Instruction(_) | Statement::LabeledInstruction { .. }) {
                index += 1;
            }
        }
        Ok(())
    }
    
    fn check_directive(&self) -> bool {
        matches!(self.peek().map(|(t, _)| t), Some(Token::EQU | Token::MEM))
    }
    
    fn parse_directive(&mut self) -> Result<Directive, ParseError> {
        todo!()
    }
}
```

#### 1.3.3 Error Handling

**File: `fv1-asm/src/error.rs`**

```rust
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ParseError {
    #[error("Unexpected end of file")]
    UnexpectedEof,
    
    #[error("Unexpected token: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: String,
        #[label("here")]
        span: std::ops::Range<usize>,
    },
    
    #[error("Expected register")]
    ExpectedRegister {
        #[label("expected register here")]
        span: std::ops::Range<usize>,
    },
    
    #[error("Expected number")]
    ExpectedNumber {
        #[label("expected number here")]
        span: std::ops::Range<usize>,
    },
    
    #[error("Too many instructions (max {max})")]
    TooManyInstructions {
        max: usize,
        #[label("instruction {count}")]
        span: std::ops::Range<usize>,
        count: usize,
    },
    
    #[error("Undefined label: {name}")]
    UndefinedLabel {
        name: String,
        #[label("label used here")]
        span: std::ops::Range<usize>,
    },
}

#[derive(Error, Debug, Diagnostic)]
pub enum CodegenError {
    #[error("Coefficient out of range: {value}")]
    CoefficientOutOfRange {
        value: f32,
    },
    
    #[error("Address out of range: {addr} (max: {max})")]
    AddressOutOfRange {
        addr: u16,
        max: u16,
    },
    
    #[error("Program too large: {size} instructions (max: {max})")]
    ProgramTooLarge {
        size: usize,
        max: usize,
    },
}
```

-----

### Milestone 1.4: Code Generation (Week 3-4)

**Goal**: Generate FV-1 binary from AST.

#### 1.4.1 Instruction Encoding

**File: `fv1-asm/src/codegen/encoder.rs`**

```rust
use crate::{
    instruction::*,
    register::*,
    constants::*,
    error::CodegenError,
};

/// Encode a single instruction to 32-bit binary
pub fn encode_instruction(inst: &Instruction) -> Result<u32, CodegenError> {
    match inst {
        Instruction::RDAX { reg, coeff } => {
            let opcode = 0b00000 << 27;
            let reg_bits = encode_register(reg)? << 21;
            let coeff_bits = encode_s114(*coeff)? & 0x7FFF;
            Ok(opcode | reg_bits | coeff_bits)
        }
        
        Instruction::WRAX { reg, coeff } => {
            let opcode = 0b00110 << 27;
            let reg_bits = encode_register(reg)? << 21;
            let coeff_bits = encode_s114(*coeff)? & 0x7FFF;
            Ok(opcode | reg_bits | coeff_bits)
        }
        
        Instruction::SOF { coeff, offset } => {
            let opcode = 0b01101 << 27;
            let c_bits = encode_s114(*coeff)? << 11;
            let d_bits = encode_s10(*offset)? & 0x7FF;
            Ok(opcode | c_bits | d_bits)
        }
        
        Instruction::MULX { reg } => {
            let opcode = 0b01010 << 27;
            let reg_bits = encode_register(reg)? << 21;
            Ok(opcode | reg_bits)
        }
        
        Instruction::CLR => {
            Ok(0b01110 << 27)
        }
        
        // ... implement all other instructions
        
        _ => todo!("Implement encoding for {:?}", inst),
    }
}

/// Encode register to 5-bit field
fn encode_register(reg: &Register) -> Result<u32, CodegenError> {
    match reg {
        Register::ADCL => Ok(0b00000),
        Register::ADCR => Ok(0b00001),
        Register::DACL => Ok(0b00010),
        Register::DACR => Ok(0b00011),
        Register::REG(n) if *n < 32 => Ok(*n as u32),
        Register::POT0 => Ok(0b00100),
        Register::POT1 => Ok(0b00101),
        Register::POT2 => Ok(0b00110),
        _ => Err(CodegenError::InvalidRegister),
    }
}

/// Encode S1.14 fixed-point coefficient (-2.0 to ~2.0)
fn encode_s114(value: f32) -> Result<u32, CodegenError> {
    if value < -2.0 || value >= 2.0 {
        return Err(CodegenError::CoefficientOutOfRange { value });
    }
    
    // Convert to S1.14: sign bit + 14 fractional bits
    let scaled = (value * 16384.0).round() as i32;
    Ok((scaled & 0x7FFF) as u32)
}

/// Encode S.10 fixed-point coefficient
fn encode_s10(value: f32) -> Result<u32, CodegenError> {
    if value < -1.0 || value >= 1.0 {
        return Err(CodegenError::CoefficientOutOfRange { value });
    }
    
    let scaled = (value * 512.0).round() as i32;
    Ok((scaled & 0x3FF) as u32)
}

/// Encode 14-bit delay address
fn encode_address(addr: u16) -> Result<u32, CodegenError> {
    if addr > MAX_DELAY_ADDR {
        return Err(CodegenError::AddressOutOfRange {
            addr,
            max: MAX_DELAY_ADDR,
        });
    }
    Ok(addr as u32 & 0x3FFF)
}
```

#### 1.4.2 Program Assembly

**File: `fv1-asm/src/codegen/assembler.rs`**

```rust
use crate::{
    ast::Program,
    codegen::encoder::encode_instruction,
    error::CodegenError,
    constants::MAX_INSTRUCTIONS,
};

pub struct Assembler {
    optimize: bool,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            optimize: false,
        }
    }
    
    pub fn with_optimization(mut self, enable: bool) -> Self {
        self.optimize = enable;
        self
    }
    
    pub fn assemble(&self, program: &Program) -> Result<Binary, CodegenError> {
        let instructions = program.instructions();
        
        if instructions.len() > MAX_INSTRUCTIONS {
            return Err(CodegenError::ProgramTooLarge {
                size: instructions.len(),
                max: MAX_INSTRUCTIONS,
            });
        }
        
        let mut binary = Binary::new();
        
        for inst in instructions {
            let encoded = encode_instruction(inst)?;
            binary.push(encoded);
        }
        
        // Pad to 128 instructions if needed
        while binary.len() < MAX_INSTRUCTIONS {
            binary.push(0); // NOP
        }
        
        if self.optimize {
            binary = self.optimize_binary(binary)?;
        }
        
        Ok(binary)
    }
    
    fn optimize_binary(&self, binary: Binary) -> Result<Binary, CodegenError> {
        // TODO: Implement peephole optimizations
        // - Remove redundant CLR
        // - Combine SOF operations
        // - Dead code elimination
        Ok(binary)
    }
}

/// Binary program (128 x 32-bit instructions)
#[derive(Debug, Clone)]
pub struct Binary {
    instructions: Vec<u32>,
}

impl Binary {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
    
    pub fn push(&mut self, instruction: u32) {
        self.instructions.push(instruction);
    }
    
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
    
    /// Export as raw binary (512 bytes)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(512);
        for &inst in &self.instructions {
            bytes.extend_from_slice(&inst.to_be_bytes());
        }
        bytes
    }
    
    /// Export as Intel HEX format
    pub fn to_hex(&self) -> String {
        let mut hex = String::new();
        let bytes = self.to_bytes();
        
        for (i, chunk) in bytes.chunks(16).enumerate() {
            let addr = i * 16;
            let len = chunk.len();
            
            // Record header: :LLAAAATT
            hex.push_str(&format!(":{:02X}{:04X}00", len, addr));
            
            // Data bytes
            let mut checksum = len + (addr >> 8) + (addr & 0xFF);
            for &byte in chunk {
                hex.push_str(&format!("{:02X}", byte));
                checksum += byte as usize;
            }
            
            // Checksum
            checksum = (256 - (checksum & 0xFF)) & 0xFF;
            hex.push_str(&format!("{:02X}\n", checksum));
        }
        
        // End of file record
        hex.push_str(":00000001FF\n");
        hex
    }
    
    /// Export as C array
    pub fn to_c_array(&self, name: &str) -> String {
        let mut c = format!("const uint32_t {}[128] = {{\n", name);
        
        for (i, &inst) in self.instructions.iter().enumerate() {
            if i % 4 == 0 {
                c.push_str("    ");
            }
            c.push_str(&format!("0x{:08X}", inst));
            if i < self.instructions.len() - 1 {
                c.push_str(", ");
            }
            if i % 4 == 3 {
                c.push('\n');
            }
        }
        
        c.push_str("};\n");
        c
    }
}
```

-----

### Milestone 1.5: CLI Tool (Week 4-5)

**Goal**: Command-line interface for assembling programs.

#### 1.5.1 CLI Structure

**File: `fv1-cli/Cargo.toml`**

```toml
[dependencies]
fv1-asm = { path = "../fv1-asm" }
clap = { version = "4.0", features = ["derive"] }
miette = { version = "5.0", features = ["fancy"] }
color-eyre = "0.6"
```

**File: `fv1-cli/src/main.rs`**

```rust
use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;
use fv1_asm::{Parser as AsmParser, Assembler};

#[derive(Parser)]
#[command(name = "fv1")]
#[command(about = "FV-1 DSP Assembler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Assemble a .spn file to binary
    Assemble {
        /// Input assembly file
        input: PathBuf,
        
        /// Output file (defaults to input with .bin extension)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Output format
        #[arg(short, long, default_value = "bin")]
        format: OutputFormat,
        
        /// Enable optimizations
        #[arg(short = 'O', long)]
        optimize: bool,
        
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Disassemble a binary file
    Disassemble {
        /// Input binary file
        input: PathBuf,
        
        /// Output assembly file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Validate an assembly file without generating output
    Check {
        /// Input assembly file
        input: PathBuf,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Bin,
    Hex,
    CArray,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bin" | "binary" => Ok(Self::Bin),
            "hex" | "ihex" => Ok(Self::Hex),
            "c" | "array" => Ok(Self::CArray),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

fn main() -> Result<()> {
    miette::set_panic_hook();
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Assemble { input, output, format, optimize, verbose } => {
            assemble_file(input, output, format, optimize, verbose)?;
        }
        Commands::Disassemble { input, output } => {
            disassemble_file(input, output)?;
        }
        Commands::Check { input } => {
            check_file(input)?;
        }
    }
    
    Ok(())
}

fn assemble_file(
    input: PathBuf,
    output: Option<PathBuf>,
    format: OutputFormat,
    optimize: bool,
    verbose: bool,
) -> Result<()> {
    // Read source
    let source = std::fs::read_to_string(&input)
        .into_diagnostic()?;
    
    if verbose {
        println!("Parsing {}...", input.display());
    }
    
    // Parse
    let mut parser = AsmParser::new(&source);
    let program = parser.parse()
        .map_err(|e| miette::Report::new(e)
            .with_source_code(source.clone()))?;
    
    if verbose {
        println!("  {} instructions", program.instructions().len());
        println!("  {} labels", program.labels.len());
    }
    
    // Assemble
    let assembler = Assembler::new()
        .with_optimization(optimize);
    
    let binary = assembler.assemble(&program)?;
    
    // Determine output path
    let output = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension(match format {
            OutputFormat::Bin => "bin",
            OutputFormat::Hex => "hex",
            OutputFormat::CArray => "c",
        });
        path
    });
    
    // Write output
    if verbose {
        println!("Writing {}...", output.display());
    }
    
    match format {
        OutputFormat::Bin => {
            std::fs::write(&output, binary.to_bytes())
                .into_diagnostic()?;
        }
        OutputFormat::Hex => {
            std::fs::write(&output, binary.to_hex())
                .into_diagnostic()?;
        }
        OutputFormat::CArray => {
            let name = input.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("program");
            std::fs::write(&output, binary.to_c_array(name))
                .into_diagnostic()?;
        }
    }
    
    println!("✓ Successfully assembled to {}", output.display());
    
    Ok(())
}

fn disassemble_file(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    // TODO: Implement disassembler
    todo!()
}

fn check_file(input: PathBuf) -> Result<()> {
    let source = std::fs::read_to_string(&input)
        .into_diagnostic()?;
    
    let mut parser = AsmParser::new(&source);
    let program = parser.parse()
        .map_err(|e| miette::Report::new(e)
            .with_source_code(source.clone()))?;
    
    println!("✓ {} is valid", input.display());
    println!("  {} instructions", program.instructions().len());
    println!("  {} labels", program.labels.len());
    
    Ok(())
}
```

-----

### Milestone 1.6: Testing & Documentation (Week 5-6)

#### 1.6.1 Test Suite

**File: `fv1-asm/tests/integration_tests.rs`**

```rust
use fv1_asm::{Parser, Assembler};

#[test]
fn test_simple_passthrough() {
    let source = r#"
        rdax adcl, 1.0
        wrax dacl, 0.0
    "#;
    
    let mut parser = Parser::new(source);
    let program = parser.parse().unwrap();
    let binary = Assembler::new().assemble(&program).unwrap();
    
    assert_eq!(binary.len(), 128);
}

#[test]
fn test_with_labels() {
    let source = r#"
        rdax adcl, 0.5
        mulx pot0
    loop:
        wrax dacl, 1.0
        skp gez, loop
    "#;
    
    let mut parser = Parser::new(source);
    let program = parser.parse().unwrap();
    
    assert!(program.labels.contains_key("loop"));
}

#[test]
fn test_coefficient_range() {
    let source = r#"
        sof 2.5, 0.0  ; Out of range
    "#;
    
    let mut parser = Parser::new(source);
    let program = parser.parse().unwrap();
    let result = Assembler::new().assemble(&program);
    
    assert!(result.is_err());
}
```

#### 1.6.2 Example Programs

Create `fv1-examples/` directory with standard effects:

- `passthrough.spn` - Simple audio passthrough
- `gain.spn` - Volume control with pot
- `delay.spn` - Simple delay line
- `reverb.spn` - Basic reverb algorithm
- `tremolo.spn` - Amplitude modulation with LFO

#### 1.6.3 Documentation

**README.md** with:

- Installation instructions
- Quick start guide
- CLI usage examples
- API documentation link

**docs/fv1-instruction-reference.md** - Complete instruction set reference

-----

## PHASE 2: Rust DSL (Option A)

### Milestone 2.1: Macro Foundation (Week 7-8)
**Status**: ✅ COMPLETE  
**Completed**: 2026-02-03

**Goal**: Create procedural macros for writing FV-1 programs in Rust.

#### 2.1.1 Project Structure

```
fv1-rs/
├── crates/
│   ├── fv1-dsl/              # DSL crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── ops.rs
│   └── fv1-dsl-macro/        # Proc macro crate
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
```

#### 2.1.2 Core DSL Types

**File: `fv1-dsl/src/lib.rs`**

```rust
pub use fv1_dsl_macro::fv1_program;
pub use fv1_asm::{Register, Instruction};

/// Builder for FV-1 programs using Rust API
pub struct ProgramBuilder {
    instructions: Vec<Instruction>,
    labels: std::collections::HashMap<String, usize>,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: std::collections::HashMap::new(),
        }
    }
    
    /// Add instruction
    pub fn inst(&mut self, inst: Instruction) -> &mut Self {
        self.instructions.push(inst);
        self
    }
    
    /// Add label at current position
    pub fn label(&mut self, name: impl Into<String>) -> &mut Self {
        self.labels.insert(name.into(), self.instructions.len());
        self
    }
    
    /// Build final program
    pub fn build(self) -> fv1_asm::ast::Program {
        todo!()
    }
}

// Fluent API for common operations
pub mod ops {
    use super::*;
    use fv1_asm::{Register, Instruction};
    
    pub fn rdax(reg: Register, coeff: f32) -> Instruction {
        Instruction::RDAX { reg, coeff }
    }
    
    pub fn wrax(reg: Register, coeff: f32) -> Instruction {
        Instruction::WRAX { reg, coeff }
    }
    
    pub fn sof(coeff: f32, offset: f32) -> Instruction {
        Instruction::SOF { coeff, offset }
    }
    
    pub fn mulx(reg: Register) -> Instruction {
        Instruction::MULX { reg }
    }
    
    pub fn clr() -> Instruction {
        Instruction::CLR
    }
    
    // ... all other instructions
}
```

#### 2.1.3 Procedural Macro

**File: `fv1-dsl-macro/Cargo.toml`**

```toml
[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full"] }
quote = "1.0"
proc-macro2 = "1.0"
```

**File: `fv1-dsl-macro/src/lib.rs`**

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprCall, ExprPath};

#[proc_macro]
pub fn fv1_program(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    
    // Parse sequence of instruction calls
    let instructions = parse_program(&expr);
    
    let output = quote! {
        {
            let mut builder = ::fv1_dsl::ProgramBuilder::new();
            #(builder.inst(#instructions);)*
            builder.build()
        }
    };
    
    output.into()
}

fn parse_program(expr: &Expr) -> Vec<Expr> {
    // Parse a block of expressions as instructions
    match expr {
        Expr::Block(block) => {
            block.block.stmts.iter()
                .filter_map(|stmt| {
                    if let syn::Stmt::Expr(e, _) = stmt {
                        Some(e.clone())
                    } else {
                        None
                    }
                })
                .collect()
        }
        _ => vec![expr.clone()],
    }
}
```

#### 2.1.4 Usage Example

```rust
use fv1_dsl::prelude::*;

fn main() {
    let program = fv1_program! {
        rdax(ADCL, 1.0);
        mulx(POT0);
        wrax(DACL, 0.0);
    };
    
    let binary = Assembler::new().assemble(&program).unwrap();
    std::fs::write("output.bin", binary.to_bytes()).unwrap();
}
```

#### Implementation Summary

**Implemented**:
- ✅ Created `fv1-dsl-macro` crate with procedural macro for `fv1_program!`
- ✅ Created `fv1-dsl` crate with `ProgramBuilder` for fluent API
- ✅ Implemented `ops` module with helper functions for all FV-1 instructions
- ✅ Added comprehensive tests for builder API and macro
- ✅ Integrated with existing `fv1-asm` crate for program assembly
- ✅ Both builder pattern (consuming self) and mutable reference APIs available

**Key Features**:
- Ergonomic Rust API for creating FV-1 programs
- Macro syntax closely resembles assembly but with Rust's type safety
- All FV-1 instructions supported through helper functions
- Full integration with existing assembler infrastructure

-----

### Milestone 2.2: Type-Safe DSL (Week 8-9)
**Status**: ✅ COMPLETE  
**Completed**: 2026-02-03

**Goal**: Add type safety and compile-time validation.

#### 2.2.1 Phantom Types for Validation

**File: `fv1-dsl/src/typed.rs`**

```rust
use std::marker::PhantomData;

/// Phantom type representing ACC state
pub struct Acc<T> {
    _phantom: PhantomData<T>,
}

/// Marker: ACC contains audio data
pub struct Audio;

/// Marker: ACC contains control data
pub struct Control;

/// Marker: ACC contains LFO data
pub struct Lfo;

/// Type-safe instruction builder
pub struct TypedBuilder<State> {
    builder: ProgramBuilder,
    _state: PhantomData<State>,
}

impl TypedBuilder<()> {
    pub fn new() -> Self {
        Self {
            builder: ProgramBuilder::new(),
            _state: PhantomData,
        }
    }
}

impl<S> TypedBuilder<S> {
    /// Read from register (transitions to Audio state)
    pub fn rdax(mut self, reg: Register, coeff: f32) -> TypedBuilder<Audio> {
        self.builder.inst(Instruction::RDAX { reg, coeff });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }
}

impl TypedBuilder<Audio> {
    /// Write to register (can output audio)
    pub fn wrax(mut self, reg: Register, coeff: f32) -> TypedBuilder<Audio> {
        self.builder.inst(Instruction::WRAX { reg, coeff });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }
    
    /// Multiply (stays in Audio state)
    pub fn mulx(mut self, reg: Register) -> TypedBuilder<Audio> {
        self.builder.inst(Instruction::MULX { reg });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }
}

impl<S> TypedBuilder<S> {
    pub fn build(self) -> fv1_asm::ast::Program {
        self.builder.build()
    }
}
```

Usage:

```rust
let program = TypedBuilder::new()
    .rdax(ADCL, 1.0)   // Now in Audio state
    .mulx(POT0)        // Still Audio
    .wrax(DACL, 0.0)   // Output audio
    .build();
```

-----

### Milestone 2.3: High-Level Abstractions (Week 9-10)

**Goal**: Provide common DSP building blocks.

**File: `fv1-dsl/src/blocks.rs`**

```rust
/// High-level DSP blocks
pub mod blocks {
    use super::*;
    
    /// Simple gain control
    pub fn gain(input: Register, amount: Register) -> Vec<Instruction> {
        vec![
            rdax(input, 1.0),
            mulx(amount),
        ]
    }
    
    /// Soft clipper
    pub fn soft_clip(threshold: f32) -> Vec<Instruction> {
        vec![
            sof(1.0, 0.0),
            // Implement soft clipping algorithm
        ]
    }
    
    /// One-pole lowpass filter
    pub fn lowpass(input: Register, cutoff: Register, state: Register) -> Vec<Instruction> {
        vec![
            // LP = state + cutoff * (input - state)
            rdax(input, 1.0),
            rdax(state, -1.0),
            mulx(cutoff),
            rdax(state, 1.0),
            wrax(state, 1.0),
        ]
    }
    
    /// Simple delay with feedback
    pub struct Delay {
        buffer: u16,
        length: u16,
    }
    
    impl Delay {
        pub fn new(length: u16) -> Self {
            Self {
                buffer: 0,
                length,
            }
        }
        
        pub fn read(&self, offset: u16) -> Vec<Instruction> {
            vec![
                Instruction::RDA {
                    addr: self.buffer + offset,
                    coeff: 1.0,
                }
            ]
        }
        
        pub fn write(&self, feedback: f32) -> Vec<Instruction> {
            vec![
                Instruction::WRA {
                    addr: self.buffer,
                    coeff: feedback,
                }
            ]
        }
    }
}
```

-----

## PHASE 3: High-Level Framework (Option C)

### Milestone 3.1: Effect Framework (Week 11-12)

**Goal**: Build high-level framework using DSL from Phase 2.

#### 3.1.1 Effect Trait

**File: `fv1-framework/src/lib.rs`**

```rust
use fv1_dsl::*;

/// Base trait for FV-1 effects
pub trait Effect {
    /// Generate the effect program
    fn generate(&self) -> ProgramBuilder;
    
    /// Get effect parameters
    fn parameters(&self) -> Vec<Parameter>;
    
    /// Effect name
    fn name(&self) -> &str;
}

/// Parameter definition
pub struct Parameter {
    pub name: String,
    pub control: Control,  // POT0, POT1, POT2
    pub description: String,
}

/// Builder for combining effects
pub struct EffectChain {
    effects: Vec<Box<dyn Effect>>,
}

impl EffectChain {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
    
    pub fn add(mut self, effect: impl Effect + 'static) -> Self {
        self.effects.push(Box::new(effect));
        self
    }
    
    pub fn build(&self) -> ProgramBuilder {
        let mut builder = ProgramBuilder::new();
        
        // Read input
        builder.inst(rdax(ADCL, 1.0));
        
        // Chain effects
        for effect in &self.effects {
            let effect_prog = effect.generate();
            // Merge effect programs
        }
        
        // Write output
        builder.inst(wrax(DACL, 0.0));
        
        builder
    }
}
```

#### 3.1.2 Standard Effects Library

**File: `fv1-framework/src/effects/mod.rs`**

```rust
pub mod delay;
pub mod reverb;
pub mod modulation;
pub mod filter;
pub mod dynamics;

pub use delay::*;
pub use reverb::*;
pub use modulation::*;
pub use filter::*;
pub use dynamics::*;
```

**File: `fv1-framework/src/effects/delay.rs`**

```rust
use super::*;

pub struct SimpleDelay {
    pub delay_time: DelayTime,
    pub feedback: f32,
    pub mix: f32,
}

pub enum DelayTime {
    Samples(u16),
    Milliseconds(f32),
    Controlled(Control),  // Use POT for delay time
}

impl Effect for SimpleDelay {
    fn name(&self) -> &str {
        "Simple Delay"
    }
    
    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter {
                name: "Delay Time".into(),
                control: Control::POT0,
                description: "Delay time in milliseconds".into(),
            },
            Parameter {
                name: "Feedback".into(),
                control: Control::POT1,
                description: "Amount of feedback (0-100%)".into(),
            },
            Parameter {
                name: "Mix".into(),
                control: Control::POT2,
                description: "Dry/wet mix".into(),
            },
        ]
    }
    
    fn generate(&self) -> ProgramBuilder {
        let mut prog = ProgramBuilder::new();
        
        // Calculate delay address
        let delay_addr = match self.delay_time {
            DelayTime::Samples(s) => s,
            DelayTime::Milliseconds(ms) => {
                (ms * 32.768).round() as u16  // 32.768 kHz sample rate
            }
            DelayTime::Controlled(_) => {
                // Use POT to modulate delay time
                todo!("Implement POT-controlled delay")
            }
        };
        
        // Read input
        prog.inst(rdax(ADCL, 1.0));
        
        // Store dry signal
        prog.inst(wrax(REG(0), self.mix));
        
        // Read from delay
        prog.inst(Instruction::RDA {
            addr: delay_addr,
            coeff: 1.0,
        });
        
        // Add feedback
        prog.inst(sof(self.feedback, 0.0));
        
        // Write to delay
        prog.inst(rdax(ADCL, 1.0));
        prog.inst(Instruction::WRA {
            addr: delay_addr,
            coeff: 0.0,
        });
        
        // Mix with dry
        prog.inst(rdax(REG(0), 1.0 - self.mix));
        
        prog
    }
}
```

**File: `fv1-framework/src/effects/reverb.rs`**

```rust
pub struct PlateReverb {
    pub pre_delay: u16,
    pub decay: f32,
    pub damping: f32,
    pub size: ReverbSize,
}

pub enum ReverbSize {
    Small,
    Medium,
    Large,
}

impl Effect for PlateReverb {
    fn name(&self) -> &str {
        "Plate Reverb"
    }
    
    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter {
                name: "Decay".into(),
                control: Control::POT0,
                description: "Reverb decay time".into(),
            },
            Parameter {
                name: "Damping".into(),
                control: Control::POT1,
                description: "High frequency damping".into(),
            },
            Parameter {
                name: "Mix".into(),
                control: Control::POT2,
                description: "Dry/wet mix".into(),
            },
        ]
    }
    
    fn generate(&self) -> ProgramBuilder {
        let mut prog = ProgramBuilder::new();
        
        // Implement Schroeder reverb structure
        // - 4 parallel comb filters
        // - 2 series allpass filters
        
        // Pre-delay
        prog.inst(rdax(ADCL, 1.0));
        prog.inst(Instruction::WRA { addr: 0, coeff: 0.0 });
        prog.inst(Instruction::RDA { addr: self.pre_delay, coeff: 1.0 });
        
        // Comb filters (parallel)
        let comb_delays = match self.size {
            ReverbSize::Small => [1557, 1617, 1491, 1422],
            ReverbSize::Medium => [2557, 2617, 2491, 2422],
            ReverbSize::Large => [3557, 3617, 3491, 3422],
        };
        
        // ... implement full reverb algorithm
        
        prog
    }
}
```

-----

### Milestone 3.2: Effect Templates & Presets (Week 12-13)

**File: `fv1-framework/src/presets.rs`**

```rust
/// Preset management
pub struct Preset {
    pub name: String,
    pub description: String,
    pub effect: Box<dyn Effect>,
}

pub mod presets {
    use super::*;
    
    pub fn chorus() -> Preset {
        Preset {
            name: "Classic Chorus".into(),
            description: "Warm analog-style chorus".into(),
            effect: Box::new(Chorus {
                depth: 0.5,
                rate: 0.5,
                voices: 2,
            }),
        }
    }
    
    pub fn hall_reverb() -> Preset {
        Preset {
            name: "Concert Hall".into(),
            description: "Large hall reverb".into(),
            effect: Box::new(PlateReverb {
                pre_delay: 100,
                decay: 0.85,
                damping: 0.3,
                size: ReverbSize::Large,
            }),
        }
    }
    
    // ... more presets
}
```

-----

### Milestone 3.3: Visualization & Analysis Tools (Week 13-14)

**File: `fv1-framework/src/analysis.rs`**

```rust
/// Analyze program resource usage
pub struct ResourceAnalysis {
    pub instruction_count: usize,
    pub delay_memory_used: usize,
    pub registers_used: Vec<Register>,
    pub computational_load: f32,  // Approximate
}

impl ResourceAnalysis {
    pub fn analyze(program: &Program) -> Self {
        let instructions = program.instructions();
        
        Self {
            instruction_count: instructions.len(),
            delay_memory_used: Self::calculate_delay_usage(instructions),
            registers_used: Self::find_registers(instructions),
            computational_load: Self::estimate_load(instructions),
        }
    }
    
    fn calculate_delay_usage(instructions: &[&Instruction]) -> usize {
        let mut max_addr = 0;
        for inst in instructions {
            match inst {
                Instruction::RDA { addr, .. } |
                Instruction::WRA { addr, .. } |
                Instruction::WRAP { addr, .. } => {
                    max_addr = max_addr.max(*addr as usize);
                }
                _ => {}
            }
        }
        max_addr
    }
    
    fn find_registers(instructions: &[&Instruction]) -> Vec<Register> {
        // Extract all used registers
        todo!()
    }
    
    fn estimate_load(instructions: &[&Instruction]) -> f32 {
        // Estimate computational complexity
        let mut load = 0.0;
        for inst in instructions {
            load += match inst {
                Instruction::MULX { .. } => 2.0,
                Instruction::EXP { .. } | Instruction::LOG { .. } => 3.0,
                _ => 1.0,
            };
        }
        load / instructions.len() as f32
    }
    
    pub fn report(&self) -> String {
        format!(
            "Resource Usage:\n\
             - Instructions: {}/{}\n\
             - Delay Memory: {} samples\n\
             - Registers: {}\n\
             - Estimated Load: {:.1}%",
            self.instruction_count,
            MAX_INSTRUCTIONS,
            self.delay_memory_used,
            self.registers_used.len(),
            self.computational_load * 100.0
        )
    }
}
```

-----

### Milestone 3.4: Documentation & Examples (Week 14-15)

#### 3.4.1 Comprehensive Examples

**examples/basic_effects.rs**

```rust
use fv1_framework::prelude::*;

fn main() {
    // Simple delay
    let delay = SimpleDelay {
        delay_time: DelayTime::Milliseconds(250.0),
        feedback: 0.5,
        mix: 0.3,
    };
    
    let program = delay.generate().build();
    let binary = Assembler::new().assemble(&program).unwrap();
    std::fs::write("delay.bin", binary.to_bytes()).unwrap();
    
    // Analyze resources
    let analysis = ResourceAnalysis::analyze(&program);
    println!("{}", analysis.report());
}
```

**examples/effect_chain.rs**

```rust
use fv1_framework::prelude::*;

fn main() {
    let chain = EffectChain::new()
        .add(Compressor { threshold: -20.0, ratio: 4.0 })
        .add(Distortion { drive: 0.7, tone: 0.5 })
        .add(SimpleDelay {
            delay_time: DelayTime::Milliseconds(200.0),
            feedback: 0.4,
            mix: 0.2,
        });
    
    let program = chain.build().build();
    // ... assemble and save
}
```

#### 3.4.2 Interactive Tutorial

Create `docs/tutorial/` with step-by-step guides:

1. Getting started with the assembler
1. Writing your first effect in the DSL
1. Using the framework for complex effects
1. Optimizing for the FV-1’s constraints
1. Testing and debugging techniques

-----

## PHASE 4: Advanced Features & Polish (Week 16-20)

### Milestone 4.1: Simulator

**File: `fv1-sim/src/lib.rs`**

```rust
/// Software FV-1 simulator
pub struct Simulator {
    registers: [f32; 64],
    delay_ram: Vec<f32>,
    acc: f32,
    pc: usize,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            registers: [0.0; 64],
            delay_ram: vec![0.0; DELAY_RAM_SIZE],
            acc: 0.0,
            pc: 0,
        }
    }
    
    pub fn load_program(&mut self, binary: &Binary) {
        // Load program into simulator
    }
    
    pub fn process_sample(&mut self, input_l: f32, input_r: f32) -> (f32, f32) {
        self.registers[Register::ADCL as usize] = input_l;
        self.registers[Register::ADCR as usize] = input_r;
        
        // Execute all 128 instructions
        for _ in 0..MAX_INSTRUCTIONS {
            self.step();
        }
        
        (
            self.registers[Register::DACL as usize],
            self.registers[Register::DACR as usize],
        )
    }
    
    fn step(&mut self) {
        // Execute one instruction
        // Decode and execute based on current PC
    }
}
```

### Milestone 4.2: Debugging Tools

- Breakpoints and step-through execution
- Register inspection
- Delay RAM visualization
- Coefficient range checking

### Milestone 4.3: IDE Integration

- VS Code extension for syntax highlighting
- Language server protocol (LSP) implementation
- Live error checking
- Code completion

### Milestone 4.4: Web Playground

- WASM-compiled simulator
- Browser-based editor
- Real-time audio processing demo
- Share and load examples

-----

## Testing Strategy

### Unit Tests

- Each instruction encoding/decoding
- Parser edge cases
- Coefficient range validation
- Register allocation

### Integration Tests

- Complete programs from assembly to binary
- Round-trip (assemble → disassemble → assemble)
- Known good binaries from SpinASM

### Property-Based Tests

- Fuzzing the parser
- Random program generation and assembly
- Coefficient encoding/decoding properties

### Audio Tests

- Compare simulator output to hardware
- Frequency response analysis
- Impulse response comparison

-----

## Documentation Deliverables

1. **API Documentation** - Full rustdoc coverage
1. **User Guide** - Getting started, tutorials, examples
1. **FV-1 Reference** - Instruction set, architecture details
1. **DSL Guide** - Using the Rust DSL effectively
1. **Framework Cookbook** - Common effects and patterns
1. **Contributing Guide** - For open source development

-----

## Success Metrics

**Option B (Assembler) Complete When:**

- ✓ All FV-1 instructions supported
- ✓ Parse real-world .spn files
- ✓ Binary output matches SpinASM
- ✓ Error messages are helpful
- ✓ Cross-platform CLI tool works

**Option A (DSL) Complete When:**

- ✓ Type-safe instruction building
- ✓ Compile-time validation
- ✓ Ergonomic Rust syntax
- ✓ Good error messages at compile time

**Option C (Framework) Complete When:**

- ✓ 10+ standard effects implemented
- ✓ Effect chaining works
- ✓ Resource analysis accurate
- ✓ Preset system functional
- ✓ Example programs work on hardware

-----

## Timeline Summary

|Phase                |Weeks      |Deliverable          |
|---------------------|-----------|---------------------|
|1.1-1.2              |1-2        |Lexer + Types        |
|1.3                  |2-3        |Parser + AST         |
|1.4                  |3-4        |Code Generation      |
|1.5                  |4-5        |CLI Tool             |
|1.6                  |5-6        |Tests + Docs         |
|**Option B Complete**|**Week 6** |**Working Assembler**|
|2.1                  |7-8        |DSL Macros           |
|2.2                  |8-9        |Type Safety          |
|2.3                  |9-10       |Abstractions         |
|**Option A Complete**|**Week 10**|**Rust DSL**         |
|3.1                  |11-12      |Effect Framework     |
|3.2                  |12-13      |Presets              |
|3.3                  |13-14      |Analysis Tools       |
|3.4                  |14-15      |Examples + Docs      |
|**Option C Complete**|**Week 15**|**Full Framework**   |
|4.1-4.4              |16-20      |Advanced Features    |

**Total: ~20 weeks for complete ecosystem**

-----

This plan provides a solid foundation for building a comprehensive Rust toolchain for the FV-1 DSP. Each phase builds on the previous one, creating increasingly powerful abstractions while maintaining the flexibility to work at any level of the stack.
