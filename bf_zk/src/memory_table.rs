use bf_vm::matrix::Matrix;
use halo2_proofs::{plonk::{TableColumn, ConstraintSystem, Error}, halo2curves::bn256::Fr, circuit::{Layouter, Value}};

#[derive(Debug,Clone)]
pub struct  MemoryTable {
    cycle: TableColumn,
    memory_pointer: TableColumn,
    memory_value: TableColumn,
    interweave_indicator: TableColumn
}

impl MemoryTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let cycle = cs.lookup_table_column();
        let memory_pointer = cs.lookup_table_column();
        let memory_value = cs.lookup_table_column();
        let interweave_indicator = cs.lookup_table_column();
        MemoryTable {cycle, memory_pointer, memory_value, interweave_indicator}
    }
    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error>{
        let mem_mat = &matrix.memory_matrix;
        layouter.assign_table( || "cycle table", |mut t| {
            for i in 0..mem_mat.len() {
                t.assign_cell(
                    || "cycle cell" ,
                 self.cycle,
                  i, 
                  || Value::known(mem_mat[i].cycle))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "memory_pointer table", |mut t| {
            for i in 0..mem_mat.len() {
                t.assign_cell(
                    || "memory_pointer cell" ,
                 self.memory_pointer,
                  i, 
                  || Value::known(mem_mat[i].memory_pointer))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "memory_value table", |mut t| {
            for i in 0..mem_mat.len() {
                t.assign_cell(
                    || "memory_value cell" ,
                 self.memory_value,
                  i, 
                  || Value::known(mem_mat[i].memory_value))?;
            }
            Ok(())
        })?;

        layouter.assign_table( || "interweave_indicator table", |mut t| {
            for i in 0..mem_mat.len() {
                t.assign_cell(
                    || "interweave_indicator cell" ,
                 self.interweave_indicator,
                  i, 
                  || Value::known(mem_mat[i].interweave_indicator))?;
            }
            Ok(())
        })?;

        Ok(())
    }
}