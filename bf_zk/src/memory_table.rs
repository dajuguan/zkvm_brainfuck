use bf_vm::matrix::Matrix;
use halo2_proofs::{plonk::{TableColumn, ConstraintSystem}, halo2curves::bn256::Fr, circuit::Layouter};

#[derive(Debug,Clone)]
pub struct  MemoryTable {
    table: TableColumn
}

impl MemoryTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let table = cs.lookup_table_column();
        MemoryTable {table}
    }
    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) {

    }
}