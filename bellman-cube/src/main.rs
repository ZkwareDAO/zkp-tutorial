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

mod cube;
use cube::*;

fn main() {
    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let mut rng = &mut thread_rng();
    
    println!("Creating parameters...");
    
    // Create parameters for our circuit
    let params = {
        let c = CubeDemo {
            x: None
        };

        generate_random_parameters::<Bls12, _, _>(c, &mut rng).unwrap()
    };
    
    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");
        
    // Create an instance of circuit
    let c = CubeDemo {
        x: Fr::from_str_vartime("3")
    };
    
    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();
    
    println!("{}", verify_proof(
        &pvk,
        &proof,
        &[Fr::from(35)]
    ).is_ok());
}
