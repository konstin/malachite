use malachite_base::crement::Crementable;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::natural::positive_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_decrement);
    register_bench!(registry, Large, benchmark_natural_decrement);
}

fn demo_natural_decrement(gm: GenerationMode, limit: usize) {
    for mut n in positive_naturals(gm).take(limit) {
        let n_old = n.clone();
        n.decrement();
        println!("n := {:?}; n.decrement(); n = {:?}", n_old, n);
    }
}

fn benchmark_natural_decrement(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.decrement()",
        BenchmarkType::Single,
        positive_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.decrement()))],
    );
}
