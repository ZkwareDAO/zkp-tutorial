use k256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use rand_core::OsRng; 

pub fn ecdsa_sign(msg: &String, signing_key: &SigningKey) -> Signature {
    let m = msg.as_bytes();
    let signature: Signature = signing_key.sign(m);
    signature   
}

pub fn ecdsa_verify(msg: &String, verify_key: &VerifyingKey, signature: &Signature) -> bool {
    let m = msg.as_bytes();
    let rtn=verify_key.verify(m, signature).is_ok();
    rtn
}