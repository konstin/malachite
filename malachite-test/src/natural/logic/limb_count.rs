use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::natural::naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_limb_count);
    register_bench!(registry, Large, benchmark_natural_limb_count);
}

fn demo_natural_limb_count(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("limb_count({}) = {}", n, n.limb_count());
    }
}

fn benchmark_natural_limb_count(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.limb_count()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.limb_count())))],
    );
}
