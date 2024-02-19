
use lambdaworks_math::field::traits::IsPrimeField;
use lambdaworks_math::polynomial::Polynomial;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::compression::BLS12381FieldElement as FE;
use lambdaworks_math::unsigned_integer::element::UnsignedInteger;

use lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field as E;

fn main() {
    let a = UnsignedInteger::from(1u64);
    let b = UnsignedInteger::from(2u64);
    let c = UnsignedInteger::from(3u64);
    let evaluations = [FE::new(a), FE::new(b), FE::new(c)];

}
