use bf_vm::{matrix::Matrix, code::PUTCHAR};
use halo2_proofs::{
    circuit::{layouter, Layouter, Value},
    halo2curves::bn256::Fr,
    plonk::{Advice, Column, ConstraintSystem, Error, Instance, Fixed, Selector, Expression}, poly::Rotation,
};

#[derive(Debug, Clone)]
pub struct OutputTable {
    pub clk: Column<Advice>,
    pub ci: Column<Fixed>,
    pub diff: Column<Advice>,  // diff cell must >= 0, it's range checked in processor table
    pub value: Column<Instance>,
    pub s_diff: Selector  //selector for clk check
}

impl OutputTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let clk = cs.advice_column();
        let ci = cs.fixed_column();
        let value = cs.instance_column();
        let diff =  cs.advice_column(); //aux col to check clk_next > clk
        let s_diff = cs.selector();

        cs.enable_equality(clk);
        cs.enable_equality(ci);
        cs.enable_equality(value);
        cs.enable_equality(diff);

        cs.create_gate("next clk > clk", |meta|{
            let cur_clk = meta.query_advice(clk, Rotation::cur());
            let next_clk = meta.query_advice(clk, Rotation::next());
            let diff = meta.query_advice(diff, Rotation::next());
            let s = meta.query_selector(s_diff);
            vec![s*(cur_clk + Expression::Constant(Fr::one()) + diff - next_clk)]
        });

        OutputTable { clk, ci, value, diff, s_diff }
    }

    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error> {
        let output_mat = &matrix.output_matrix;
        layouter.assign_region(
            || "output table",
            |mut region| {
                for i in 0..output_mat.len() {
                    if i < output_mat.len() - 1{
                        self.s_diff.enable(&mut region, i)?;
                    }
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

                    region.assign_advice(
                        || "diff cell",
                        self.diff,
                        i,
                        || Value::known(output_mat[i].diff),
                    )?;
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}
