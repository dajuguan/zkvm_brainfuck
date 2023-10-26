use crate::{
    is_zero::{IsZeroChip, IsZeroConfig},
    range_table::{RangeTable, RangeTableConfig},
    utilts::*,
};
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
pub struct ProcessorTable<const RANGE: usize> {
    pub clk: Column<Advice>,
    pub instruction_pointer: Column<Advice>,
    pub current_instruction: Column<Advice>,
    pub next_instruction: Column<Advice>,
    pub memory_pointer: Column<Advice>,
    pub memory_value: Column<Advice>,
    pub mv_iszero_config: IsZeroConfig<Fr>,
    pub s_p: Selector, //selector for processor table
    pub s_b: Selector, //selector for boundary constraints
    range_config: RangeTableConfig<RANGE>,
}

fn create_deselecor(ci: Expression<Fr>, op: u8) -> Expression<Fr> {
    let one = Expression::Constant(Fr::one());
    OPCODES.iter().fold(one, |expr, v| {
        if *v == op {
            expr
        } else {
            expr * (ci.clone() - Expression::Constant(Fr::from(*v as u64)))
        }
    })
}

impl<const RANGE: usize> ProcessorTable<RANGE> {
    pub fn configure(cs: &mut ConstraintSystem<Fr>) -> Self {
        let clk = cs.advice_column();
        let instruction_pointer = cs.advice_column();
        cs.enable_equality(instruction_pointer);
        let current_instruction = cs.advice_column();
        cs.enable_equality(current_instruction);
        let next_instruction = cs.advice_column();
        cs.enable_equality(next_instruction);
        let memory_pointer = cs.advice_column();
        cs.enable_equality(memory_pointer);
        let memory_value = cs.advice_column();
        cs.enable_equality(memory_value);
        let memory_value_inverse = cs.advice_column();
        let s_p = cs.selector();
        let s_b = cs.selector();

        let range_config = RangeTableConfig::configure(cs);

        let mv_iszero_config = IsZeroChip::configure(
            cs,
            |meta| meta.query_selector(s_p),
            |meta| meta.query_advice(memory_value, Rotation::cur()),
            memory_value_inverse,
        );

        let zero = Expression::Constant(Fr::zero());
        let one = Expression::Constant(Fr::one());
        let two = Expression::Constant(Fr::from(2));
        let range_max = Expression::Constant(Fr::from((RANGE - 1) as u64));

        cs.create_gate("boundary constraints", |meta| {
            let s_b = meta.query_selector(s_b);
            let cur_clk = meta.query_advice(clk, Rotation::cur());
            let ip = meta.query_advice(instruction_pointer, Rotation::cur());
            let mp = meta.query_advice(memory_pointer, Rotation::cur());
            let mv = meta.query_advice(memory_value, Rotation::cur());
            Constraints::with_selector(
                s_b,
                //clk_0, ip_0, mp_0, mv_0   = 0
                vec![cur_clk, ip, mp, mv],
            )
        });

        cs.create_gate("invirant constrains", |meta| {
            let sp = meta.query_selector(s_p);
            let cur_clk = meta.query_advice(clk, Rotation::cur());
            let next_clk = meta.query_advice(clk, Rotation::next());

            let mv = meta.query_advice(memory_value, Rotation::cur());
            let mv_inv = meta.query_advice(memory_value_inverse, Rotation::cur());

            Constraints::with_selector(
                sp,
                vec![
                    // clk increase one per step
                    (next_clk - cur_clk - one.clone()),
                    // mv is 0 or mv_inv is the inverse of mv
                    mv.clone() * (mv.clone() * mv_inv.clone() - one.clone()),
                    // mv_inv is 0 or mv_inv is the inverse of mv
                    mv_inv.clone() * (mv * mv_inv - one.clone()),
                ],
            )
        });

        cs.lookup("Range-Check: mv are within 0-255", |meta| {
            let mv = meta.query_advice(memory_value, Rotation::cur());
            vec![(mv, range_config.table)]
        });

        cs.create_gate("instruction constraints", |meta| {
            let ci = meta.query_advice(current_instruction, Rotation::cur());
            let deselectors = OPCODES
                .iter()
                .map(|op| create_deselecor(ci.clone(), *op))
                .collect::<Vec<_>>();
            let cur_mp = meta.query_advice(memory_pointer, Rotation::cur());
            let next_mp = meta.query_advice(memory_pointer, Rotation::next());
            let sp = meta.query_selector(s_p);
            let cur_ip = meta.query_advice(instruction_pointer, Rotation::cur());
            let cur_ni = meta.query_advice(next_instruction, Rotation::cur());
            let next_ip = meta.query_advice(instruction_pointer, Rotation::next());
            let cur_mv = meta.query_advice(memory_value, Rotation::cur());
            let next_mv = meta.query_advice(memory_value, Rotation::next());
            let mv_iszero = mv_iszero_config.expr();

            //--------------------------------Instruct pointer constraints part-----------------------------//
            // ADD:+, SUB:-, SHR:>, SHL:<, GETCHAR:",", PUTCHAR:"." share the same p1 condition:
            // ip increases by 1
            let expr1 = (deselectors[ADD].clone()
                + deselectors[SUB].clone()
                + deselectors[SHR].clone()
                + deselectors[SHL].clone()
                + deselectors[GETCHAR].clone()
                + deselectors[PUTCHAR].clone())
                * (next_ip.clone() - cur_ip.clone() - one.clone());

            // LB:[ if mv != 0 ⇒ ip increases by 2 and if mv == 0 ⇒ ip is set to ni
            let expr_lb = deselectors[LB].clone()
                * ((one.clone() - mv_iszero.clone())
                    * (next_ip.clone() - cur_ip.clone() - two.clone())
                    + mv_iszero.clone() * (next_ip.clone() - cur_ni.clone()));
            // RB:] if mv == 0 ⇒ ip increases by 2 and if mv != 0 ⇒ ip is set to ni
            let expr_rb = deselectors[RB].clone()
                * (mv_iszero.clone() * (next_ip.clone() - cur_ip.clone() - two)
                    + (one.clone() - mv_iszero.clone())
                        * (cur_mv.clone() * (next_ip.clone() - cur_ni.clone())));

            //--------------------------------Memory pointer constraints part-----------------------------//
            // ADD:+, SUB:-, LB:[, RB:], GETCHAR:, PUTCHAR share the same p2 condition:
            // memory pointer stay at the same
            let expr2 = (deselectors[ADD].clone()
                + deselectors[SUB].clone()
                + deselectors[LB].clone()
                + deselectors[RB].clone()
                + deselectors[GETCHAR].clone()
                + deselectors[PUTCHAR].clone())
                * (next_mp.clone() - cur_mp.clone());

            //SHL:< mp decreases by one
            let expr_shl =
                deselectors[SHL].clone() * (next_mp.clone() - cur_mp.clone() + one.clone());
            //SHRL:> mp increases by one
            let expr_shr =
                deselectors[SHR].clone() * (next_mp.clone() - cur_mp.clone() - one.clone());

            //--------------------------------Memory value constraints part-----------------------------//
            // LB, RB, PUTCHAR share the same p3 condition:
            // memory value stay at the same
            let expr3 =
                (deselectors[LB].clone() + deselectors[RB].clone() + deselectors[PUTCHAR].clone())
                    * (next_mv.clone() - cur_mv.clone());
            // note: we have lookup table to ensure all mvs are within [0-255],
            // therefore, value can only decreases by 255 iff cur_mv=255, next_mv=0
            // same goes for wrapping_sub
            // ADD: mv increases by 1, or decreases by 255
            let expr_add = deselectors[ADD].clone()
                * (next_mv.clone() - cur_mv.clone() - one.clone())
                * (next_mv.clone() - cur_mv.clone() + range_max.clone());
            // sub: mv decreases by 1, or increases by 255
            let expr_sub = deselectors[SUB].clone()
                * (next_mv.clone() - cur_mv.clone() + one.clone())
                * (next_mv.clone() - cur_mv.clone() - range_max.clone());
            // SHL, SHR, GETCHAR: always true (check elsewhere)
            let expr4 = (deselectors[SHL].clone()
                + deselectors[SHR].clone()
                + deselectors[GETCHAR].clone())
                * (zero);

            Constraints::with_selector(
                sp,
                vec![
                    expr1 + expr_lb + expr_rb,
                    expr2 + expr_shl + expr_shr,
                    expr3 + expr_add + expr_sub + expr4,
                ],
            )
        });

        ProcessorTable {
            clk,
            instruction_pointer,
            current_instruction,
            next_instruction,
            memory_pointer,
            memory_value,
            mv_iszero_config,
            s_p,
            s_b,
            range_config,
        }
    }
    pub fn load(&mut self, mut layouter: impl Layouter<Fr>, matrix: &Matrix) -> Result<(), Error> {
        let processor_mat = &matrix.processor_matrix;
        let iszero_chip = IsZeroChip::construct(self.mv_iszero_config.clone());

        self.range_config.load_table(&mut layouter)?;

        layouter.assign_region(
            || "processor table",
            |mut region| {
                for i in 0..processor_mat.len() {
                    if i == 0 {
                        self.s_b.enable(&mut region, i)?;
                    } else {
                        self.s_p.enable(&mut region, i - 1)?;
                    }

                    region.assign_advice(
                        || "clk cell",
                        self.clk,
                        i,
                        || Value::known(processor_mat[i].cycle),
                    )?;

                    region.assign_advice(
                        || "instruction pointer cell",
                        self.instruction_pointer,
                        i,
                        || Value::known(processor_mat[i].instruction_pointer),
                    )?;

                    region.assign_advice(
                        || "current_instruction cell",
                        self.current_instruction,
                        i,
                        || Value::known(processor_mat[i].current_instruction),
                    )?;

                    region.assign_advice(
                        || "next_instruction cell",
                        self.next_instruction,
                        i,
                        || Value::known(processor_mat[i].next_instruction),
                    )?;

                    region.assign_advice(
                        || "memory_pointer cell",
                        self.memory_pointer,
                        i,
                        || Value::known(processor_mat[i].memory_pointer),
                    )?;

                    region.assign_advice(
                        || "memory_value cell",
                        self.memory_value,
                        i,
                        || Value::known(processor_mat[i].memory_value),
                    )?;

                    iszero_chip.assign(
                        &mut region,
                        i,
                        Value::known(processor_mat[i].memory_value),
                    )?;
                }

                Ok(())
            },
        )?;

       
        Ok(())
    }
}
