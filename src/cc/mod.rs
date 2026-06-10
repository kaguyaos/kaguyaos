//! Tiny C JIT compiler.
//!
//! Public API: [`compile_and_run`].
//!
//! Internal layout:
//!   lexer   — source text → tokens
//!   parser  — tokens → AST / values
//!   codegen — values → x86-64 machine bytes

pub mod lexer;
pub mod parser;
pub mod codegen;

use alloc::string::String;
use crate::tinyasm::jit::JitMemory;

/// Compile a tiny C function and immediately execute it, returning the result.
///
/// # Supported grammar
/// ```c
/// uint64_t <name>() { return <integer>; }
/// ```
///
/// # Example
/// ```
/// let result = compile_and_run("uint64_t answer() { return 42; }").unwrap();
/// assert_eq!(result, 42);
/// ```
pub fn compile_and_run(src: &str) -> Result<u64, String> {
    // 1. Lex
    let tokens = lexer::lex(src)?;

    // 2. Parse
    let return_value = parser::parse_return_value(&tokens)?;

    // 3. Emit machine code
    let code = codegen::emit_return_u64(return_value);

    // 4. Load into executable memory and call
    let mut mem = JitMemory::new(code.len())?;
    mem.write(&code)?;
    mem.make_executable()?;

    let result = unsafe { mem.as_fn_u64()() };
    Ok(result)
}