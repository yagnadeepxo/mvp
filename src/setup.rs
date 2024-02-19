use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::default_types::FrElement as FE;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::unsigned_integer::element::{UnsignedInteger};

use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let random_u64 = rng.gen::<u64>();
    let rand = UnsignedInteger::from_u64(random_u64);
    let tau = FE::new(rand);
    let n = u32::pow(2, 5);
    
    // Initialize vectors with capacity
    let mut ptau = Vec::with_capacity((2 * n) as usize);
    let mut Gsrs = Vec::with_capacity((2 * n) as usize);

    // Get the generator once
    let g1 = BLS12381Curve::generator();
    
    // Initialize the first element of ptau and Gsrs
    ptau.push(FE::one());
    Gsrs.push(g1.clone()); // Assuming clone is cheap; if expensive, adjust accordingly

    // Start with tau and G as the first elements
    let mut current_tau_power = tau.clone();
    let mut current_g = g1.operate_with_self(tau.representative());

    for _ in 1..2 * n {
        // For ptau, push the current power of tau
        ptau.push(current_tau_power.clone());

        // For Gsrs, use the previously calculated point and multiply by tau again
        Gsrs.push(current_g.clone());

        // Calculate the next power of tau and the next group element
        current_tau_power = &current_tau_power * &tau; // Adjust based on actual API for multiplication
        current_g = current_g.operate_with_self(tau.representative()); // Re-multiplication by tau
    }
    
    println!("{}",Gsrs[1].to_affine().x().representative());
    println!("{}",Gsrs[1].to_affine().y().representative());
    println!("ok");

    let x = FE::from_hex_unchecked("0x123");
    let y = FE::from_hex_unchecked("0x345");

    
    

}
