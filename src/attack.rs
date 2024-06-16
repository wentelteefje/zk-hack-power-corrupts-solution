use ark_bls12_cheon::{Bls12Cheon, Fq12, Fr, G1Projective as G1, G2Projective as G2};

use crate::utils::{bigInt_to_u128, pow_sp, pow_sp2};
use ark_ec::pairing::Pairing;
use ark_ff::{Field, PrimeField};

const d1: u64 = 11726539;
const d2: u64 = 690320833;
const d: u64 = d1 + d2;
const q: u128 = 1114157594638178892192613;
const q1_d: u64 = ((q - 1) / d as u128 ) as u64;

pub fn attack(P: G1, tau_P: G1, tau_d1_P: G1, Q: G2, tau_d2_Q: G2) -> i128 {
    // Using pairing to obtain g := e(P,Q), g^tau := e(tau P,Q), g^tau^d = e(tau^d P, Q)
    let g: Fq12 = Bls12Cheon::pairing(P, Q).0;
    let g_tau: Fq12 = Bls12Cheon::pairing(tau_P, Q).0;
    let g_tau_d: Fq12 = Bls12Cheon::pairing(tau_d1_P, tau_d2_Q).0;

    let Ap1: u64 = 1089478584172544;
    let two_pow_d = pow_sp(Fr::from(2u64), d.into(), 31);
    let giant = g_tau_d.pow(pow_sp(two_pow_d.inverse().unwrap(), Ap1.into(), 51).into_bigint());
    // Solve DLP in H_1, i.e. find k0
    let k0 = match baby_step_giant_step(g, two_pow_d, q1_d, giant, 1 << 20) {
        Some(result) => Ap1 + result,
        None => {
            eprintln!("Error: BSGS failed to find k0.");
            std::process::exit(1);
        }
    };
    println!("Found k0 = {}", k0);

    let two_pow_q1_d = pow_sp(Fr::from(2u64), q1_d.into(), 51);
    let giant = g_tau.pow(pow_sp(Fr::from(2u64).inverse().unwrap(), k0.into(), 51).into_bigint());
    // Solve DLP in H_2, i.e. find k1
    let k1 = match baby_step_giant_step(g, two_pow_q1_d, d, giant, 1 << 16) {
        Some(k1) => k1,
        None => {
            eprintln!("Error: BSGS failed to find k1.");
            std::process::exit(1);
        }
    };
    println!("Found k1 = {}", k1);

    // Calculate τ = 2^(k0 + k1 (q-1)/d)
    let exp = (k0 as u128) + (k1 as u128) * (q1_d as u128);
    let tau = pow_sp(Fr::from(2u64), exp, 80);
    println!("Found τ = {}", tau);

    return bigInt_to_u128(tau.into_bigint()) as i128;
}

fn baby_step_giant_step<S: Field>(a: S, b: Fr, group_ord: u64, mut giant: S, split: u64) -> Option<u64> {
    let baby_steps = pow_sp2(a, b, split);
    let giant_step = pow_sp(b.inverse().unwrap(), split.into(), 64).into_bigint();
    let n = group_ord / split;

    for i in 0..n {
        if let Some(j) = baby_steps.get(&giant) {
            // return Some(i * split + (*j));
            return Some(i * split + (*j));
        }
        giant = giant.pow(giant_step);
    }
    None
}