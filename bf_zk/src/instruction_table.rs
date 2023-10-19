use bf_vm::matrix::Matrix;
use halo2_proofs::{plonk::{TableColumn, ConstraintSystem}, halo2curves::bn256::Fr, circuit::{Layouter, Value}};

#[derive(Debug,Clone)]
pub struct InstructionTable {
    instruction_pointer: TableColumn,
    current_instruction: TableColumn,
    next_instruction: TableColumn
}

impl InstructionTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let instruction_pointer = cs.lookup_table_column();
        let current_instruction = cs.lookup_table_column();
        let next_instruction = cs.lookup_table_column();
        InstructionTable {instruction_pointer, current_instruction, next_instruction}
    }
    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), halo2_proofs::plonk::Error> {
        let instruction_mat = &matrix.instruction_matrix;
        layouter.assign_table( || "instruction pointer table", |mut t| {
            for i in 0..instruction_mat.len() {
                t.assign_cell(
                    || "instruction pointer cell" ,
                 self.instruction_pointer,
                  i, 
                  || Value::known(instruction_mat[i].instruction_pointer))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "current_instruction table", |mut t| {
            for i in 0..instruction_mat.len() {
                t.assign_cell(
                    || "current_instruction cell" ,
                 self.current_instruction,
                  i, 
                  || Value::known(instruction_mat[i].current_instruction))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "next_instruction table", |mut t| {
            for i in 0..instruction_mat.len() {
                t.assign_cell(
                    || "next_instruction cell" ,
                 self.next_instruction,
                  i, 
                  || Value::known(instruction_mat[i].next_instruction))?;
            }
            Ok(())
        })?;

        Ok(())
    }
}