use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bls12_381::{Scalar};

fn bls12_381_scalar_inv() {
    // Compute -(q^{-1} mod 2^64) mod 2^64 by exponentiating
    // by totient(2**64) - 1
    let one = Scalar::one();
    let seven = one+one+one+one+one+one+one;
    let mut k = one;
    for _ in 0..1000 {
        k = k*seven;
        let _ = k.invert();
    }
}

fn ffbench(c: &mut Criterion) {
    c.bench_function("ff", |b| {
        b.iter(||{ 
            bls12_381_scalar_inv();
        });
    });
}

criterion_group!(benches, ffbench);
criterion_main!(benches);
