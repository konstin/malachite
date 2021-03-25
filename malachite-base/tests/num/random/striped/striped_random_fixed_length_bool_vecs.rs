use itertools::Itertools;
use malachite_base::num::random::striped::striped_random_fixed_length_bool_vecs;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;
use num::random::striped::get_striped_bool_vec::bool_slice_to_string;

fn striped_random_fixed_length_bool_vecs_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    len: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let xs = striped_random_fixed_length_bool_vecs(
        EXAMPLE_SEED,
        mean_stripe_numerator,
        mean_stripe_denominator,
        len,
    );
    let values = xs
        .clone()
        .take(20)
        .map(|bs| bool_slice_to_string(&bs))
        .collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone())
        .into_iter()
        .map(|(bs, freq)| (bool_slice_to_string(&bs), freq))
        .collect_vec();
    let (median_lo, median_hi) = median(xs.take(1000000));
    let median_lo = bool_slice_to_string(&median_lo);
    let median_hi = median_hi.map(|bs| bool_slice_to_string(&bs));
    assert_eq!(
        (
            values.iter().map(String::as_str).collect_vec().as_slice(),
            common_values
                .iter()
                .map(|(s, f)| (s.as_str(), *f))
                .collect_vec()
                .as_slice(),
            (median_lo.as_str(), median_hi.as_deref())
        ),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_striped_random_fixed_length_bool_vecs() {
    striped_random_fixed_length_bool_vecs_helper(10, 1, 0, &[""; 20], &[("", 1000000)], ("", None));
    striped_random_fixed_length_bool_vecs_helper(
        10,
        1,
        1,
        &[
            "0", "0", "0", "0", "0", "1", "0", "1", "0", "1", "1", "0", "0", "0", "1", "0", "1",
            "0", "0", "1",
        ],
        &[("1", 500079), ("0", 499921)],
        ("1", None),
    );
    striped_random_fixed_length_bool_vecs_helper(
        10,
        1,
        2,
        &[
            "00", "00", "00", "00", "00", "11", "00", "11", "00", "11", "11", "00", "00", "00",
            "11", "00", "11", "00", "01", "11",
        ],
        &[("11", 449989), ("00", 449537), ("01", 50384), ("10", 50090)],
        ("10", None),
    );
    striped_random_fixed_length_bool_vecs_helper(
        10,
        1,
        5,
        &[
            "00000", "00000", "00000", "00000", "00011", "11000", "00000", "11111", "01111",
            "11111", "10000", "00011", "00000", "00000", "11000", "00000", "11111", "00000",
            "00000", "11111",
        ],
        &[
            ("11111", 328176),
            ("00000", 327532),
            ("00001", 36685),
            ("10000", 36616),
            ("00111", 36602),
            ("01111", 36495),
            ("11110", 36487),
            ("11000", 36446),
            ("00011", 36354),
            ("11100", 36250),
        ],
        ("10000", None),
    );
}

#[test]
#[should_panic]
fn striped_random_fixed_length_bool_vecs_fail_1() {
    striped_random_fixed_length_bool_vecs(EXAMPLE_SEED, 1, 0, 5);
}

#[test]
#[should_panic]
fn striped_random_fixed_length_bool_vecs_fail_2() {
    striped_random_fixed_length_bool_vecs(EXAMPLE_SEED, 2, 3, 5);
}
