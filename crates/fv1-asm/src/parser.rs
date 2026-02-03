use crate::{
    ast::*,
    error::ParseError,
    instruction::*,
    lexer::{Lexer, Token},
    register::*,
};

/// Parser for FV-1 assembly source code
pub struct Parser<'source> {
    tokens: Vec<(Result<Token, ()>, std::ops::Range<usize>)>,
    pos: usize,
    #[allow(dead_code)]
    source: &'source str,
}

impl<'source> Parser<'source> {
    /// Create a new parser for the given source code
    pub fn new(source: &'source str) -> Self {
        let tokens: Vec<_> = Lexer::new(source).collect();
        Self {
            tokens,
            pos: 0,
            source,
        }
    }

    /// Parse the source code into a Program
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::new();

        while !self.is_at_end() {
            // Try to parse directive or statement
            if self.check_directive() {
                program.directives.push(self.parse_directive()?);
            } else {
                let stmt = self.parse_statement()?;
                program.add_statement(stmt);
            }
        }

        Ok(program)
    }

    /// Parse a statement (label, instruction, or labeled instruction)
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        // Check for label followed by colon
        if let Some((Ok(Token::Identifier(name)), _)) = self.peek() {
            if matches!(self.peek_next(), Some((Ok(Token::Colon), _))) {
                let label = name.clone();
                self.advance(); // consume identifier
                self.advance(); // consume colon

                // Check if there's an instruction on the same line
                if !self.is_at_end() && self.is_instruction() {
                    let instruction = self.parse_instruction()?;
                    return Ok(Statement::LabeledInstruction { label, instruction });
                } else {
                    return Ok(Statement::Label(label));
                }
            }
        }

        // Parse standalone instruction
        let instruction = self.parse_instruction()?;
        Ok(Statement::Instruction(instruction))
    }

    /// Parse an instruction
    fn parse_instruction(&mut self) -> Result<Instruction, ParseError> {
        let (token, span) = self.advance_checked()?;

        match token {
            Token::RDAX => {
                let reg = self.parse_register()?;
                self.expect(Token::Comma)?;
                let coeff = self.parse_number()? as f32;
                Ok(Instruction::RDAX { reg, coeff })
            }
            Token::RDA => {
                let addr = self.parse_number()? as u16;
                self.expect(Token::Comma)?;
                let coeff = self.parse_number()? as f32;
                Ok(Instruction::RDA { addr, coeff })
            }
            Token::WRAX => {
                let reg = self.parse_register()?;
                self.expect(Token::Comma)?;
                let coeff = self.parse_number()? as f32;
                Ok(Instruction::WRAX { reg, coeff })
            }
            Token::WRA => {
                let addr = self.parse_number()? as u16;
                self.expect(Token::Comma)?;
                let coeff = self.parse_number()? as f32;
                Ok(Instruction::WRA { addr, coeff })
            }
            Token::WRAP => {
                let addr = self.parse_number()? as u16;
                self.expect(Token::Comma)?;
                let coeff = self.parse_number()? as f32;
                Ok(Instruction::WRAP { addr, coeff })
            }
            Token::RMPA => {
                let coeff = self.parse_number()? as f32;
                Ok(Instruction::RMPA { coeff })
            }
            Token::MULX => {
                let reg = self.parse_register()?;
                Ok(Instruction::MULX { reg })
            }
            Token::RDFX => {
                let reg = self.parse_register()?;
                self.expect(Token::Comma)?;
                let coeff = self.parse_number()? as f32;
                Ok(Instruction::RDFX { reg, coeff })
            }
            Token::RDFX2 => {
                let reg = self.parse_register()?;
                self.expect(Token::Comma)?;
                let coeff = self.parse_number()? as f32;
                Ok(Instruction::RDFX2 { reg, coeff })
            }
            Token::LDAX => {
                let reg = self.parse_register()?;
                Ok(Instruction::LDAX { reg })
            }
            Token::SOF => {
                let coeff = self.parse_number()? as f32;
                self.expect(Token::Comma)?;
                let offset = self.parse_number()? as f32;
                Ok(Instruction::SOF { coeff, offset })
            }
            Token::AND => {
                let mask = self.parse_number()? as u32;
                Ok(Instruction::AND { mask })
            }
            Token::OR => {
                let mask = self.parse_number()? as u32;
                Ok(Instruction::OR { mask })
            }
            Token::XOR => {
                let mask = self.parse_number()? as u32;
                Ok(Instruction::XOR { mask })
            }
            Token::EXP => {
                let coeff = self.parse_number()? as f32;
                self.expect(Token::Comma)?;
                let offset = self.parse_number()? as f32;
                Ok(Instruction::EXP { coeff, offset })
            }
            Token::LOG => {
                let coeff = self.parse_number()? as f32;
                self.expect(Token::Comma)?;
                let offset = self.parse_number()? as f32;
                Ok(Instruction::LOG { coeff, offset })
            }
            Token::SKP => {
                let condition = self.parse_skip_condition()?;
                self.expect(Token::Comma)?;
                let offset = self.parse_number()? as i8;
                Ok(Instruction::SKP { condition, offset })
            }
            Token::WLDS => {
                let lfo = self.parse_lfo()?;
                self.expect(Token::Comma)?;
                let freq = self.parse_number()? as u16;
                self.expect(Token::Comma)?;
                let amplitude = self.parse_number()? as u16;
                Ok(Instruction::WLDS { lfo, freq, amplitude })
            }
            Token::JAM => {
                let lfo = self.parse_lfo()?;
                Ok(Instruction::JAM { lfo })
            }
            Token::CHO => {
                let mode = self.parse_cho_mode()?;
                self.expect(Token::Comma)?;
                let lfo = self.parse_lfo()?;
                self.expect(Token::Comma)?;
                let flags = self.parse_cho_flags()?;
                self.expect(Token::Comma)?;
                let addr = self.parse_number()? as u16;
                Ok(Instruction::CHO { mode, lfo, flags, addr })
            }
            Token::ABSA => Ok(Instruction::ABSA),
            Token::SHL => Ok(Instruction::SHL),
            Token::SHR => Ok(Instruction::SHR),
            Token::CLR => Ok(Instruction::CLR),
            Token::NOP => Ok(Instruction::NOP),
            _ => Err(ParseError::UnexpectedToken {
                expected: "instruction".to_string(),
                found: format!("{:?}", token),
                span,
            }),
        }
    }

    /// Parse a register
    fn parse_register(&mut self) -> Result<Register, ParseError> {
        let (token, span) = self.advance_checked()?;

        match token {
            Token::ACC => Ok(Register::ACC),
            Token::ADCL => Ok(Register::ADCL),
            Token::ADCR => Ok(Register::ADCR),
            Token::DACL => Ok(Register::DACL),
            Token::DACR => Ok(Register::DACR),
            Token::ADDR_PTR => Ok(Register::ADDR_PTR),
            Token::LR => Ok(Register::LR),
            Token::REG(n) => Ok(Register::REG(n)),
            Token::SIN0_RATE => Ok(Register::SIN0_RATE),
            Token::SIN0_RANGE => Ok(Register::SIN0_RANGE),
            Token::SIN1_RATE => Ok(Register::SIN1_RATE),
            Token::SIN1_RANGE => Ok(Register::SIN1_RANGE),
            Token::RMP0_RATE => Ok(Register::RMP0_RATE),
            Token::RMP0_RANGE => Ok(Register::RMP0_RANGE),
            Token::RMP1_RATE => Ok(Register::RMP1_RATE),
            Token::RMP1_RANGE => Ok(Register::RMP1_RANGE),
            _ => Err(ParseError::ExpectedRegister { span }),
        }
    }

    /// Parse a numeric value (float or integer)
    fn parse_number(&mut self) -> Result<f64, ParseError> {
        let (token, span) = self.advance_checked()?;

        match token {
            Token::Float(f) => Ok(f as f64),
            Token::Integer(i) => Ok(i as f64),
            _ => Err(ParseError::ExpectedNumber { span }),
        }
    }

    /// Parse an LFO
    fn parse_lfo(&mut self) -> Result<Lfo, ParseError> {
        let (token, span) = self.advance_checked()?;

        match token {
            Token::SIN0 => Ok(Lfo::SIN0),
            Token::SIN1 => Ok(Lfo::SIN1),
            Token::RMP0 => Ok(Lfo::RMP0),
            Token::RMP1 => Ok(Lfo::RMP1),
            _ => Err(ParseError::UnexpectedToken {
                expected: "LFO (sin0, sin1, rmp0, rmp1)".to_string(),
                found: format!("{:?}", token),
                span,
            }),
        }
    }

    /// Parse a skip condition
    fn parse_skip_condition(&mut self) -> Result<SkipCondition, ParseError> {
        let (token, span) = self.advance_checked()?;

        match token {
            Token::GEZ => Ok(SkipCondition::GEZ),
            Token::NEG => Ok(SkipCondition::NEG),
            Token::ZRC => Ok(SkipCondition::ZRC),
            Token::ZRO => Ok(SkipCondition::ZRO),
            Token::RUN => Ok(SkipCondition::RUN),
            _ => Err(ParseError::UnexpectedToken {
                expected: "skip condition (gez, neg, zrc, zro, run)".to_string(),
                found: format!("{:?}", token),
                span,
            }),
        }
    }

    /// Parse CHO mode
    fn parse_cho_mode(&mut self) -> Result<ChoMode, ParseError> {
        let (token, span) = self.advance_checked()?;

        match token {
            Token::RDA => Ok(ChoMode::RDA),
            Token::SOF => Ok(ChoMode::SOF),
            Token::RDAL => Ok(ChoMode::RDAL),
            _ => Err(ParseError::UnexpectedToken {
                expected: "CHO mode (rda, sof, rdal)".to_string(),
                found: format!("{:?}", token),
                span,
            }),
        }
    }

    /// Parse CHO flags (simplified - just returns default flags for now)
    fn parse_cho_flags(&mut self) -> Result<ChoFlags, ParseError> {
        // For now, parse a single identifier or integer representing flags
        // This is simplified; a full implementation would parse flag combinations
        let (_token, _span) = self.advance_checked()?;

        // Return default flags
        Ok(ChoFlags {
            rptr2: false,
            na: false,
            compc: false,
            compa: false,
            rptr2_select: false,
        })
    }

    /// Parse a directive
    fn parse_directive(&mut self) -> Result<Directive, ParseError> {
        let (token, span) = self.advance_checked()?;

        match token {
            Token::EQU => {
                let name = self.parse_identifier()?;
                self.expect(Token::Comma)?;
                let value = self.parse_value()?;
                Ok(Directive::Equate { name, value })
            }
            Token::MEM => {
                let name = self.parse_identifier()?;
                let size = self.parse_number()? as u16;
                Ok(Directive::MemoryAllocation { name, size })
            }
            Token::SPINASM => {
                let version = self.parse_identifier()?;
                Ok(Directive::SpinAsm { version })
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "directive (equ, mem, spinasm)".to_string(),
                found: format!("{:?}", token),
                span,
            }),
        }
    }

    /// Parse an identifier
    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        let (token, span) = self.advance_checked()?;

        match token {
            Token::Identifier(s) => Ok(s),
            _ => Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: format!("{:?}", token),
                span,
            }),
        }
    }

    /// Parse a value (for directives)
    fn parse_value(&mut self) -> Result<Value, ParseError> {
        let (token, _span) = self.advance_checked()?;

        match token {
            Token::Float(f) => Ok(Value::Float(f)),
            Token::Integer(i) => Ok(Value::Integer(i)),
            Token::Identifier(s) => Ok(Value::Identifier(s)),
            _ => Err(ParseError::ExpectedNumber { span: 0..0 }),
        }
    }

    // Helper methods

    /// Check if current token is a directive
    fn check_directive(&self) -> bool {
        matches!(
            self.peek(),
            Some((Ok(Token::EQU | Token::MEM | Token::SPINASM), _))
        )
    }

    /// Check if current token is an instruction
    fn is_instruction(&self) -> bool {
        matches!(
            self.peek(),
            Some((
                Ok(Token::RDAX
                    | Token::RDA
                    | Token::WRAX
                    | Token::WRA
                    | Token::WRAP
                    | Token::RMPA
                    | Token::MULX
                    | Token::RDFX
                    | Token::RDFX2
                    | Token::LDAX
                    | Token::SOF
                    | Token::AND
                    | Token::OR
                    | Token::XOR
                    | Token::EXP
                    | Token::LOG
                    | Token::SKP
                    | Token::WLDS
                    | Token::JAM
                    | Token::CHO
                    | Token::ABSA
                    | Token::SHL
                    | Token::SHR
                    | Token::CLR
                    | Token::NOP),
                _
            ))
        )
    }

    /// Check if at end of token stream
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Peek at current token without consuming
    fn peek(&self) -> Option<&(Result<Token, ()>, std::ops::Range<usize>)> {
        self.tokens.get(self.pos)
    }

    /// Peek at next token without consuming
    fn peek_next(&self) -> Option<&(Result<Token, ()>, std::ops::Range<usize>)> {
        self.tokens.get(self.pos + 1)
    }

    /// Advance to next token
    fn advance(&mut self) -> Option<&(Result<Token, ()>, std::ops::Range<usize>)> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    /// Advance and return token, or error if at end
    fn advance_checked(&mut self) -> Result<(Token, std::ops::Range<usize>), ParseError> {
        if self.is_at_end() {
            return Err(ParseError::UnexpectedEof);
        }
        let (token_result, span) = self.advance().unwrap();
        match token_result {
            Ok(token) => Ok((token.clone(), span.clone())),
            Err(_) => Err(ParseError::InvalidToken { span: span.clone() }),
        }
    }

    /// Expect a specific token
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let (token, span) = self.advance_checked()?;

        if std::mem::discriminant(&token) == std::mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: format!("{:?}", token),
                span,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_instruction() {
        let source = "clr";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        assert_eq!(program.instructions().len(), 1);
        assert!(matches!(program.instructions()[0], Instruction::CLR));
    }

    #[test]
    fn test_parse_rdax() {
        let source = "rdax adcl, 1.0";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        assert_eq!(program.instructions().len(), 1);
        match program.instructions()[0] {
            Instruction::RDAX { reg, coeff } => {
                assert_eq!(*reg, Register::ADCL);
                assert_eq!(*coeff, 1.0);
            }
            _ => panic!("Wrong instruction type"),
        }
    }

    #[test]
    fn test_parse_sof() {
        let source = "sof 0.5, 0.0";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        assert_eq!(program.instructions().len(), 1);
        match program.instructions()[0] {
            Instruction::SOF { coeff, offset } => {
                assert_eq!(*coeff, 0.5);
                assert_eq!(*offset, 0.0);
            }
            _ => panic!("Wrong instruction type"),
        }
    }

    #[test]
    fn test_parse_label() {
        let source = "start: clr";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        assert_eq!(program.instructions().len(), 1);
        assert_eq!(program.labels.get("start"), Some(&0));
    }

    #[test]
    fn test_parse_multiple_instructions() {
        let source = r#"
            clr
            rdax adcl, 1.0
            sof 0.5, 0.0
            wrax dacl, 0
        "#;
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        assert_eq!(program.instructions().len(), 4);
    }

    #[test]
    fn test_parse_with_comments() {
        let source = r#"
            ; Clear accumulator
            clr
            rdax adcl, 1.0  ; Read left input
        "#;
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        assert_eq!(program.instructions().len(), 2);
    }

    #[test]
    fn test_parse_register_variants() {
        let source = "rdax reg0, 1.0";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        match program.instructions()[0] {
            Instruction::RDAX { reg, .. } => {
                assert_eq!(*reg, Register::REG(0));
            }
            _ => panic!("Wrong instruction"),
        }
    }

    #[test]
    fn test_parse_skip_instruction() {
        let source = "skp gez, 2";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        match program.instructions()[0] {
            Instruction::SKP { condition, offset } => {
                assert_eq!(*condition, SkipCondition::GEZ);
                assert_eq!(*offset, 2);
            }
            _ => panic!("Wrong instruction"),
        }
    }

    #[test]
    fn test_parse_directive_equ() {
        let source = "equ GAIN, 0.5";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        assert_eq!(program.directives.len(), 1);
        match &program.directives[0] {
            Directive::Equate { name, value } => {
                assert_eq!(name, "GAIN");
                match value {
                    Value::Float(v) => assert_eq!(*v, 0.5),
                    _ => panic!("Wrong value type"),
                }
            }
            _ => panic!("Wrong directive"),
        }
    }

    #[test]
    fn test_parse_directive_mem() {
        let source = "mem delay 4096";
        let mut parser = Parser::new(source);
        let program = parser.parse().unwrap();

        assert_eq!(program.directives.len(), 1);
        match &program.directives[0] {
            Directive::MemoryAllocation { name, size } => {
                assert_eq!(name, "delay");
                assert_eq!(*size, 4096);
            }
            _ => panic!("Wrong directive"),
        }
    }
}
