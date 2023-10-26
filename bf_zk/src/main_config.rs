use std::marker::PhantomData;

use crate::instruction_table::InstructionTable;
use crate::memory_table::MemoryTable;
use crate::processor_table::ProcessorTable;
use crate::program_table::ProgramTable;
use bf_vm::matrix::Matrix;
use halo2_proofs::arithmetic::Field;
use halo2_proofs::{circuit::*, halo2curves::bn256::Fr, plonk::*, poly::Rotation};

#[derive(Debug, Clone)]
pub struct MainConfig<const RANGE: usize> {
    program_conf: ProgramTable,
    processor_conf: ProcessorTable<RANGE>,
    mem_conf: MemoryTable,
    ins_conf: InstructionTable,
}

impl<const RANGE: usize> MainConfig<RANGE> {
    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self {
        let program_conf = ProgramTable::configure(meta);
        let processor_conf = ProcessorTable::configure(meta);
        let mem_conf = MemoryTable::configure(meta);
        let ins_conf = InstructionTable::configure(meta);

        // meta.lookup_any("program lookup", |meta|{
        //     let inst = meta.query_advice(program_conf.current_instruction, Rotation::cur());
        //     let cur_ins = meta.query_advice(processor_conf.current_instruction, Rotation::cur());
        //     vec![(cur_ins,inst)]
        // });

        MainConfig {
            program_conf,
            processor_conf,
            mem_conf,
            ins_conf,
        }
    }

    fn assign(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error> {
        self.program_conf
            .load(layouter.namespace(|| "program layouter"), matrix)?;
        self.processor_conf
            .load(layouter.namespace(|| "processor layouter"), matrix)?;
        self.mem_conf
            .load(layouter.namespace(|| "memory layouter"), matrix)?;
        self.ins_conf
            .load(layouter.namespace(|| "instuction layouter"), matrix)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct VMCircuit<F: Field, const RANGE: usize> {
    pub matrix: Matrix,
    pub _marker: PhantomData<F>,
}

impl<const RANGE: usize> Circuit<Fr> for VMCircuit<Fr, RANGE> {
    type Config = MainConfig<RANGE>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        VMCircuit::default()
    }
    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        MainConfig::configure(meta)
    }

    fn synthesize(
        &self,
        mut config: Self::Config,
        layouter: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        config.assign(layouter, &self.matrix)?;
        Ok(())
    }
}
