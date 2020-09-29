extern crate pairedtest;
use pairedtest::tests::*;

#[macro_use]
extern crate criterion; 

fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("test_base_bls381_g1_add", |b| b.iter(|| test_base_bls381_g1_add()));
    c.bench_function("test_base_bls381_g1_mul", |b| b.iter(|| test_base_bls381_g1_mul()));
    c.bench_function("test_base_bls381_g2_add", |b| b.iter(|| test_base_bls381_g2_add()));
    c.bench_function("test_base_bls381_g2_mul", |b| b.iter(|| test_base_bls381_g2_mul()));
    c.bench_function("test_base_bls381_pairing_2", |b| b.iter(|| test_base_bls381_pairing_2()));
    c.bench_function("test_base_bls381_pairing_4", |b| b.iter(|| test_base_bls381_pairing_4()));
    c.bench_function("test_base_bls381_pairing_8", |b| b.iter(|| test_base_bls381_pairing_8()));
    c.bench_function("test_base_bls381_fp_to_g1", |b| b.iter(|| test_base_bls381_fp_to_g1()));
    c.bench_function("test_base_bls381_fp2_to_g2", |b| b.iter(|| test_base_bls381_fp2_to_g2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);