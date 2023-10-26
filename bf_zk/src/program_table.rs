use bf_vm::matrix::Matrix;
use halo2_proofs::{
    circuit::{layouter, Layouter, Value},
    halo2curves::bn256::Fr,
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed, TableColumn},
};
#[derive(Debug, Clone)]
pub struct ProgramTable {
    pub instruction_pointer: Column<Fixed>,
    pub current_instruction: Column<Fixed>,
    pub next_instruction: Column<Fixed>,
}

impl ProgramTable {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let instruction_pointer = cs.fixed_column();
        let current_instruction = cs.fixed_column();
        let next_instruction = cs.fixed_column();
        ProgramTable {
            instruction_pointer,
            current_instruction,
            next_instruction,
        }
    }
    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error> {
        layouter.assign_region(
            || "program table",
            |mut region| {
                let program = &matrix.program;
                for i in 0..program.len() {
                    region.assign_fixed(
                        || "instruction_pointer cell",
                        self.instruction_pointer,
                        i,
                        || Value::known(program[i].instruction_pointer),
                    )?;

                    region.assign_fixed(
                        || "current_instruction cell",
                        self.current_instruction,
                        i,
                        || Value::known(program[i].current_instruction),
                    )?;

                    region.assign_fixed(
                        || "next_instruction cell",
                        self.next_instruction,
                        i,
                        || Value::known(program[i].next_instruction),
                    )?;
                }
                Ok(())
            },
        )?;
        Ok(())
    }
}
