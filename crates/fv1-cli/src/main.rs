use fv1_asm::{Instruction, Register};

fn main() {
    println!("FV-1 Assembler CLI");
    println!("==================");
    println!();

    // Example usage
    let inst = Instruction::RDAX {
        reg: Register::ADCL,
        coeff: 1.0,
    };

    println!("Example instruction: {:?}", inst);
}
