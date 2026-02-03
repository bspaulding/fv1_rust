use clap::{Parser, Subcommand};
use fv1_asm::{Assembler, Parser as FV1Parser};
use miette::{Context, IntoDiagnostic, Result};
use std::fs;
use std::path::PathBuf;

/// FV-1 DSP Assembler
#[derive(Parser, Debug)]
#[command(name = "fv1")]
#[command(about = "FV-1 DSP Assembler", long_about = None)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Assemble a .asm file to binary
    Assemble {
        /// Input assembly file
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

    match cli.command {
        Commands::Assemble {
            input,
            output,
            format,
            name,
            optimize,
            verbose,
        } => assemble_file(input, output, format, name, optimize, verbose)?,
        Commands::Disassemble { input, output } => disassemble_file(input, output)?,
        Commands::Check { input } => check_file(input)?,
    }

    Ok(())
}

fn assemble_file(
    input: PathBuf,
    output: Option<PathBuf>,
    format: OutputFormat,
    name: String,
    optimize: bool,
    verbose: bool,
) -> Result<()> {
    if verbose {
        println!("FV-1 Assembler");
        println!("==============");
        println!("Input:  {}", input.display());
        println!("Format: {:?}", format);
        println!();
    }

    // Read input file
    let source = fs::read_to_string(&input)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read input file: {}", input.display()))?;

    // Parse
    if verbose {
        println!("Parsing...");
    }
    let mut parser = FV1Parser::new(&source);
    let program = parser
        .parse()
        .wrap_err("Failed to parse assembly program")?;

    if verbose {
        println!("Program has {} instructions", program.instructions().len());
    }

    // Assemble
    if verbose {
        println!("Assembling...");
    }
    let assembler = Assembler::new().with_optimization(optimize);
    let binary = assembler
        .assemble(&program)
        .wrap_err("Failed to assemble program")?;

    if verbose {
        println!("Generated {} instruction binary", binary.len());
    }

    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        path.set_extension(match format {
            OutputFormat::Bin => "bin",
            OutputFormat::Hex => "hex",
            OutputFormat::C => "c",
        });
        path
    });

    // Generate output based on format
    match format {
        OutputFormat::Bin => {
            let bytes = binary.to_bytes();
            fs::write(&output_path, bytes)
                .into_diagnostic()
                .wrap_err_with(|| {
                    format!("Failed to write output file: {}", output_path.display())
                })?;
        }
        OutputFormat::Hex => {
            let hex = binary.to_hex();
            fs::write(&output_path, hex)
                .into_diagnostic()
                .wrap_err_with(|| {
                    format!("Failed to write output file: {}", output_path.display())
                })?;
        }
        OutputFormat::C => {
            let c_array = binary.to_c_array(&name);
            fs::write(&output_path, c_array)
                .into_diagnostic()
                .wrap_err_with(|| {
                    format!("Failed to write output file: {}", output_path.display())
                })?;
        }
    }

    if verbose {
        println!("Output written to: {}", output_path.display());
    } else {
        println!("✓ Successfully assembled to {}", output_path.display());
    }

    Ok(())
}

fn disassemble_file(input: PathBuf, _output: Option<PathBuf>) -> Result<()> {
    // TODO: Implement disassembler
    println!("Disassembling: {}", input.display());
    println!("⚠ Disassembler not yet implemented");
    Ok(())
}

fn check_file(input: PathBuf) -> Result<()> {
    let source = fs::read_to_string(&input)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to read input file: {}", input.display()))?;

    let mut parser = FV1Parser::new(&source);
    let program = parser
        .parse()
        .wrap_err("Failed to parse assembly program")?;

    println!("✓ {} is valid", input.display());
    println!("  {} instructions", program.instructions().len());
    println!("  {} labels", program.labels.len());

    Ok(())
}
