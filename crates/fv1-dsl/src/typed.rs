use crate::{Instruction, ProgramBuilder, Register};
use std::marker::PhantomData;

/// Phantom type representing the state of the accumulator (ACC)
pub struct Acc<T> {
    _phantom: PhantomData<T>,
}

/// Marker: ACC contains audio data
pub struct Audio;

/// Marker: ACC contains control data
pub struct Control;

/// Marker: ACC contains LFO data
pub struct Lfo;

/// Type-safe instruction builder that tracks accumulator state
///
/// This builder uses phantom types to provide compile-time guarantees
/// about the state of the accumulator. Different instruction types
/// transition between states, ensuring correct usage at compile time.
///
/// # Example
///
/// ```
/// use fv1_dsl::TypedBuilder;
/// use fv1_asm::Register;
///
/// let program = TypedBuilder::new()
///     .rdax(Register::ADCL, 1.0)   // Transitions to Audio state
///     .mulx(Register::REG(0))      // Stays in Audio state
///     .wrax(Register::DACL, 0.0)   // Stays in Audio state
///     .build();
/// ```
pub struct TypedBuilder<State> {
    builder: ProgramBuilder,
    _state: PhantomData<State>,
}

impl TypedBuilder<()> {
    /// Create a new typed builder in the initial state
    pub fn new() -> Self {
        Self {
            builder: ProgramBuilder::new(),
            _state: PhantomData,
        }
    }
}

impl Default for TypedBuilder<()> {
    fn default() -> Self {
        Self::new()
    }
}

// Instructions available from any state
impl<S> TypedBuilder<S> {
    /// Read from register and accumulate (transitions to Audio state)
    ///
    /// RDAX reads a value from a register, multiplies it by a coefficient,
    /// and adds it to the accumulator.
    pub fn rdax(mut self, reg: Register, coeff: f32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::RDAX { reg, coeff });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Clear the accumulator (transitions to Audio state with zero)
    pub fn clr(mut self) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::CLR);
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// No operation
    pub fn nop(mut self) -> TypedBuilder<S> {
        self.builder = self.builder.inst(Instruction::NOP);
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Build the final program
    pub fn build(self) -> fv1_asm::Program {
        self.builder.build()
    }
}

// Instructions available in Audio state
impl TypedBuilder<Audio> {
    /// Write to register and accumulate (stays in Audio state)
    ///
    /// WRAX writes the current accumulator value to a register,
    /// then multiplies the accumulator by a coefficient.
    pub fn wrax(mut self, reg: Register, coeff: f32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::WRAX { reg, coeff });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Multiply accumulator by register (stays in Audio state)
    ///
    /// MULX multiplies the accumulator by the value in a register.
    pub fn mulx(mut self, reg: Register) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::MULX { reg });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Scale and offset (stays in Audio state)
    ///
    /// SOF multiplies the accumulator by a coefficient and adds an offset.
    pub fn sof(mut self, coeff: f32, offset: f32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::SOF { coeff, offset });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Read from delay memory (stays in Audio state)
    ///
    /// RDA reads from delay memory at the specified address,
    /// multiplies by coefficient, and adds to accumulator.
    pub fn rda(mut self, addr: u16, coeff: f32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::RDA { addr, coeff });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Write to delay memory (stays in Audio state)
    ///
    /// WRA writes the accumulator to delay memory and multiplies
    /// accumulator by coefficient.
    pub fn wra(mut self, addr: u16, coeff: f32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::WRA { addr, coeff });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Write to delay memory and wrap (stays in Audio state)
    ///
    /// WRAP is similar to WRA but handles delay line wrapping.
    pub fn wrap(mut self, addr: u16, coeff: f32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::WRAP { addr, coeff });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Read-multiply-accumulate (stays in Audio state)
    ///
    /// RMPA reads from delay memory using a pointer register and accumulates.
    pub fn rmpa(mut self, coeff: f32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::RMPA { coeff });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Load accumulator with register * coefficient (stays in Audio state)
    pub fn ldax(mut self, reg: Register) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::LDAX { reg });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Absolute value (stays in Audio state)
    pub fn absa(mut self) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::ABSA);
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Exponential conversion (stays in Audio state)
    pub fn exp(mut self, coeff: f32, offset: f32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::EXP { coeff, offset });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Logarithmic conversion (stays in Audio state)
    pub fn log(mut self, coeff: f32, offset: f32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::LOG { coeff, offset });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Bitwise AND (stays in Audio state)
    pub fn and(mut self, mask: u32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::AND { mask });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Bitwise OR (stays in Audio state)
    pub fn or(mut self, mask: u32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::OR { mask });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }

    /// Bitwise XOR (stays in Audio state)
    pub fn xor(mut self, mask: u32) -> TypedBuilder<Audio> {
        self.builder = self.builder.inst(Instruction::XOR { mask });
        TypedBuilder {
            builder: self.builder,
            _state: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typed_builder_creation() {
        let builder = TypedBuilder::new();
        let program = builder.build();
        assert_eq!(program.instructions().len(), 0);
    }

    #[test]
    fn test_typed_builder_simple_chain() {
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 2);
    }

    #[test]
    fn test_typed_builder_audio_operations() {
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .mulx(Register::REG(0))
            .sof(0.5, 0.0)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 4);
    }

    #[test]
    fn test_typed_builder_with_clr() {
        let program = TypedBuilder::new()
            .clr()
            .rdax(Register::ADCL, 1.0)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 3);
    }

    #[test]
    fn test_typed_builder_with_nop() {
        let program = TypedBuilder::new()
            .nop()
            .rdax(Register::ADCL, 1.0)
            .nop()
            .wrax(Register::DACL, 0.0)
            .nop()
            .build();

        assert_eq!(program.instructions().len(), 5);
    }

    #[test]
    fn test_typed_builder_delay_operations() {
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .wra(0, 0.0)
            .rda(4000, 0.5)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 4);
    }

    #[test]
    fn test_typed_builder_complex_chain() {
        let program = TypedBuilder::new()
            .clr()
            .rdax(Register::ADCL, 1.0)
            .mulx(Register::REG(16)) // POT0
            .sof(0.8, 0.0)
            .wrax(Register::REG(0), 0.0)
            .rda(8000, 0.5)
            .mulx(Register::REG(17)) // POT1
            .rdax(Register::REG(0), 1.0)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 9);
    }

    #[test]
    fn test_typed_builder_absa() {
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .absa()
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 3);
    }

    #[test]
    fn test_typed_builder_exp_log() {
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .exp(1.0, 0.0)
            .log(1.0, 0.0)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 4);
    }

    #[test]
    fn test_typed_builder_bitwise_ops() {
        let program = TypedBuilder::new()
            .rdax(Register::ADCL, 1.0)
            .and(0xFFFF)
            .or(0x0001)
            .xor(0x0001)
            .wrax(Register::DACL, 0.0)
            .build();

        assert_eq!(program.instructions().len(), 5);
    }
}
