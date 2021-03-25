use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_pair_gen, unsigned_pair_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_hamming_distance_unsigned);
    register_signed_demos!(runner, demo_checked_hamming_distance_signed);
    register_unsigned_benches!(runner, benchmark_hamming_distance_unsigned);
    register_signed_benches!(runner, benchmark_checked_hamming_distance_signed);
}

fn demo_hamming_distance_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, y) in unsigned_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("{}.hamming_distance({}) = {}", x, y, x.hamming_distance(y));
    }
}

fn demo_checked_hamming_distance_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, y) in signed_pair_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "({}).checked_hamming_distance({}) = {:?}",
            x,
            y,
            x.checked_hamming_distance(y)
        );
    }
}

fn benchmark_hamming_distance_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.hamming_distance({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.hamming_distance(y)))],
    );
}

fn benchmark_checked_hamming_distance_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_hamming_distance({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(x.checked_hamming_distance(y))
        })],
    );
}
