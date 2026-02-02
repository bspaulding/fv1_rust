# FV-1 Rust

A comprehensive Rust ecosystem for programming the Spin Semiconductor FV-1 DSP chip.

## Project Structure

This is a Cargo workspace containing multiple crates:

- **fv1-asm**: Core assembler library with instruction set and types
- **fv1-cli**: Command-line tool for assembling FV-1 programs
- **fv1-examples**: Example programs demonstrating various effects

## Current Status

✅ **Phase 1 - Milestone 1.1**: Project Setup & Core Types (In Progress)
- [x] Workspace structure created
- [x] Core type definitions (Register, Control, Lfo)
- [x] Instruction set enum
- [x] Basic tests
- [ ] Assembler core
- [ ] Binary generation

## Getting Started

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the CLI

```bash
cargo run --bin fv1-cli
```

## Development Plan

See [PLAN.md](PLAN.md) for the complete development roadmap.

## License

MIT OR Apache-2.0
