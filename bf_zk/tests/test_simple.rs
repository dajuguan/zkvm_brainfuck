use std::{marker::PhantomData, usize};

use bf_vm::{code, interpreter::Interpreter};
use bf_zk::main_config::VMCircuit;
use halo2_proofs::{dev::MockProver, halo2curves::bn256::Fr};

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
    println!(
        "{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
        vm.matrix.program.len(),
        vm.matrix.input_matrix.len(),
        vm.matrix.instruction_matrix.len(),
        vm.matrix.memory_matrix.len(),
        vm.matrix.processor_matrix.len(),
        vm.matrix.output_matrix.len(),
        vm.matrix.input_matrix.len()
    );
}

fn setup_circuit() -> VMCircuit<Fr, 8> {
    let program = code::compile(include_bytes!("../../res/hello_world.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.run();
    let program = code::compile(include_bytes!("../../res/hello_world.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.run();
    
    VMCircuit::<Fr, 8> {
        matrix: vm.matrix,
        _marker: PhantomData,
    }
}

#[test]
fn test_vmcircuit() {
    let k = 9;
    let mut vmcircuit = setup_circuit();
    let output_val = vmcircuit.matrix.output_matrix.iter().map(|v| v.value).collect::<Vec<Fr>>();
    // println!("output_val: {:?}", output_val);
    let prover = MockProver::run(k, &vmcircuit, vec![output_val.clone()]).unwrap();
    prover.assert_satisfied();

    vmcircuit.matrix.output_matrix.reverse();
    let prover = MockProver::run(k, &vmcircuit, vec![output_val]).unwrap();
    assert!(prover.verify().is_err());
    println!("vm circuit sucessfuly verified!")
}

#[cfg(feature = "dev-graph")]
#[test]
fn vmcircuit_graph() {
    let circuit = setup_circuit();
    use plotters::prelude::*;
    let root = BitMapBackend::new("./simple.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root
        .titled("Simple Circuit without chip", ("sans-serif", 60))
        .unwrap();

    halo2_proofs::dev::CircuitLayout::default()
        // You can optionally render only a section of the circuit.
        // .view_width(0..2)
        // .view_height(0..16)
        // You can hide labels, which can be useful with smaller areas.
        .show_labels(true)
        // Render the circuit onto your area!
        // The first argument is the size parameter for the circuit.
        .render(9, &circuit, &root)
        .unwrap();
}
