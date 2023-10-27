# zkvm_brainfuck
A BrainFuck zkVM implementation using Halo2.
Zkvm_brainfuck is a zero-knowledge BrainFuck VM, designed as a custom, STARK-inspired version of PSE's Halo2. It's only for learning purposes only, out of a desire to gain a deeper, detailed understanding of Halo2 and VM arithmetization.

For what's about Brainfuck language and it's constraints, please read [the BrainSTARK introduction](https://neptune.cash/learn/brainfuck-tutorial/) and [Arithmetization of Brainfuck VM](https://aszepieniec.github.io/stark-brainfuck/arithmetization) first.

## Difference with BrainSTARK 

### Primitives
1. In BrainSTARK, the author uses Permuation Running Product to do permutation check, which requires some challeges values chosen by the Fiat-Shamir public oracle, thus might incur potential security risk. Here we use Halo2's in-house `lookup_any` API to do the Permuation check.

2. In BrainSTARK, the author uses Running Evaluation to verify that one table contains rows that are an (order-preserving) sublist of another table. Here, by combining `lookup` and a order-perservating gate, we can constrait the same thing on output table and input table.

### Protocol
Because we can directly do the above two constraints in halo2, there is no need to introduce a Instrcution table as a bridge fullfill the Permuation Running Product and Running Evaluation.

For clarity, the two gates involved with memory inverse `inv` required by Processor table in [Arithmetization of Brainfuck VM](https://aszepieniec.github.io/stark-brainfuck/arithmetization) is replace by `IsZeroChip`. 

A range table is used to constrain the memory value and cycle offset, here the chosen range is `[0,255]`.

## VM

Credits to cryptape to [VM implementation](https://github.com/cryptape/ckb-bf-zkvm). We made some minor changes to the `Matrix` structure in `bf_vm/src/matrix.rs` for easier lookup integration.

### test 2 simple program in `bf_vm/tests`
```
cd bf_vm && cargo test
```

## ZK_VM
In `bf_zk/tests` dir, we provide two basic test cases, of which one is without inputs while the other is with inputs.
```
cd bf_zk
cargo test test_vmcircuit -- --show-output
```