use bf_vm::matrix::Matrix;
use halo2_proofs::{
    circuit::{Layouter, Value},
    halo2curves::bn256::Fr,
    plonk::{
        Advice, Column, ConstraintSystem, Constraints, Error, Expression, Selector, TableColumn,
    },
    poly::Rotation,
};

#[derive(Debug, Clone)]
pub struct MemoryTable {
    pub clk: Column<Advice>,
    pub memory_pointer: Column<Advice>,
    pub memory_value: Column<Advice>,
    pub s_m: Selector,
}

impl MemoryTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let clk = cs.advice_column();
        let memory_pointer = cs.advice_column();
        let memory_value = cs.advice_column();
        let s_m = cs.selector();

        cs.enable_equality(clk);
        cs.enable_equality(memory_pointer);
        cs.enable_equality(memory_value);

        let one = Expression::Constant(Fr::one());

        cs.create_gate("mem gates", |meta| {
            let sm = meta.query_selector(s_m);
            let cur_mp = meta.query_advice(memory_pointer, Rotation::cur());
            let next_mp = meta.query_advice(memory_pointer, Rotation::next());
            let cur_mv = meta.query_advice(memory_value, Rotation::cur());
            let next_mv = meta.query_advice(memory_value, Rotation::next());
            let cur_clk = meta.query_advice(clk, Rotation::cur());
            let next_clk = meta.query_advice(clk, Rotation::next());

            // M0: memory pointer either increase by one or by zero
            let m0 = (next_mp.clone() - cur_mp.clone() - one.clone())
                * (next_mp.clone() - cur_mp.clone());
            // M1: If mp increases by 1, then mv must be set to zero
            let m1 = (next_mp.clone() - cur_mp.clone()) * next_mv.clone();
            // M2: when the memory pointer does not change:
            // a) memory value remains the same or b) the cycle count only increases by one
            let m2 = (next_mp.clone() - cur_mp.clone() - one.clone())
                * (cur_mv.clone() - next_mv.clone())
                * (next_clk - cur_clk.clone() - one.clone());

            Constraints::with_selector(sm, vec![m0, m1, m2])
        });

        MemoryTable {
            clk,
            memory_pointer,
            memory_value,
            s_m,
        }
    }
    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error> {
        let mem_mat = &matrix.memory_matrix;
        layouter.assign_region(
            || "mem table",
            |mut region| {
                for i in 0..mem_mat.len() {
                    if i < mem_mat.len() - 1 {
                        self.s_m.enable(&mut region, i)?;
                    }
                    region.assign_advice(
                        || "clk cell",
                        self.clk,
                        i,
                        || Value::known(mem_mat[i].cycle),
                    )?;
                    region.assign_advice(
                        || "mem pointer cell",
                        self.memory_pointer,
                        i,
                        || Value::known(mem_mat[i].memory_pointer),
                    )?;
                    region.assign_advice(
                        || "mem value cell",
                        self.memory_value,
                        i,
                        || Value::known(mem_mat[i].memory_value),
                    )?;
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}
