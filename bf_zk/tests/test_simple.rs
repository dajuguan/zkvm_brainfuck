use bf_vm::{code, interpreter::Interpreter};
use bf_zk::main_config::VMCircuit;
use halo2_proofs::dev::MockProver;

#[test]
fn test_run() {
    let program = code::compile(include_bytes!("../../res/hello_world.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.run();
    let program = code::compile(include_bytes!("../../res/hello_world.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.run(); 
    println!("{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
        vm.matrix.program.len(),
        vm.matrix.input_matrix.len(),
        vm.matrix.instruction_matrix.len(),
        vm.matrix.memory_matrix.len(),
        vm.matrix.processor_matrix.len(),
        vm.matrix.output_matrix.len(),
        vm.matrix.input_matrix.len()
    );

    let circuit = VMCircuit{matrix: vm.matrix};
    println!("program:{:?}", circuit.matrix.program);
}

#[test]
fn test_vmcircuit() {
    let program = code::compile(include_bytes!("../../res/hello_world.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.run();
    let program = code::compile(include_bytes!("../../res/hello_world.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.run(); 

    let k = 4;
    let vmcircuit = VMCircuit {matrix: vm.matrix};
    let prover = MockProver::run(k, &vmcircuit, vec![]).unwrap();
    prover.assert_satisfied();
    println!("vm circuit sucessfuly verified!")
}