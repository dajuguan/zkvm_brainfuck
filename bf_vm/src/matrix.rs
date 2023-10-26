extern crate alloc;
use alloc::vec::Vec;

use crate::interpreter::Register;
use core::convert::From;
use halo2_proofs::halo2curves::bn256::Fr;

#[derive(Default)]
pub struct Matrix {
    pub processor_matrix: Vec<Register>,
    pub instruction_matrix: Vec<InstructionMatrixRow>,
    pub memory_matrix: Vec<MemoryMatrixRow>,
    pub input_matrix: Vec<Fr>,
    pub output_matrix: Vec<IOMatrixRow>,
    pub program: Vec<InstructionMatrixRow>,
}

#[derive(Debug, Clone)]
pub struct InstructionMatrixRow {
    pub instruction_pointer: Fr,
    pub current_instruction: Fr,
    pub next_instruction: Fr,
}

impl From<&Register> for InstructionMatrixRow {
    fn from(r: &Register) -> Self {
        Self {
            instruction_pointer: r.instruction_pointer,
            current_instruction: r.current_instruction,
            next_instruction: r.next_instruction,
        }
    }
}

pub struct MemoryMatrixRow {
    pub cycle: Fr,
    pub memory_pointer: Fr,
    pub memory_value: Fr,
    pub interweave_indicator: Fr,
}

impl From<&Register> for MemoryMatrixRow {
    fn from(r: &Register) -> Self {
        Self {
            cycle: r.cycle,
            memory_pointer: r.memory_pointer,
            memory_value: r.memory_value,
            interweave_indicator: Fr::zero(),
        }
    }
}

#[derive(Debug)]
pub struct IOMatrixRow {
    pub cycle: Fr,
    pub value: Fr,
}
