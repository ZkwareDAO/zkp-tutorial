// For randomness (during paramgen and proof generation)
use rand::{thread_rng};

// Bring in some tools for using pairing-friendly curves
use pairing::{
    Engine
};

use ff::{
    Field, PrimeField
};
// We're going to use the BLS12-381 pairing-friendly elliptic curve.
use bls12_381::{Bls12, Scalar as Fr};

// We're going to use the Groth16 proving system.
use bellman::groth16::{
    batch, create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
    Proof,
};

// We'll use these interfaces to construct our circuit.
use bellman::{
    Circuit,
    ConstraintSystem,
    SynthesisError
};

mod multiply;
use multiply::MultiplyDemo;

fn main() {
    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let mut rng = &mut thread_rng();
    
    println!("Creating parameters...");
    
    // Create parameters for our circuit
    let params = {
        let c = MultiplyDemo {
            a: None,
            // make option values as None for these variables, for paramgen
            // don't want to bake these nums into parameters
            b: None,
            c: None
        };

        generate_random_parameters::<Bls12, _, _>(c, &mut rng).unwrap()
    };
    
    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");
    
    let public_input = Fr::from_str_vartime("21");
    
    // Create an instance of circuit
    let c = MultiplyDemo {
        a: Fr::from_str_vartime("7"),
        // when creating instance here, pass in Some of actual variables you're using
        b: Fr::from_str_vartime("3"),
        c: public_input
    };
    
    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();
    
    println!("{}", verify_proof(
        &pvk,
        &proof,
        &[public_input.unwrap()]
    ).is_ok());
}
