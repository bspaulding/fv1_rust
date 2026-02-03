use clap::Parser;
use fv1_asm::{Assembler, Parser as FV1Parser};
use miette::{Context, IntoDiagnostic, Result};
use std::fs;
use std::path::PathBuf;

/// FV-1 Assembler - Assemble FV-1 DSP programs
#[derive(Parser, Debug)]
#[command(name = "fv1-asm")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input assembly file
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output file (defaults to input filename with new extension)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "bin")]
    format: OutputFormat,

    /// Name for C array output (only used with --format=c)
    #[arg(short = 'n', long, default_value = "fv1_program")]
    name: String,

    /// Enable optimization
    #[arg(long)]
    optimize: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    /// Raw binary format (.bin)
    Bin,
    /// Intel HEX format (.hex)
    Hex,
    /// C array format (.c)
    C,
}

fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(3)
                .build(),
        )
    }))
    .into_diagnostic()?;

    let cli = Cli::parse();

    if cli.verbose {
        println!("FV-1 Assembler");
        println!("==============");
        println!("Input:  {}", cli.input.display());
        println!("Format: {:?}", cli.format);
        println!();
    }

    // Read input file
    let source = fs::read_to_string(&cli.input)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read input file: {}", cli.input.display()))?;

    // Parse
    if cli.verbose {
        println!("Parsing...");
    }
    let mut parser = FV1Parser::new(&source);
    let program = parser
        .parse()
        .wrap_err("Failed to parse assembly program")?;

    if cli.verbose {
        println!("Program has {} instructions", program.instructions().len());
    }

    // Assemble
    if cli.verbose {
        println!("Assembling...");
    }
    let assembler = Assembler::new().with_optimization(cli.optimize);
    let binary = assembler
        .assemble(&program)
        .wrap_err("Failed to assemble program")?;

    if cli.verbose {
        println!("Generated {} instruction binary", binary.len());
    }

    // Determine output path
    let output_path = cli.output.unwrap_or_else(|| {
        let mut path = cli.input.clone();
        path.set_extension(match cli.format {
            OutputFormat::Bin => "bin",
            OutputFormat::Hex => "hex",
            OutputFormat::C => "c",
        });
        path
    });

    // Generate output based on format
    match cli.format {
        OutputFormat::Bin => {
            let bytes = binary.to_bytes();
            fs::write(&output_path, bytes)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to write output file: {}", output_path.display()))?;
        }
        OutputFormat::Hex => {
            let hex = binary.to_hex();
            fs::write(&output_path, hex)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to write output file: {}", output_path.display()))?;
        }
        OutputFormat::C => {
            let c_array = binary.to_c_array(&cli.name);
            fs::write(&output_path, c_array)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to write output file: {}", output_path.display()))?;
        }
    }

    if cli.verbose {
        println!("Output written to: {}", output_path.display());
    } else {
        println!("Successfully assembled: {}", output_path.display());
    }

    Ok(())
}
