use common::GenerationMode;
use malachite_nz::integer::Integer;
use num::BigUint;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;

type It = Iterator<Item = u32>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_u())
}

pub fn random_inputs() -> Box<It> {
    Box::new(random_x(&EXAMPLE_SEED))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(_) => random_inputs(),
    }
}

pub fn demo_integer_from_u32(gm: GenerationMode, limit: usize) {
    for u in select_inputs(gm).take(limit) {
        println!("from({}) = {}", u, Integer::from(u));
    }
}

pub fn benchmark_integer_from_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer::from(u32)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|u| Integer::from(u)),
        function_g: &(|u| BigUint::from(u)),
        function_h: &(|u| rugint::Integer::from(u)),
        x_cons: &(|&u| u),
        y_cons: &(|&u| u),
        z_cons: &(|&u| u),
        x_param: &(|&u| (32 - u.leading_zeros()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Integer::from(u32)",
        x_axis_label: "u.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
