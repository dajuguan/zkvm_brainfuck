use bf_vm::matrix::Matrix;
use halo2_proofs::{plonk::{TableColumn, ConstraintSystem, Error}, halo2curves::bn256::Fr, circuit::{layouter, Layouter, Value}};
#[derive(Debug,Clone)]
pub struct ProgramTable{
    instruction: TableColumn
}

impl ProgramTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let instruction = cs.lookup_table_column();
        ProgramTable {instruction}
    }
    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error>{
        layouter.assign_table( || "program table", |mut t| {
            let program = &matrix.program;
            for i in 0..program.len() {
                t.assign_cell(
                    || "program cell" ,
                 self.instruction,
                  i, 
                  || Value::known(program[i]))?;
            }
            Ok(())
        })
    }
}