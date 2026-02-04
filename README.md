# FV-1 Rust

⚠️ This is currently copilot slop ⚠️

A comprehensive Rust ecosystem for programming the Spin Semiconductor FV-1 DSP chip.

## Project Structure

This is a Cargo workspace containing multiple crates:

- **fv1-asm**: Core assembler library with instruction set and types
- **fv1-cli**: Command-line tool for assembling FV-1 programs
- **fv1-examples**: Example programs demonstrating various effects

## Current Status

✅ **Phase 1 - Milestone 1.1**: Project Setup & Core Types (Complete)
✅ **Phase 1 - Milestone 1.2**: Assembler Core (Complete)
✅ **Phase 1 - Milestone 1.3**: Code Generation (Complete)
✅ **Phase 1 - Milestone 1.4**: CLI Tool Enhancement (Complete)
✅ **Phase 2 - Milestone 2.1**: Macro Foundation (Complete)
✅ **Phase 2 - Milestone 2.2**: Type-Safe DSL (Complete)
✅ **Phase 2 - Milestone 2.3**: High-Level Abstractions (Complete)
✅ **Phase 2 - Milestone 2.4**: DSL Examples (Complete)

## Features

- ✅ Full FV-1 instruction set support
- ✅ Lexer and parser for FV-1 assembly syntax
- ✅ Code generation to FV-1 binary format
- ✅ Multiple output formats:
  - Raw binary (.bin)
  - Intel HEX (.hex)
  - C array (.c)
- ✅ Beautiful error reporting with miette
- ✅ Example programs included

## Getting Started

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Using the CLI

The `fv1-cli` tool can assemble FV-1 assembly programs into various output formats.

**Basic usage:**

```bash
# Assemble to binary format (default)
cargo run --bin fv1-cli -- input.asm

# Assemble to Intel HEX format
cargo run --bin fv1-cli -- input.asm --format hex

# Assemble to C array format
cargo run --bin fv1-cli -- input.asm --format c --name my_program

# Specify output file
cargo run --bin fv1-cli -- input.asm -o output.bin

# Verbose output
cargo run --bin fv1-cli -- input.asm --verbose
```

**Try the examples:**

```bash
# Assemble passthrough example
cargo run --bin fv1-cli -- crates/fv1-examples/examples/passthrough.asm --verbose

# Assemble gain control example to hex
cargo run --bin fv1-cli -- crates/fv1-examples/examples/gain_control.asm --format hex

# Assemble delay echo to C array
cargo run --bin fv1-cli -- crates/fv1-examples/examples/delay_echo.asm --format c --name delay_effect
```

### Example Programs

See the `crates/fv1-examples/examples/` directory for example FV-1 programs:

- **passthrough.asm**: Simple audio pass-through
- **gain_control.asm**: Volume control using POT0
- **delay_echo.asm**: Basic delay/echo effect with feedback

## Development Plan

See [PLAN.md](PLAN.md) for the complete development roadmap.

## License

MIT OR Apache-2.0
