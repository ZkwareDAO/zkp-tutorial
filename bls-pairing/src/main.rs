use bls12_381::*;
use group::Curve;

fn main() {
    let g = G1Affine::generator();
    let h = G2Affine::generator();
    let mut s = Scalar::one();
    s = s.pow(&[1u64, 2u64, 3u64, 4u64]);
    let sg = g*s;
    let sga = sg.to_affine();
    let sh = h*s;
    let sha = sh.to_affine();
    // println!("{}", pairing(&g, &h));
    println!("{}", pairing(&g, &sha) == pairing(&sga, &h));
}
