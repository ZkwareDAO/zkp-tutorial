#[macro_use]
extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Bencher};

use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;

extern crate bls12_381;
use bls12_381::*;
use group::{Curve, Group};
use pairing_lib::{Engine, MillerLoopResult, MultiMillerLoop};

fn bench_pairing_full(c: &mut Criterion) {
    const SAMPLES: usize = 2000;

    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    let v: Vec<(G1Affine, G2Affine)> = (0..SAMPLES)
        .map(|_| {
            (
                G1Projective::random(&mut rng).to_affine(),
                G2Projective::random(&mut rng).to_affine(),
            )
        })
        .collect();

    let mut count = 0;
    c.bench_function("bls12-381 pairing", |b| {
        b.iter(|| {
            for _ in 0..SAMPLES {
                let tmp = pairing(&v[count].0, &v[count].1);
                count = (count + 1) % SAMPLES;
            }
        });
    });
}

criterion_group!(benches, bench_pairing_full);
criterion_main!(benches);