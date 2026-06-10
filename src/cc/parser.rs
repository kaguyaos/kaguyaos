/// Parses the token stream produced by the lexer.
///
/// Currently understands one grammar rule:
///
///   uint64_t <name>() { return <integer>; }
///
/// Extend this file to support richer statements, expressions, and types.

use alloc::string::{String, ToString};
use alloc::format;

use super::lexer::Token;

/// Parse a single function and return its `return` value.
pub fn parse_return_value(tokens: &[Token]) -> Result<u64, String> {
    let mut i = 0;

    // Return-type identifier (we accept any ident, e.g. `uint64_t`)
    match tokens.get(i) {
        Some(Token::Ident(_)) => i += 1,
        _ => return Err("Expected return-type identifier".to_string()),
    }
    // Function name
    match tokens.get(i) {
        Some(Token::Ident(_)) => i += 1,
        _ => return Err("Expected function name".to_string()),
    }
    // ()
    expect(tokens, &mut i, &Token::LParen)?;
    expect(tokens, &mut i, &Token::RParen)?;
    // { return <n>; }
    expect(tokens, &mut i, &Token::LBrace)?;
    expect(tokens, &mut i, &Token::Return)?;
    let value = match tokens.get(i) {
        Some(Token::Number(n)) => { i += 1; *n }
        _ => return Err("Expected integer literal after 'return'".to_string()),
    };
    expect(tokens, &mut i, &Token::Semicolon)?;
    expect(tokens, &mut i, &Token::RBrace)?;

    Ok(value)
}

fn expect(tokens: &[Token], i: &mut usize, expected: &Token) -> Result<(), String> {
    match tokens.get(*i) {
        Some(t) if t == expected => { *i += 1; Ok(()) }
        other => Err(format!("Expected {:?}, got {:?}", expected, other)),
    }
}