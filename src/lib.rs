use num_bigint::{BigUint, RandBigInt};
use rand::Rng;

// this is going the the exponentiation operation alpha**x etc
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    n.modpow(exponent, modulus)
}

// output = s = k - c * x mod q
pub fn solve(k: &BigUint, challenge: &BigUint, secret: &BigUint, modulus: &BigUint) -> BigUint {
    if *k >= challenge * secret {
        return (k - challenge * secret).modpow(&BigUint::new(vec![1u32]), modulus);
    } else {
        return modulus - (challenge * secret - k).modpow(&BigUint::from(1u32), modulus);
    }
}

// check R1 = (alpha ** s * y1 ** c ) mod p
// check R2 = (beta ** s * y2 ** c) mod p
pub fn verify(
    r1: &BigUint,
    r2: &BigUint,
    y1: &BigUint,
    y2: &BigUint,
    alpha: &BigUint,
    beta: &BigUint,
    s: &BigUint,
    c: &BigUint,
    modulus: &BigUint,
) -> bool {
    let condition1 = *r1
        == (alpha.modpow(s, modulus) * y1.modpow(c, modulus)).modpow(&BigUint::from(1u32), modulus);

    let condition2 = *r2
        == (beta.modpow(s, modulus) * y2.modpow(c, modulus)).modpow(&BigUint::from(1u32), modulus);

    // bcoz of the & operation both need to be true
    condition1 && condition2
}

pub fn generate_random_below(bound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    rng.gen_biguint_below(bound)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toy_example() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let secret = BigUint::from(6u32);
        let k = BigUint::from(7u32);
        let c = BigUint::from(4u32);

        let y1 = super::exponentiate(&alpha, &secret, &p);
        assert!(y1 == BigUint::from(2u32));

        let y2 = super::exponentiate(&beta, &secret, &p);
        assert!(y2 == BigUint::from(3u32));

        let r1 = super::exponentiate(&alpha, &k, &p);
        assert!(r1 == BigUint::from(8u32));

        let r2 = super::exponentiate(&beta, &k, &p);
        assert!(r2 == BigUint::from(4u32));

        let s = super::solve(&k, &c, &secret, &q);
        assert_eq!(s, BigUint::from(5u32));

        let result = super::verify(&r1, &r2, &y1, &y2, &alpha, &beta, &s, &c, &p);
        assert!(result == true);

        // trying a fake value
        let secret_fake = BigUint::from(7u32);
        let s_fake = super::solve(&k, &c, &secret_fake, &q);

        let result = super::verify(&r1, &r2, &y1, &y2, &alpha, &beta, &s_fake, &c, &p);
        assert!(result == false);
    }

    #[test]
    fn test_toy_example_with_random() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let secret = super::generate_random_below(&q);
        let k = super::generate_random_below(&q);
        let c = super::generate_random_below(&q);

        let y1 = super::exponentiate(&alpha, &secret, &p);

        let y2 = super::exponentiate(&beta, &secret, &p);

        let r1 = super::exponentiate(&alpha, &k, &p);

        let r2 = super::exponentiate(&beta, &k, &p);

        let s = super::solve(&k, &c, &secret, &q);

        let result = super::verify(&r1, &r2, &y1, &y2, &alpha, &beta, &s, &c, &p);
        assert!(result == true);
    }
}
