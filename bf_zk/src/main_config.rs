use halo2_proofs::{circuit::*, plonk::*, poly::Rotation, halo2curves::bn256::Fr};
use bf_vm::matrix::Matrix;
use crate::program_table::ProgramTable;
use crate::instruction_table::InstructionTable;
use crate::memory_table::MemoryTable;
use crate::processor_table::ProcessorTable;

#[derive(Debug,Clone)]
pub struct MainConfig{
    program_conf: ProgramTable,
    processor_conf: ProcessorTable,
    mem_conf: MemoryTable,
    ins_conf: InstructionTable
}


impl MainConfig {
    fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let program_conf = ProgramTable::configure(cs);
        let processor_conf = ProcessorTable::configure(cs);
        let mem_conf = MemoryTable::configure(cs);
        let ins_conf = InstructionTable::configure(cs);

        MainConfig { program_conf, processor_conf, mem_conf, ins_conf }
    }

    fn assign(&mut self, mut layouter: impl Layouter<Fr>,matrix: &Matrix) {
        self.program_conf.load(layouter.namespace(|| "program table"), matrix);
        self.processor_conf.load(layouter.namespace(|| "processor table"), matrix);
        self.mem_conf.load(layouter.namespace(|| "memory table"), matrix);
        self.ins_conf.load(layouter.namespace(|| "instuction table"), matrix)
    }
}



#[derive(Default)]
pub struct VMCircuit {
    pub matrix: Matrix
}

impl Circuit<Fr> for VMCircuit {
    type Config = MainConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        VMCircuit::default()
    }
    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        MainConfig::configure(meta)
    }

    fn synthesize(&self, mut config: Self::Config, layouter: impl Layouter<Fr>) -> Result<(), Error> {
        config.assign(layouter, &self.matrix);
        Ok(())
    }
}