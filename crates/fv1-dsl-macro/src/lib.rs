use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Token};

/// Custom parser for a sequence of semicolon-terminated expressions
struct ProgramStatements {
    statements: Vec<Expr>,
}

impl Parse for ProgramStatements {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut statements = Vec::new();

        while !input.is_empty() {
            let expr: Expr = input.parse()?;
            statements.push(expr);

            // Consume optional semicolon
            if input.peek(Token![;]) {
                let _: Token![;] = input.parse()?;
            }
        }

        Ok(ProgramStatements { statements })
    }
}

/// Procedural macro for writing FV-1 programs using Rust syntax
///
/// # Example
///
/// ```ignore
/// use fv1_dsl::prelude::*;
///
/// let program = fv1_program! {
///     rdax(Register::ADCL, 1.0);
///     mulx(Register::POT0);
///     wrax(Register::DACL, 0.0);
/// };
/// ```
#[proc_macro]
pub fn fv1_program(input: TokenStream) -> TokenStream {
    let program_stmts = parse_macro_input!(input as ProgramStatements);
    let instructions = program_stmts.statements;

    let output = quote! {
        {
            let mut builder = ::fv1_dsl::ProgramBuilder::new();
            #(builder.add_inst(#instructions);)*
            builder.build()
        }
    };

    output.into()
}
