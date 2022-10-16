// use signature::{
//     digest::{core_api::BlockSizeUser, Digest, FixedOutput, FixedOutputReset},
//     hazmat::PrehashSigner,
//     rand_core::{CryptoRng, RngCore},
//     DigestSigner, RandomizedDigestSigner, RandomizedSigner, Signer,
// };

use k256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use rand_core::OsRng; 
mod ecdsa_secp256k1;
use ecdsa_secp256k1::*;

fn main() {
    let signing_key = SigningKey::random(&mut OsRng); 
    let sk=signing_key.to_bytes();
    let mut msg =  String::from("Hello,world");

    println!("\nSigning key: {:x?}",hex::encode(sk));
    let sig = ecdsa_sign(&msg, &signing_key);
    println!("\nSignature : {:x?}",hex::encode(sig)); 
    let verify_key = VerifyingKey::from(&signing_key);
    let rtn = ecdsa_verify(&msg, &verify_key, &sig);
    if rtn==true { println!("\nMessage '{0}' signature correct", msg); }
    else { println!("\nMessage '{0}' signature incorrect", msg);}
}
