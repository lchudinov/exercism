use num_bigint::BigUint;

pub fn private_key(p: u64) -> u64 {
    3
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow_mod(a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow_mod(a, p)
}

pub trait PowerMod {
    fn pow_mod(&self, p: Self, m: Self) -> Self;
}

impl PowerMod for u64 {
    fn pow_mod(&self, p: u64, m: u64) -> u64 {
        let base = BigUint::from(*self);
        let result = base.modpow(&BigUint::from(p), &BigUint::from(m));
        result.iter_u64_digits().next().unwrap()
    }
}