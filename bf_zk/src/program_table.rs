use bf_vm::matrix::Matrix;
use halo2_proofs::{plonk::{TableColumn, ConstraintSystem}, halo2curves::bn256::Fr, circuit::{layouter, Layouter}};

#[derive(Debug,Clone)]
pub struct ProgramTable{
    table: TableColumn
}

impl ProgramTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let table = cs.lookup_table_column();
        ProgramTable {table}
    }
    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) {

    }
}