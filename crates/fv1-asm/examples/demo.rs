//! Demo program showcasing the complete FV-1 assembler workflow

use fv1_asm::{Assembler, Parser};

fn main() {
    println!("FV-1 Assembler Demo\n");
    println!("===================\n");

    // Example FV-1 program: Simple gain control using POT0
    let source = r#"
        ; Gain control example
        ; POT0 controls the gain level
        
        rdax  adcl, 1.0    ; Read left input
        mulx  reg0         ; Multiply by POT0 (in REG0)
        wrax  dacl, 0.0    ; Write to left output
        
        rdax  adcr, 1.0    ; Read right input  
        mulx  reg0         ; Multiply by POT0 (in REG0)
        wrax  dacr, 0.0    ; Write to right output
    "#;

    println!("Source code:\n{}\n", source);

    // Parse the source
    println!("Parsing...");
    let mut parser = Parser::new(source);
    let program = match parser.parse() {
        Ok(prog) => {
            println!("✓ Parse successful");
            prog
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
            return;
        }
    };

    println!("  - {} instructions parsed\n", program.instructions().len());

    // Assemble to binary
    println!("Assembling...");
    let assembler = Assembler::new();
    let binary = match assembler.assemble(&program) {
        Ok(bin) => {
            println!("✓ Assembly successful");
            bin
        }
        Err(e) => {
            eprintln!("✗ Assembly error: {}", e);
            return;
        }
    };

    println!("  - {} instructions in binary\n", binary.len());

    // Show different output formats
    println!("Output Formats:\n");

    // Raw bytes
    let bytes = binary.to_bytes();
    println!("1. Raw Binary:");
    println!("   Size: {} bytes", bytes.len());
    println!("   First 16 bytes: {:02X?}\n", &bytes[0..16]);

    // Intel HEX
    let hex = binary.to_hex();
    println!("2. Intel HEX Format:");
    let hex_lines: Vec<_> = hex.lines().take(5).collect();
    for line in hex_lines {
        println!("   {}", line);
    }
    println!("   ... ({} lines total)\n", hex.lines().count());

    // C array
    let c_array = binary.to_c_array("gain_control");
    println!("3. C Array Format:");
    let c_lines: Vec<_> = c_array.lines().take(6).collect();
    for line in c_lines {
        println!("   {}", line);
    }
    println!("   ...\n");

    println!("Demo complete! 🎉");
}
