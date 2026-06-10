/// x86-64 code generation.
///
/// Each `emit_*` function appends machine bytes to a `Vec<u8>`.
/// Add new emitters here as the compiler grows (arithmetic, calls, etc.).

use alloc::vec::Vec;

/// `MOV RAX, imm64` + `RET` — the simplest possible function body.
///
/// Encoding:
///   48 B8 <imm64-le>   REX.W MOV RAX, imm64
///   C3                 RET
pub fn emit_return_u64(value: u64) -> Vec<u8> {
    let mut code = Vec::with_capacity(11);
    code.push(0x48);        // REX.W
    code.push(0xB8);        // MOV RAX, imm64
    code.extend_from_slice(&value.to_le_bytes());
    code.push(0xC3);        // RET
    code
}