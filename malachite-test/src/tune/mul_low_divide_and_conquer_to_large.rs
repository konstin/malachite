use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_bench::tune::{compare_two, ComparisonResult};
use malachite_nz::natural::arithmetic::mul::mul_low::{
    _limbs_mul_low_same_length_divide_and_conquer,
    _limbs_mul_low_same_length_divide_and_conquer_scratch_len, _limbs_mul_low_same_length_large,
};
use malachite_nz::platform::Limb;

use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::triples_of_unsigned_vec_var_52;

pub(crate) fn tune() -> Vec<String> {
    let result = compare_two(
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())];
            _limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch);
        }),
        &mut (|(mut out, xs, ys): (Vec<Limb>, Vec<Limb>, Vec<Limb>)| {
            let mut scratch =
                vec![0; _limbs_mul_low_same_length_divide_and_conquer_scratch_len(xs.len())];
            _limbs_mul_low_same_length_large(&mut out, &xs, &ys, &mut scratch);
        }),
        triples_of_unsigned_vec_var_52(GenerationMode::Random(u32::power_of_two(15))),
        10000,
        &(|&(_, ref xs, _)| xs.len()),
    );
    let mut lines = Vec::new();
    match result {
        ComparisonResult::SecondBetterAbove(threshold) => {
            lines.push(format!(
                "pub const MULLO_MUL_N_THRESHOLD: usize = {};",
                threshold
            ));
        }
        ComparisonResult::NeitherBetter => {
            lines.push("pub const MULLO_MUL_N_THRESHOLD: usize = 100000;".to_string());
        }
        _ => {
            panic!(
                "Unexpected mul low divide-and-conquer to large tuning result: {:?}",
                result
            );
        }
    }
    lines
}
