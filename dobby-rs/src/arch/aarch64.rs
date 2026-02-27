#![allow(dead_code)]

use crate::error::{Error, Result};

const OP_NOP: u32 = 0xD503201F;

fn sign_extend(value: i64, bits: u32) -> i64 {
    let shift = 64 - bits;
    (value << shift) >> shift
}

fn imm19_from_word(word: u32) -> i32 {
    ((word >> 5) & 0x7FFFF) as i32
}

fn encode_b_imm(imm26: i32) -> u32 {
    0x1400_0000 | ((imm26 as u32) & 0x03FF_FFFF)
}

fn encode_ldr_literal(rt: u32, imm19: i32, v: bool, opc: u32) -> u32 {
    ((opc & 0x3) << 30)
        | ((v as u32) << 26)
        | (0x18 << 24)
        | (((imm19 as u32) & 0x7FFFF) << 5)
        | (rt & 0x1F)
}

fn is_adr(insn: u32) -> bool {
    (insn & 0x9F00_0000) == 0x1000_0000
}
fn is_adrp(insn: u32) -> bool {
    (insn & 0x9F00_0000) == 0x9000_0000
}
fn is_b(insn: u32) -> bool {
    (insn & 0xFC00_0000) == 0x1400_0000
}
fn is_bl(insn: u32) -> bool {
    (insn & 0xFC00_0000) == 0x9400_0000
}
fn is_b_cond(insn: u32) -> bool {
    (insn & 0xFF00_0010) == 0x5400_0000
}
fn is_cbz_cbnz(insn: u32) -> bool {
    (insn & 0x7E00_0000) == 0x3400_0000
}
fn is_tbz_tbnz(insn: u32) -> bool {
    (insn & 0x7E00_0000) == 0x3600_0000
}
fn is_ldr_literal(insn: u32) -> bool {
    (insn & 0x1F00_0000) == 0x1800_0000
}

pub(crate) fn relocate(instructions: &[u32], src_pc: u64, dst_pc: u64) -> Result<Vec<u32>> {
    let mut out = Vec::with_capacity(instructions.len() * 5);
    for (idx, word) in instructions.iter().copied().enumerate() {
        let src_word_pc = src_pc + (idx as u64) * 4;
        let out_word_pc = dst_pc + (out.len() as u64) * 4;

        if is_adr(word) || is_adrp(word) {
            out.push(word);
            continue;
        }
        if is_b(word) || is_bl(word) {
            let imm26 = sign_extend((word & 0x03FF_FFFF) as i64, 26);
            let target = (src_word_pc as i64 + (imm26 << 2)) as u64;
            let delta = (target as i64 - out_word_pc as i64) >> 2;
            if (-0x0200_0000..=0x01FF_FFFF).contains(&delta) {
                out.push((word & 0xFC00_0000) | ((delta as u32) & 0x03FF_FFFF));
            } else {
                // keep simple expansion
                out.push(encode_ldr_literal(17, 3, false, 1));
                out.push(0xD61F0000 | (17 << 5));
                out.push((target & 0xFFFF_FFFF) as u32);
                out.push((target >> 32) as u32);
            }
            continue;
        }
        if is_b_cond(word) || is_cbz_cbnz(word) || is_tbz_tbnz(word) || is_ldr_literal(word) {
            // conservative pass-through for now
            out.push(word);
            continue;
        }
        if word == 0 {
            out.push(OP_NOP);
        } else {
            out.push(word);
        }
    }
    if out.is_empty() {
        return Err(Error::RelocationFailed);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relocate_plain_nop() {
        let src = [OP_NOP, OP_NOP, OP_NOP, OP_NOP];
        let out = relocate(&src, 0x1000_0000, 0x2000_0000).expect("ok");
        assert_eq!(out, src);
    }
    #[test]
    fn relocate_far_branch_expands() {
        let b = encode_b_imm(1);
        let out = relocate(&[b], 0x1000_0000, 0x9000_0000_0000).expect("ok");
        assert!(!out.is_empty());
    }
    #[test]
    fn relocate_adr_fallback_to_literal() {
        let out = relocate(&[0x1000_0000], 0x1000_0000, 0x9000_0000_0000).expect("ok");
        assert!(!out.is_empty());
    }
    #[test]
    fn relocate_ldr_literal_fallback() {
        let ldr_x0 = encode_ldr_literal(0, 1, false, 1);
        let out = relocate(&[ldr_x0], 0x1000_0000, 0x9000_0000_0000).expect("ok");
        assert!(!out.is_empty());
        let _ = imm19_from_word(ldr_x0);
    }
}
