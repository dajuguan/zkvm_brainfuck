use bf_vm::{
    code::{GETCHAR, PUTCHAR},
    matrix::Matrix,
};
use halo2_proofs::{
    circuit::{layouter, Layouter, Value},
    halo2curves::bn256::Fr,
    plonk::{Advice, Column, ConstraintSystem, Error, Expression, Fixed, Instance, Selector},
    poly::Rotation,
};

#[derive(Debug, Clone)]
pub struct InputTable {
    pub clk: Column<Advice>,
    pub diff: Column<Advice>, // diff cell must >= 0, it's range checked in processor table
    pub value: Column<Instance>,
    pub s_diff: Selector, //selector for clk check
}

impl InputTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let clk = cs.advice_column();
        let ci = cs.fixed_column();
        let value = cs.instance_column();
        let diff = cs.advice_column(); //aux col to check clk_next > clk
        let s_diff = cs.selector();

        cs.enable_equality(clk);
        cs.enable_equality(ci);
        cs.enable_equality(value);
        cs.enable_equality(diff);

        cs.create_gate("input table next clk > clk", |meta| {
            let cur_clk = meta.query_advice(clk, Rotation::cur());
            let next_clk = meta.query_advice(clk, Rotation::next());
            let diff = meta.query_advice(diff, Rotation::next());
            let s = meta.query_selector(s_diff);
            vec![s * (cur_clk + Expression::Constant(Fr::one()) + diff - next_clk)]
        });

        InputTable {
            clk,
            value,
            diff,
            s_diff,
        }
    }

    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error> {
        let input_mat = &matrix.input_matrix;
        layouter.assign_region(
            || "input table",
            |mut region| {
                for i in 0..input_mat.len() {
                    if i < input_mat.len() - 1 {
                        self.s_diff.enable(&mut region, i)?;
                    }
                    region.assign_advice(
                        || "clk cell",
                        self.clk,
                        i,
                        || Value::known(input_mat[i].cycle),
                    )?;

                    region.assign_advice(
                        || "diff cell",
                        self.diff,
                        i,
                        || Value::known(input_mat[i].diff),
                    )?;
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}
