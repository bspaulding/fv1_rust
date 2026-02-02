/// FV-1 Hardware Constants

/// Maximum number of instructions in a program
pub const MAX_INSTRUCTIONS: usize = 128;

/// Delay RAM size in samples
pub const DELAY_RAM_SIZE: usize = 32768;

/// Number of general purpose registers
pub const NUM_REGISTERS: usize = 32;

/// Sample rate of the FV-1 (fixed at 32.768 kHz)
pub const SAMPLE_RATE: f32 = 32768.0;

/// Maximum delay time in seconds (DELAY_RAM_SIZE / SAMPLE_RATE)
pub const MAX_DELAY_TIME: f32 = DELAY_RAM_SIZE as f32 / SAMPLE_RATE;

/// S1.14 fixed-point format used by FV-1
pub const FIXED_POINT_SCALE: f32 = 16384.0; // 2^14

/// S.23 fixed-point format for delay addresses
pub const ADDR_FIXED_POINT_SCALE: f32 = 8388608.0; // 2^23

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(MAX_INSTRUCTIONS, 128);
        assert_eq!(DELAY_RAM_SIZE, 32768);
        assert_eq!(NUM_REGISTERS, 32);
        assert_eq!(SAMPLE_RATE, 32768.0);
    }

    #[test]
    fn test_max_delay_time() {
        // Should be 1 second
        assert_eq!(MAX_DELAY_TIME, 1.0);
    }
}
