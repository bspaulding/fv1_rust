/// High-level DSP building blocks
///
/// This module provides common DSP building blocks built on top of the low-level
/// instruction helpers. These abstractions make it easier to build complex effects
/// by composing reusable components.
use crate::ops::*;
use crate::{Instruction, Register};

/// Simple gain control
///
/// Reads an input register with unity gain. This is the first step in a gain control chain.
/// Follow with `mulx(amount)` to apply the gain.
///
/// # Arguments
/// * `input` - Register containing the input signal
/// * `_amount` - The register containing the gain amount (for documentation; apply with MULX separately)
///
/// # Example
///
/// ```
/// use fv1_dsl::prelude::*;
/// use fv1_dsl::blocks;
///
/// let program = ProgramBuilder::new()
///     .inst(blocks::gain(Register::ADCL, Register::REG(16))) // POT0
///     .inst(mulx(Register::REG(16)))  // Apply the gain
///     .inst(wrax(Register::DACL, 0.0))
///     .build();
/// ```
pub fn gain(input: Register, _amount: Register) -> Instruction {
    // This is the first instruction in a gain chain: read the input
    // The amount parameter is for documentation; users should follow with mulx(amount)
    rdax(input, 1.0)
}

/// One-pole lowpass filter
///
/// Implements a simple one-pole lowpass filter:
/// `LP = state + cutoff * (input - state)`
///
/// This assumes the input signal is already in ACC. Returns a vector of instructions
/// that should be added to a program. The state register is used to store the filter
/// state between samples.
///
/// # Arguments
/// * `_input` - Should be ACC (for documentation; assumes input is already in accumulator)
/// * `cutoff` - Register containing the cutoff coefficient (0.0 to 1.0)
/// * `state` - Register to store the filter state
///
/// # Example
///
/// ```
/// use fv1_dsl::prelude::*;
/// use fv1_dsl::blocks;
///
/// let mut builder = ProgramBuilder::new();
/// builder.add_inst(rdax(Register::ADCL, 1.0));  // Input now in ACC
/// for inst in blocks::lowpass(Register::ACC, Register::REG(16), Register::REG(1)) {
///     builder.add_inst(inst);
/// }
/// builder.add_inst(wrax(Register::DACL, 0.0));
/// let program = builder.build();
/// ```
pub fn lowpass(_input: Register, cutoff: Register, state: Register) -> Vec<Instruction> {
    vec![
        // LP = state + cutoff * (input - state)
        // Since input is already in ACC, we compute: ACC - state
        rdax(state, -1.0), // ACC = input - state
        mulx(cutoff),      // ACC = cutoff * (input - state)
        rdax(state, 1.0),  // ACC = state + cutoff * (input - state)
        wrax(state, 1.0),  // Store result in state, keep in ACC
    ]
}

/// Soft clipper
///
/// Implements a simple soft clipping algorithm.
/// When the signal exceeds the threshold, it progressively limits the output.
///
/// This is a simplified version that uses ABSA (absolute value) to create
/// a soft clipping effect.
///
/// # Example
///
/// ```
/// use fv1_dsl::prelude::*;
/// use fv1_dsl::blocks;
///
/// let mut builder = ProgramBuilder::new();
/// builder.add_inst(rdax(Register::ADCL, 1.0));
/// for inst in blocks::soft_clip(0.8) {
///     builder.add_inst(inst);
/// }
/// builder.add_inst(wrax(Register::DACL, 0.0));
/// let program = builder.build();
/// ```
pub fn soft_clip(threshold: f32) -> Vec<Instruction> {
    vec![
        // Simple soft clipping using ABSA and SOF
        sof(threshold, 0.0),       // Scale by threshold
        absa(),                    // Take absolute value for symmetrical clipping
        sof(1.0 / threshold, 0.0), // Scale back
    ]
}

/// Simple delay line abstraction
///
/// Provides a higher-level interface for working with delay lines.
/// The delay line uses delay RAM addresses for reading and writing delayed signals.
///
/// # Example
///
/// ```
/// use fv1_dsl::prelude::*;
/// use fv1_dsl::blocks::Delay;
///
/// let delay = Delay::new(0, 4000);
///
/// let mut builder = ProgramBuilder::new();
/// builder.add_inst(rdax(Register::ADCL, 1.0));
/// builder.add_inst(wrax(Register::REG(0), 0.0));
///
/// // Read delayed signal
/// for inst in delay.read(0) {
///     builder.add_inst(inst);
/// }
///
/// // Process and write back with feedback
/// builder.add_inst(mulx(Register::REG(17))); // POT1 for feedback
/// builder.add_inst(rdax(Register::REG(0), 1.0));
///
/// for inst in delay.write(0.0) {
///     builder.add_inst(inst);
/// }
///
/// let program = builder.build();
/// ```
pub struct Delay {
    /// Starting address in delay RAM
    pub buffer: u16,
    /// Length of the delay in samples
    pub length: u16,
}

impl Delay {
    /// Create a new delay line
    ///
    /// # Arguments
    /// * `buffer` - Starting address in delay RAM (0-32767)
    /// * `length` - Length of the delay in samples
    pub fn new(buffer: u16, length: u16) -> Self {
        Self { buffer, length }
    }

    /// Read from the delay line at a given offset
    ///
    /// Returns instructions to read the delayed signal into ACC.
    ///
    /// # Arguments
    /// * `offset` - Offset from the buffer start (in samples)
    pub fn read(&self, offset: u16) -> Vec<Instruction> {
        vec![rda(self.buffer + offset, 1.0)]
    }

    /// Write to the delay line with optional feedback
    ///
    /// Returns instructions to write ACC to the delay line.
    ///
    /// # Arguments
    /// * `feedback` - Coefficient for crossfading with existing delay content
    pub fn write(&self, feedback: f32) -> Vec<Instruction> {
        vec![wra(self.buffer, feedback)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_block() {
        let inst = gain(Register::ADCL, Register::REG(16));
        match inst {
            Instruction::RDAX { reg, coeff } => {
                assert_eq!(reg, Register::ADCL);
                assert_eq!(coeff, 1.0);
            }
            _ => panic!("Expected RDAX instruction"),
        }
    }

    #[test]
    fn test_lowpass_block() {
        let instructions = lowpass(Register::ACC, Register::REG(16), Register::REG(1));
        assert_eq!(instructions.len(), 4);

        // Verify the sequence
        match &instructions[0] {
            Instruction::RDAX { reg, coeff } => {
                assert_eq!(*reg, Register::REG(1));
                assert_eq!(*coeff, -1.0);
            }
            _ => panic!("Expected RDAX instruction"),
        }

        match &instructions[1] {
            Instruction::MULX { reg } => {
                assert_eq!(*reg, Register::REG(16));
            }
            _ => panic!("Expected MULX instruction"),
        }
    }

    #[test]
    fn test_soft_clip_block() {
        let instructions = soft_clip(0.8);
        assert_eq!(instructions.len(), 3);

        match &instructions[0] {
            Instruction::SOF { coeff, offset } => {
                assert_eq!(*coeff, 0.8);
                assert_eq!(*offset, 0.0);
            }
            _ => panic!("Expected SOF instruction"),
        }

        assert_eq!(instructions[1], Instruction::ABSA);
    }

    #[test]
    fn test_delay_creation() {
        let delay = Delay::new(0, 4000);
        assert_eq!(delay.buffer, 0);
        assert_eq!(delay.length, 4000);
    }

    #[test]
    fn test_delay_read() {
        let delay = Delay::new(0, 4000);
        let instructions = delay.read(100);
        assert_eq!(instructions.len(), 1);

        match &instructions[0] {
            Instruction::RDA { addr, coeff } => {
                assert_eq!(*addr, 100);
                assert_eq!(*coeff, 1.0);
            }
            _ => panic!("Expected RDA instruction"),
        }
    }

    #[test]
    fn test_delay_write() {
        let delay = Delay::new(0, 4000);
        let instructions = delay.write(0.5);
        assert_eq!(instructions.len(), 1);

        match &instructions[0] {
            Instruction::WRA { addr, coeff } => {
                assert_eq!(*addr, 0);
                assert_eq!(*coeff, 0.5);
            }
            _ => panic!("Expected WRA instruction"),
        }
    }
}
