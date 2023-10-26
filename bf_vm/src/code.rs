extern crate alloc;

use alloc::vec::Vec;
use core::convert::From;
use halo2_proofs::halo2curves::bn256::Fr;

pub const SHL: u8 = 0x3C;
pub const SHR: u8 = 0x3E;
pub const ADD: u8 = 0x2B;
pub const SUB: u8 = 0x2D;
pub const GETCHAR: u8 = 0x2C;
pub const PUTCHAR: u8 = 0x2E;
pub const LB: u8 = 0x5B;
pub const RB: u8 = 0x5D;

pub fn easygen(code: &str) -> Vec<Fr> {
    code.as_bytes()
        .iter()
        .map(|&x| Fr::from(x as u64))
        .collect()
}

/**
 * Alan's implementation employs a direct target address for jump operations,
 * causing incompatibility with u8 for larger programs.
 * Using u16 is a temporary measure to accommodate all test cases.
 * Once hash-based public input verification is implemented, this can be removed.
 */
pub fn compile_to_u16(code: Vec<u8>) -> Vec<u16> {
    let filter = vec![SHL, SHR, ADD, SUB, GETCHAR, PUTCHAR, LB, RB];
    let mut instrs = Vec::<u16>::new();
    let mut jstack = Vec::<u16>::new();
    for i in code {
        if !filter.contains(&i) {
            continue;
        }
        instrs.push(i as u16);
        if i == LB {
            instrs.push(0);
            jstack.push(instrs.len() as u16 - 1);
        }
        if i == RB {
            instrs.push(*jstack.last().unwrap() + 1);
            instrs[*jstack.last().unwrap() as usize] = instrs.len() as u16;
            jstack.pop();
        }
    }
    return instrs;
}

pub fn compile(code: Vec<u8>) -> Vec<Fr> {
    compile_to_u16(code)
        .into_iter()
        .map(|x| Fr::from(x as u64))
        .collect()
}
