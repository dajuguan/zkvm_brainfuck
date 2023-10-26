use std::marker::PhantomData;

use crate::instruction_table::InstructionTable;
use crate::memory_table::MemoryTable;
use crate::output_table::OutputTable;
use crate::processor_table::ProcessorTable;
use crate::program_table::ProgramTable;
use crate::utilts::PUTCHAR;
use bf_vm::matrix::Matrix;
use halo2_proofs::arithmetic::Field;
use halo2_proofs::{circuit::*, halo2curves::bn256::Fr, plonk::*, poly::Rotation};

#[derive(Debug, Clone)]
pub struct MainConfig<const RANGE: usize> {
    program_conf: ProgramTable,
    processor_conf: ProcessorTable<RANGE>,
    mem_conf: MemoryTable,
    ins_conf: InstructionTable,
    output_conf: OutputTable,
}

impl<const RANGE: usize> MainConfig<RANGE> {
    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self {
        let program_conf = ProgramTable::configure(meta);
        let processor_conf = ProcessorTable::configure(meta);
        let mem_conf = MemoryTable::configure(meta);
        let ins_conf = InstructionTable::configure(meta);
        let output_conf = OutputTable::configure(meta);

        meta.lookup_any("program lookup", |meta| {
            let program_ci = meta.query_fixed(program_conf.current_instruction, Rotation::cur());
            let processor_ci =
                meta.query_advice(processor_conf.current_instruction, Rotation::cur());
            let program_ip = meta.query_fixed(program_conf.instruction_pointer, Rotation::cur());
            let processor_ip =
                meta.query_advice(processor_conf.instruction_pointer, Rotation::cur());
            let program_ni = meta.query_fixed(program_conf.next_instruction, Rotation::cur());
            let processor_ni = meta.query_advice(processor_conf.next_instruction, Rotation::cur());

            vec![
                (program_ci, processor_ci),
                (program_ip, processor_ip),
                (program_ni, processor_ni),
            ]
        });

        meta.lookup_any("memory lookup", |meta| {
            let mem_clk = meta.query_advice(mem_conf.clk, Rotation::cur());
            let processor_clk = meta.query_advice(processor_conf.clk, Rotation::cur());
            let mem_mp = meta.query_advice(mem_conf.memory_pointer, Rotation::cur());
            let processor_mp = meta.query_advice(processor_conf.memory_pointer, Rotation::cur());
            let mem_mv = meta.query_advice(mem_conf.memory_value, Rotation::cur());
            let processor_mv = meta.query_advice(processor_conf.memory_value, Rotation::cur());

            vec![
                (mem_clk, processor_clk),
                (mem_mp, processor_mp),
                (mem_mv, processor_mv),
            ]
        });

        meta.lookup_any("output lookup", |meta| {
            let processor_clk = meta.query_advice(processor_conf.clk, Rotation::cur());
            let output_clk = meta.query_advice(output_conf.clk, Rotation::cur());
            let processor_ci =
                meta.query_advice(processor_conf.current_instruction, Rotation::cur());
            let output_ci = meta.query_fixed(output_conf.ci, Rotation::cur());
            let processor_mv = meta.query_advice(processor_conf.memory_value, Rotation::cur());
            let output_val = meta.query_instance(output_conf.value, Rotation::cur());
            vec![
                (output_clk, processor_clk),
                (output_ci, processor_ci),
                (output_val, processor_mv),
            ]
        });

        MainConfig {
            program_conf,
            processor_conf,
            mem_conf,
            ins_conf,
            output_conf,
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
        self.output_conf
            .load(layouter.namespace(|| "output layouter"), matrix)?;
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
