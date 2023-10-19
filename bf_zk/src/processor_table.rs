use bf_vm::matrix::Matrix;
use halo2_proofs::{plonk::{TableColumn, ConstraintSystem, Error}, halo2curves::bn256::Fr, circuit::{Layouter, Value}};

#[derive(Debug,Clone)]
pub struct ProcessorTable {
    cycle: TableColumn,
    instruction_pointer: TableColumn,
    current_instruction: TableColumn,
    next_instruction: TableColumn,
    memory_pointer: TableColumn,
    memory_value: TableColumn,
    memory_value_inverse: TableColumn
}

impl ProcessorTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let cycle = cs.lookup_table_column();
        let instruction_pointer = cs.lookup_table_column();
        let current_instruction = cs.lookup_table_column();
        let next_instruction = cs.lookup_table_column();
        let memory_pointer = cs.lookup_table_column();
        let memory_value = cs.lookup_table_column();
        let memory_value_inverse = cs.lookup_table_column();
        ProcessorTable {cycle, instruction_pointer, current_instruction, next_instruction, memory_pointer, memory_value, memory_value_inverse}
    }
    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error> {
        let processor_mat = &matrix.processor_matrix;
        layouter.assign_table( || "cycle table", |mut t| {
            for i in 0..processor_mat.len() {
                t.assign_cell(
                    || "cycle cell" ,
                 self.cycle,
                  i, 
                  || Value::known(processor_mat[i].cycle))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "instruction pointer table", |mut t| {
            for i in 0..processor_mat.len() {
                t.assign_cell(
                    || "instruction pointer cell" ,
                 self.instruction_pointer,
                  i, 
                  || Value::known(processor_mat[i].instruction_pointer))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "current_instruction table", |mut t| {
            for i in 0..processor_mat.len() {
                t.assign_cell(
                    || "current_instruction cell" ,
                 self.current_instruction,
                  i, 
                  || Value::known(processor_mat[i].current_instruction))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "next_instruction table", |mut t| {
            for i in 0..processor_mat.len() {
                t.assign_cell(
                    || "next_instruction cell" ,
                 self.next_instruction,
                  i, 
                  || Value::known(processor_mat[i].next_instruction))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "memory_pointer table", |mut t| {
            for i in 0..processor_mat.len() {
                t.assign_cell(
                    || "memory_pointer cell" ,
                 self.memory_pointer,
                  i, 
                  || Value::known(processor_mat[i].memory_pointer))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "memory_value table", |mut t| {
            for i in 0..processor_mat.len() {
                t.assign_cell(
                    || "memory_value cell" ,
                 self.memory_value,
                  i, 
                  || Value::known(processor_mat[i].memory_value))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "memory_value_inverse table", |mut t| {
            for i in 0..processor_mat.len() {
                t.assign_cell(
                    || "memory_value_inverse cell" ,
                 self.memory_value_inverse,
                  i, 
                  || Value::known(processor_mat[i].memory_value_inverse))?;
            }
            Ok(())
        })?;

        Ok(())
    }
}