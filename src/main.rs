use ark_ec::bls12;
use ark_ff::MontFp;
use ark_std::ops::Mul;
use  ark_bls12_cheon::{
     G1Projective as G1, G2Projective as G2, Fr as Fr, Fq, Fq2, 
     Parameters
};

pub type G2Affine = bls12::G2Affine<crate::Parameters>;
pub type G1Affine = bls12::G1Affine<crate::Parameters>;

fn main() {

    // Put your solution for tau in the line below
    // E.g., let tau  = Fr::from(1 as i128);
    let tau  = Fr::from(1 as i128);

    // p is the generator for G1
    let p_x: Fq = MontFp!("4366845981406663127346140105392043296067620632748305894915559567990751463871846461571751242076416842353760718219463");
    let p_y: Fq = MontFp!("2322936086289479068066490612801140015408721761579169972917817367391045591350600034547617432914931369293155123935728");
    let p = G1::from(G1Affine::new_unchecked(p_x, p_y));

    // tau_p is tau * p
    let tau_p_x: Fq = MontFp!("4098523714512767373909357151251054769972251070043521443432671846694698445310918888674563384159054413429407635337545");
    let tau_p_y: Fq = MontFp!("5104199250567875649831070757916425527562742910121942289068977475282886267374972984853923971702383949149250750103817");
    let tau_p = G1::from(G1Affine::new_unchecked(tau_p_x, tau_p_y));

    // _tau_d1_p is (tau ^ d1) * p
    let tau_d1_p_x: Fq = MontFp!("5666793736468521094298738103921693621508309431638529348089160865885867964805742107934338916926827890667749984768215");
    let tau_d1_p_y: Fq = MontFp!("3326251098281288448602352180414320622119338868949804322483594574847275370379159993188118130786632123868776051955196");
    let _tau_d1_p = G1::from(G1Affine::new_unchecked(tau_d1_p_x, tau_d1_p_y));

    // _q is the generator for G2
    let q_c0_x: Fq = MontFp!("2847190178490156899798643792842723617787968359868175140038826869144776012793105029391523604954249120667126821536281");
    let q_c1_x: Fq = MontFp!("1513797577242500304678874752065526230408447782356629533374984043360635354098197307045487457331199798062484459984831");
    let q_x: Fq2 = Fq2::new(q_c0_x, q_c1_x);
    let q_c0_y: Fq = MontFp!("2398127858646538650279262747029238501121661957103909673770298065006753715123740323569605568913154172079135187452386");
    let q_c1_y: Fq = MontFp!("5444946257649901533268220138726124417824817651440748374257708320447300055543369665159277001725118567443194417165086");
    let q_y: Fq2 = Fq2::new(q_c0_y, q_c1_y);
    let _q = G2::from(G2Affine::new_unchecked(q_x, q_y));

    // _tau_d2_q is (tau ^ d2) * _q
    let tau_d2_q_c0_x: Fq = MontFp!("3536383419772898871062064633012296862124372086039789534814905834326827967479599778599887194095392165550880125330266");
    let tau_d2_q_c1_x: Fq = MontFp!("2315075704417849395497347082310199859284883937672695000597201154920791698799875018503579607990866594389648640170976");
    let tau_d2_q_x: Fq2 = Fq2::new(tau_d2_q_c0_x, tau_d2_q_c1_x);
    let tau_d2_q_c0_y: Fq = MontFp!("58522461936731088989461032245338237080030815519467180488197672431529427745827070450637290003234818635632376291077");
    let tau_d2_q_c1_y: Fq = MontFp!("200313434320582884299950030908390796161004965251373896142196467499133624968316891420231529223691679020778320981956");
    let tau_d2_q_y: Fq2 = Fq2::new(tau_d2_q_c0_y, tau_d2_q_c1_y);
    let _tau_d2_q = G2::from(G2Affine::new_unchecked(tau_d2_q_x, tau_d2_q_y));

    println!("Have you correctly solved the puzzle? {}", if tau_p == p.mul(Fr::from(tau)) { "Yes!" } else { "No." });
}
