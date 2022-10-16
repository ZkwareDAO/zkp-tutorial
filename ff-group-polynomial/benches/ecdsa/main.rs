use criterion::{black_box, criterion_group, criterion_main, Criterion};

use k256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use rand_core::OsRng; 

fn ecdsa_sign_verify() {
    let msg = String::from("Hello,world");
    let m = msg.as_bytes();
    let signing_key = SigningKey::random(&mut OsRng); 
    let verify_key = VerifyingKey::from(&signing_key);
    for _ in 0..1000 {
        let signature: Signature = signing_key.sign(m);
        let rtn=verify_key.verify(m, &signature).is_ok();
    }
}

fn ecdsa_bench(c: &mut Criterion) {
    c.bench_function("ecdsa", |b| {
        b.iter(||{ 
            ecdsa_sign_verify();
        });
    });
}

criterion_group!(benches, ecdsa_bench);
criterion_main!(benches);
