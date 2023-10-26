use crate::code;
use crate::matrix::{InstructionMatrixRow, Matrix, MemoryMatrixRow};
use alloc::vec::Vec;
use core::convert::From;
use halo2_proofs::arithmetic::{Field};
use halo2_proofs::halo2curves::{bn256::Fr};
use halo2curves::ff::PrimeField;

#[derive(Clone, Debug, Default)]
pub struct Register {
    pub cycle: Fr,
    pub instruction_pointer: Fr,
    pub current_instruction: Fr,
    pub next_instruction: Fr,
    pub memory_pointer: Fr,
    pub memory_value: Fr,
    pub memory_value_inverse: Fr,
}

trait FieldExt {
    fn get_lower_128(&self) -> u128;
}

impl FieldExt for Fr {
    fn get_lower_128(&self) -> u128 {
        let to_limbs = |e: &Fr| {
            let repr = e.to_repr();
            let repr = repr.as_ref();
            let tmp0 = u64::from_le_bytes(repr[0..8].try_into().unwrap());
            let tmp1 = u64::from_le_bytes(repr[8..16].try_into().unwrap());
            let tmp2 = u64::from_le_bytes(repr[16..24].try_into().unwrap());
            let tmp3 = u64::from_le_bytes(repr[24..32].try_into().unwrap());
            [tmp0, tmp1, tmp2, tmp3]
        };
    
        let e = to_limbs(self);
        u128::from(e[0]) | (u128::from(e[1]) << 64)
    }
}

impl Register {
    fn ip(&self) -> usize {
        self.instruction_pointer.get_lower_128()  as usize
    }

    fn mp(&self) -> usize {
        self.memory_pointer.get_lower_128()  as usize
    }
}

pub struct Interpreter {
    pub code: Vec<Fr>,
    pub input: Vec<Fr>,
    pub memory: Vec<Fr>,
    pub register: Register,
    pub matrix: Matrix,
    pub bits: u64,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            input: Vec::new(),
            memory: vec![Fr::zero()],
            register: Register::default(),
            matrix: Matrix::default(),
            bits: 8,
        }
    }

    pub fn set_code(&mut self, code: Vec<Fr>) {
        self.code = code.clone();
        let mut program = vec![];
        for i in 0..code.len() {
           
            if i > 0 && (code[i-1] == ('[' as u64).into() || code[i-1] == (']' as u64).into()) {
                let ip = code[i].get_lower_128() as usize;
                let current_instruction = if  ip == code.len() {
                    Fr::from(0)
                } else {
                    code[ip]
                };

                let next_instruction = if ip + 1 >= code.len() {
                    Fr::from(0)
                } else {
                    code[ip + 1]
                };

                program.push(InstructionMatrixRow{
                    instruction_pointer: code[i],
                    current_instruction,
                    next_instruction
                });
                continue;
            }
            let next_instruction = if i == code.len() -1 {
                Fr::from(0)
            } else {
                code[i+1]
            };
            program.push(InstructionMatrixRow{
                instruction_pointer: Fr::from(i as u64),
                current_instruction: code[i],
                next_instruction
            });
        }

        self.matrix.program = program


    }

    pub fn set_input(&mut self, input: Vec<Fr>) {
        self.input = input;
    }

    pub fn set_bits(&mut self, bits: u64) {
        self.bits = bits
    }

    pub fn run(&mut self) {
        self.register.current_instruction = self.code[0];
        if self.code.len() == 1 {
            self.register.next_instruction = Fr::zero()
        } else {
            self.register.next_instruction = self.code[1];
        }
        for i in 0..self.code.len() {
            self.matrix.instruction_matrix.push(InstructionMatrixRow {
                instruction_pointer: Fr::from(i as u64),
                current_instruction: self.code[i],
                next_instruction: if i == self.code.len() - 1 {
                    Fr::zero()
                } else {
                    self.code[i + 1]
                },
            });
        }
        loop {
            if self.register.instruction_pointer >= Fr::from(self.code.len() as u64) {
                break;
            }
            self.matrix.processor_matrix.push(self.register.clone());
            self.matrix.instruction_matrix.push(InstructionMatrixRow::from(&self.register));
            self.matrix.memory_matrix.push(MemoryMatrixRow::from(&self.register));
            match self.register.current_instruction.get_lower_128() as u8 {
                code::SHL => {
                    self.register.memory_pointer -= Fr::one();
                    self.register.instruction_pointer += Fr::one();
                }
                code::SHR => {
                    self.register.memory_pointer += Fr::one();
                    if self.register.mp() == self.memory.len() {
                        self.memory.push(Fr::zero())
                    }
                    self.register.instruction_pointer += Fr::one();
                }
                code::ADD => {
                    if self.memory[self.register.mp()] == Fr::from((1 << self.bits) - 1) {
                        self.memory[self.register.mp()] = Fr::zero()
                    } else {
                        self.memory[self.register.mp()] = self.memory[self.register.mp()] + Fr::one();
                    }
                    self.register.instruction_pointer += Fr::one();
                }
                code::SUB => {
                    if self.memory[self.register.mp()] == Fr::zero() {
                        self.memory[self.register.mp()] = Fr::from((1 << self.bits) - 1)
                    } else {
                        self.memory[self.register.mp()] = self.memory[self.register.mp()] - Fr::one();
                    }
                    self.register.instruction_pointer += Fr::one();
                }
                code::GETCHAR => {
                    let val = self.input.remove(0);
                    self.memory[self.register.mp()] = val;
                    self.matrix.input_matrix.push(val);
                    self.register.instruction_pointer += Fr::one();
                }
                code::PUTCHAR => {
                    self.matrix.output_matrix.push(self.register.memory_value);
                    self.register.instruction_pointer += Fr::one();
                }
                code::LB => {
                    if self.memory[self.register.mp()] == Fr::zero() {
                        self.register.instruction_pointer = self.code[self.register.ip() + 1];
                    } else {
                        self.register.instruction_pointer += Fr::from(2);
                    }
                }
                code::RB => {
                    if self.memory[self.register.mp()] != Fr::zero() {
                        self.register.instruction_pointer = self.code[self.register.ip() + 1];
                    } else {
                        self.register.instruction_pointer += Fr::from(2);
                    }
                }
                _ => unreachable!(),
            }
            self.register.cycle += Fr::one();
            if self.register.instruction_pointer < Fr::from(self.code.len() as u64) {
                self.register.current_instruction = self.code[self.register.ip()];
            } else {
                self.register.current_instruction = Fr::zero();
            }
            if self.register.instruction_pointer < Fr::from(self.code.len() as u64) - Fr::one() {
                self.register.next_instruction = self.code[self.register.ip() + 1];
            } else {
                self.register.next_instruction = Fr::zero()
            }
            self.register.memory_value = self.memory[self.register.mp()];
            self.register.memory_value_inverse = if self.register.memory_value == Fr::zero() {
                Fr::zero()
            } else {
                self.register.memory_value.invert().unwrap()
            };
        }
        self.matrix.processor_matrix.push(self.register.clone());
        self.matrix.memory_matrix.push(MemoryMatrixRow::from(&self.register));
        self.matrix.instruction_matrix.push(InstructionMatrixRow::from(&self.register));
        self.matrix.instruction_matrix.sort_by_key(|row| row.instruction_pointer);
        self.matrix.memory_matrix.sort_by_key(|row| row.memory_pointer);

        // Append dummy memory rows
        // let mut i = 1;
        // while i < self.matrix.memory_matrix.len() - 1 {
        //     if self.matrix.memory_matrix[i + 1].memory_pointer == self.matrix.memory_matrix[i].memory_pointer
        //         && self.matrix.memory_matrix[i + 1].cycle != self.matrix.memory_matrix[i].cycle + Fr::one()
        //     {
        //         let interleaved_value = MemoryMatrixRow {
        //             cycle: self.matrix.memory_matrix[i].cycle + Fr::one(),
        //             memory_pointer: self.matrix.memory_matrix[i].memory_pointer,
        //             memory_value: self.matrix.memory_matrix[i].memory_value,
        //             interweave_indicator: Fr::one(),
        //         };
        //         self.matrix.memory_matrix.insert(i + 1, interleaved_value);
        //     }
        //     i += 1;
        // }
    }
}