use bf_vm::{matrix::Matrix, code::PUTCHAR};
use halo2_proofs::{
    circuit::{layouter, Layouter, Value},
    halo2curves::bn256::Fr,
    plonk::{Advice, Column, ConstraintSystem, Error, Instance, Fixed},
};

#[derive(Debug, Clone)]
pub struct OutputTable {
    pub clk: Column<Advice>,
    pub ci: Column<Fixed>,
    pub value: Column<Instance>,
}

impl OutputTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let clk = cs.advice_column();
        let ci = cs.fixed_column();
        let value = cs.instance_column();

        cs.enable_equality(clk);
        cs.enable_equality(ci);
        cs.enable_equality(value);

        OutputTable { clk, ci, value }
    }

    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error> {
        let output_mat = &matrix.output_matrix;
        layouter.assign_region(
            || "output table",
            |mut region| {
                for i in 0..output_mat.len() {
                    region.assign_advice(
                        || "clk cell",
                        self.clk,
                        i,
                        || Value::known(output_mat[i].cycle),
                    )?;

                    region.assign_fixed(
                        || "ci cell",
                        self.ci,
                        i,
                        || Value::known(Fr::from(PUTCHAR as u64)),
                    )?;
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}
