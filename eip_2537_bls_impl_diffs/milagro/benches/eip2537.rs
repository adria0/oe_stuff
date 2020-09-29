extern crate amcl;
extern crate rustc_hex;

#[macro_use]
extern crate criterion;

use rustc_hex::{ToHex,FromHex};

fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("test_base_bls381_g1_add", |b| b.iter(|| amcl::test_utils::eip2537::test_base_bls381_g1_add()));
    c.bench_function("test_base_bls381_g1_mul", |b| b.iter(|| amcl::test_utils::eip2537::test_base_bls381_g1_mul()));
    c.bench_function("test_base_bls381_g2_add", |b| b.iter(|| amcl::test_utils::eip2537::test_base_bls381_g2_add()));
    c.bench_function("test_base_bls381_g2_mul", |b| b.iter(|| amcl::test_utils::eip2537::test_base_bls381_g2_mul()));
    c.bench_function("test_base_bls381_pairing_2", |b| b.iter(|| amcl::test_utils::eip2537::test_base_bls381_pairing_2()));
    c.bench_function("test_base_bls381_pairing_4", |b| b.iter(|| amcl::test_utils::eip2537::test_base_bls381_pairing_4()));
    c.bench_function("test_base_bls381_pairing_8", |b| b.iter(|| amcl::test_utils::eip2537::test_base_bls381_pairing_8()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


