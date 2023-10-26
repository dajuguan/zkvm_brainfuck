use bf_vm::code;
use bf_vm::interpreter::Interpreter;
use halo2_proofs::halo2curves::bn256::Fr;
use halo2curves::ff::PrimeField;

trait FieldExt {
    fn get_lower_128(&self) -> u128;
}

impl FieldExt for Fr {
    fn get_lower_128(&self) -> u128 {
        let to_limbs = |e: &Fr| {
            let repr = e.to_repr();
            let repr = repr.as_ref();
            let tmp0 = u64::from_le_bytes(repr[0..8].try_into().unwrap());
            let tmp1 = u64::from_le_bytes(repr[8..16].try_into().unwrap());
            let tmp2 = u64::from_le_bytes(repr[16..24].try_into().unwrap());
            let tmp3 = u64::from_le_bytes(repr[24..32].try_into().unwrap());
            [tmp0, tmp1, tmp2, tmp3]
        };

        let e = to_limbs(self);
        u128::from(e[0]) | (u128::from(e[1]) << 64)
    }
}

#[test]
fn test_run_hello_world() {
    let program = code::compile(include_bytes!("../../res/hello_world.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.run();
    assert_eq!(
        vm.matrix
            .output_matrix
            .iter()
            .map(|v| v.value)
            .collect::<Vec<Fr>>(),
        code::easygen("Hello World!\n")
    );
}

#[test]
fn test_run_neptune() {
    let program = code::compile(include_bytes!("../../res/neptune_tutorial.bf").to_vec());
    let mut vm = Interpreter::new();
    vm.set_code(program);
    vm.set_input(code::easygen("a"));
    vm.run();
    assert_eq!(vm.matrix.processor_matrix.len(), 19);
    assert_eq!(vm.matrix.memory_matrix.len(), 19);
    assert_eq!(vm.matrix.instruction_matrix.len(), 33);
    assert_eq!(vm.matrix.input_matrix, code::easygen("a"));
    assert_eq!(
        vm.matrix
            .output_matrix
            .iter()
            .map(|v| v.value)
            .collect::<Vec<Fr>>(),
        code::easygen("bc")
    );

    let mut program = vec![];
    for p in vm.matrix.program {
        program.push(vec![
            p.instruction_pointer.get_lower_128() as u8,
            p.current_instruction.get_lower_128() as u8,
            p.next_instruction.get_lower_128() as u8,
        ]);
    }

    let expect = vec![
        [0, 43, 43],
        [1, 43, 62],
        [2, 62, 44],
        [3, 44, 60],
        [4, 60, 91],
        [5, 91, 14],
        [14, 0, 0],
        [7, 62, 43],
        [8, 43, 46],
        [9, 46, 60],
        [10, 60, 45],
        [11, 45, 93],
        [12, 93, 7],
        [7, 62, 43],
    ];
    assert_eq!(program, expect);
}
