use logos::Logos;

/// Token types for FV-1 assembly language
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")] // Skip whitespace
#[logos(skip r";[^\n]*")] // Skip comments starting with semicolon
#[allow(non_camel_case_types)] // Allow register names like ADDR_PTR, SIN0_RATE
pub enum Token {
    // Instructions (case-insensitive)
    #[token("rdax", ignore(ascii_case))]
    RDAX,
    #[token("rda", ignore(ascii_case))]
    RDA,
    #[token("wrax", ignore(ascii_case))]
    WRAX,
    #[token("wra", ignore(ascii_case))]
    WRA,
    #[token("wrap", ignore(ascii_case))]
    WRAP,
    #[token("rmpa", ignore(ascii_case))]
    RMPA,
    #[token("mulx", ignore(ascii_case))]
    MULX,
    #[token("rdfx", ignore(ascii_case))]
    RDFX,
    #[token("absa", ignore(ascii_case))]
    ABSA,
    #[token("ldax", ignore(ascii_case))]
    LDAX,
    #[token("rdfx2", ignore(ascii_case))]
    RDFX2,
    #[token("sof", ignore(ascii_case))]
    SOF,
    #[token("and", ignore(ascii_case))]
    AND,
    #[token("or", ignore(ascii_case))]
    OR,
    #[token("xor", ignore(ascii_case))]
    XOR,
    #[token("shl", ignore(ascii_case))]
    SHL,
    #[token("shr", ignore(ascii_case))]
    SHR,
    #[token("clr", ignore(ascii_case))]
    CLR,
    #[token("nop", ignore(ascii_case))]
    NOP,
    #[token("exp", ignore(ascii_case))]
    EXP,
    #[token("log", ignore(ascii_case))]
    LOG,
    #[token("skp", ignore(ascii_case))]
    SKP,
    #[token("wlds", ignore(ascii_case))]
    WLDS,
    #[token("jam", ignore(ascii_case))]
    JAM,
    #[token("cho", ignore(ascii_case))]
    CHO,

    // Registers (case-insensitive)
    #[token("acc", ignore(ascii_case))]
    ACC,
    #[token("adcl", ignore(ascii_case))]
    ADCL,
    #[token("adcr", ignore(ascii_case))]
    ADCR,
    #[token("dacl", ignore(ascii_case))]
    DACL,
    #[token("dacr", ignore(ascii_case))]
    DACR,
    #[token("addr_ptr", ignore(ascii_case))]
    ADDR_PTR,
    #[token("lr", ignore(ascii_case))]
    LR,

    // REG pattern: reg0-reg31 (case-insensitive)
    #[regex(r"(?i)reg([0-9]|[12][0-9]|3[01])", priority = 2, callback = parse_reg)]
    REG(u8),

    // Control inputs (case-insensitive)
    #[regex(r"(?i)pot[0-2]", callback = parse_pot)]
    POT(u8),

    // LFOs (case-insensitive)
    #[token("sin0", ignore(ascii_case))]
    SIN0,
    #[token("sin1", ignore(ascii_case))]
    SIN1,
    #[token("rmp0", ignore(ascii_case))]
    RMP0,
    #[token("rmp1", ignore(ascii_case))]
    RMP1,

    // LFO rate/range registers
    #[token("sin0_rate", ignore(ascii_case))]
    SIN0_RATE,
    #[token("sin0_range", ignore(ascii_case))]
    SIN0_RANGE,
    #[token("sin1_rate", ignore(ascii_case))]
    SIN1_RATE,
    #[token("sin1_range", ignore(ascii_case))]
    SIN1_RANGE,
    #[token("rmp0_rate", ignore(ascii_case))]
    RMP0_RATE,
    #[token("rmp0_range", ignore(ascii_case))]
    RMP0_RANGE,
    #[token("rmp1_rate", ignore(ascii_case))]
    RMP1_RATE,
    #[token("rmp1_range", ignore(ascii_case))]
    RMP1_RANGE,

    // Skip conditions
    #[token("gez", ignore(ascii_case))]
    GEZ,
    #[token("neg", ignore(ascii_case))]
    NEG,
    #[token("zrc", ignore(ascii_case))]
    ZRC,
    #[token("zro", ignore(ascii_case))]
    ZRO,
    #[token("run", ignore(ascii_case))]
    RUN,

    // CHO modes
    #[token("rdal", ignore(ascii_case))]
    RDAL,

    // CHO flags
    #[token("rptr2", ignore(ascii_case))]
    RPTR2,
    #[token("na", ignore(ascii_case))]
    NA,
    #[token("compc", ignore(ascii_case))]
    COMPC,
    #[token("compa", ignore(ascii_case))]
    COMPA,

    // Numeric literals
    // Float: Must come before integer to match decimal numbers correctly
    #[regex(r"-?[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", parse_float)]
    #[regex(r"-?[0-9]+[eE][+-]?[0-9]+", parse_float)]
    Float(f32),

    // Hex integer: 0x prefix or $ prefix
    #[regex(r"0x[0-9a-fA-F]+", parse_hex)]
    #[regex(r"\$[0-9a-fA-F]+", parse_hex_dollar)]
    // Binary integer: % prefix
    #[regex(r"%[01]+", parse_binary)]
    // Decimal integer
    #[regex(r"-?[0-9]+", priority = 1, callback = parse_int)]
    Integer(i64),

    // Identifiers (labels, equates) - lower priority than keywords
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 1, callback = |lex| lex.slice().to_string())]
    Identifier(String),

    // Operators and punctuation
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("=")]
    Equals,
    #[token("|")]
    Pipe,

    // Directives
    #[token("equ", ignore(ascii_case))]
    EQU,
    #[token("mem", ignore(ascii_case))]
    MEM,
    #[token("spinasm", ignore(ascii_case))]
    SPINASM,

    // Special
    #[token("#")]
    Hash,
}

// Helper functions for parsing token values
fn parse_reg(lex: &mut logos::Lexer<Token>) -> Option<u8> {
    let slice = lex.slice();
    let num_part = &slice[3..]; // Skip "reg" prefix
    num_part.parse().ok()
}

fn parse_pot(lex: &mut logos::Lexer<Token>) -> Option<u8> {
    let slice = lex.slice();
    let num_part = &slice[3..]; // Skip "pot" prefix
    num_part.parse().ok()
}

fn parse_float(lex: &mut logos::Lexer<Token>) -> Option<f32> {
    lex.slice().parse().ok()
}

fn parse_int(lex: &mut logos::Lexer<Token>) -> Option<i64> {
    lex.slice().parse().ok()
}

fn parse_hex(lex: &mut logos::Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();
    i64::from_str_radix(&slice[2..], 16).ok()
}

fn parse_hex_dollar(lex: &mut logos::Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();
    i64::from_str_radix(&slice[1..], 16).ok()
}

fn parse_binary(lex: &mut logos::Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();
    i64::from_str_radix(&slice[1..], 2).ok()
}

/// Lexer for FV-1 assembly source code
pub struct Lexer<'source> {
    inner: logos::Lexer<'source, Token>,
}

impl<'source> Lexer<'source> {
    /// Create a new lexer for the given source code
    pub fn new(source: &'source str) -> Self {
        Self {
            inner: Token::lexer(source),
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = (Result<Token, ()>, std::ops::Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.inner.next()?;
        let span = self.inner.span();
        Some((token, span))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "rdax adcl, 0.5";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::RDAX);
        assert_eq!(tokens[1], Token::ADCL);
        assert_eq!(tokens[2], Token::Comma);
        assert_eq!(tokens[3], Token::Float(0.5));
    }

    #[test]
    fn test_case_insensitive() {
        let source = "RDAX AdCl, 0.5";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::RDAX);
        assert_eq!(tokens[1], Token::ADCL);
    }

    #[test]
    fn test_comments() {
        let source = r#"
            rdax adcl, 0.5  ; read left input
            ; full line comment
            sof 0, 0        ; clear
        "#;

        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        // Should skip comments and only get instruction tokens
        assert_eq!(tokens.len(), 8); // RDAX ADCL , 0.5 SOF 0 , 0
    }

    #[test]
    fn test_register_tokens() {
        let source = "reg0 reg15 reg31";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::REG(0));
        assert_eq!(tokens[1], Token::REG(15));
        assert_eq!(tokens[2], Token::REG(31));
    }

    #[test]
    fn test_pot_tokens() {
        let source = "pot0 pot1 pot2";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::POT(0));
        assert_eq!(tokens[1], Token::POT(1));
        assert_eq!(tokens[2], Token::POT(2));
    }

    #[test]
    fn test_numeric_literals() {
        let source = "1.5 -0.5 42 -10 0x1A $FF %1010";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::Float(1.5));
        assert_eq!(tokens[1], Token::Float(-0.5));
        assert_eq!(tokens[2], Token::Integer(42));
        assert_eq!(tokens[3], Token::Integer(-10));
        assert_eq!(tokens[4], Token::Integer(0x1A));
        assert_eq!(tokens[5], Token::Integer(0xFF));
        assert_eq!(tokens[6], Token::Integer(0b1010));
    }

    #[test]
    fn test_identifiers() {
        let source = "my_label loop_start _private";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::Identifier("my_label".to_string()));
        assert_eq!(tokens[1], Token::Identifier("loop_start".to_string()));
        assert_eq!(tokens[2], Token::Identifier("_private".to_string()));
    }

    #[test]
    fn test_lfo_tokens() {
        let source = "sin0 sin1 rmp0 rmp1";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::SIN0);
        assert_eq!(tokens[1], Token::SIN1);
        assert_eq!(tokens[2], Token::RMP0);
        assert_eq!(tokens[3], Token::RMP1);
    }

    #[test]
    fn test_skip_conditions() {
        let source = "gez neg zrc zro run";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::GEZ);
        assert_eq!(tokens[1], Token::NEG);
        assert_eq!(tokens[2], Token::ZRC);
        assert_eq!(tokens[3], Token::ZRO);
        assert_eq!(tokens[4], Token::RUN);
    }

    #[test]
    fn test_directives() {
        let source = "equ mem";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::EQU);
        assert_eq!(tokens[1], Token::MEM);
    }

    #[test]
    fn test_label_syntax() {
        let source = "loop: rdax adcl, 1.0";
        let tokens: Vec<_> = Lexer::new(source)
            .map(|(tok, _)| tok)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens[0], Token::Identifier("loop".to_string()));
        assert_eq!(tokens[1], Token::Colon);
        assert_eq!(tokens[2], Token::RDAX);
    }
}
