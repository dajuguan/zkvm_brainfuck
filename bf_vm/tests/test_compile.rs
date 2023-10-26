use bf_vm::code;
use halo2_proofs::halo2curves::bn256::Fr;

#[test]
fn test_compile_neptune() {
    let output = code::compile("++>,<[>+.<-]".as_bytes().to_vec());
    let expect: Vec<Fr> = vec![
        '+' as u64, '+' as u64, '>' as u64, ',' as u64, '<' as u64, '[' as u64, 14, '>' as u64,
        '+' as u64, '.' as u64, '<' as u64, '-' as u64, ']' as u64, 7,
    ]
    .iter()
    .map(|&x| Fr::from(x))
    .collect();
    assert_eq!(output, expect);
}
