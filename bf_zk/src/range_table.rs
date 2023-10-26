use halo2_proofs::circuit::{Layouter, Value};
use halo2_proofs::halo2curves::bn256::Fr;
use halo2_proofs::plonk::*;

pub trait RangeTable {
    fn configure(cs: &mut ConstraintSystem<Fr>) -> Self;
    fn load_table(&self, layouter: &mut impl Layouter<Fr>) -> Result<(), Error>;
}

#[derive(Clone, Debug, Copy)]
pub struct RangeTableConfig<const RANGE: usize> {
    pub table: TableColumn,
}

impl<const RANGE: usize> RangeTable for RangeTableConfig<RANGE> {
    fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let table = cs.lookup_table_column();
        Self { table }
    }

    fn load_table(&self, layouter: &mut impl Layouter<Fr>) -> Result<(), Error> {
        layouter.assign_table(
            || "load range-check table",
            |mut table| {
                let mut offset = 0;
                for value in 0 as i64..1 << RANGE {
                    table.assign_cell(
                        || "value",
                        self.table,
                        offset,
                        || Value::known(Fr::from(value as u64)),
                    )?;
                    offset += 1;
                }

                Ok(())
            },
        )
    }
}
