pub mod psponge;
pub mod poseidon_params;

// #[cfg(feature="bls12_381")]
// pub mod poseidon_zk_param_bls12_381;
// #[cfg(feature="bls12_381")]
// pub use poseidon_zk_param_bls12_381::*;
#[cfg(feature="bls12_381")]
use ark_bls12_381::Bls12_381;
#[cfg(feature="bls12_381")]
pub type CurveTypeG = Bls12_381;
#[cfg(feature="bls12_381")]
pub use ark_bls12_381::*;

// #[cfg(feature="bls12_377")]
// pub mod poseidon_zk_param_bls12_377;
// #[cfg(feature="bls12_377")]
// pub use poseidon_zk_param_bls12_377::*;
#[cfg(feature="bls12_377")]
use ark_bls12_377::Bls12_377;
#[cfg(feature="bls12_377")]
pub type CurveTypeG = Bls12_377;
#[cfg(feature="bls12_377")]
pub use ark_bls12_377::*;

// #[cfg(feature="bn254c")]
// pub mod poseidon_zk_param_bn254;
// #[cfg(feature="bn254c")]
// pub use poseidon_zk_param_bn254::*;
#[cfg(feature="bn254c")]
use ark_bn254::Bn254;
#[cfg(feature="bn254c")]
pub type CurveTypeG= Bn254;
#[cfg(feature="bn254c")]
pub use ark_bn254::*;

#[allow(unused)]
pub static SIZEOFOUTPUT: usize = 2;
pub  const  SIZEOFINPUT: usize = 64;


use ark_sponge::poseidon::PoseidonSponge;
pub use psponge::*;
pub use poseidon_params::*;

use rand_chacha::ChaCha20Rng;
use ark_std::rand::SeedableRng;

//pub mod poseidon_params;
//pub use poseidon_params::*;
//pub mod zk_params_bls12_377;
//pub use zk_params_bls12_377::ZK_PARAM_BYTES;

//pub use psponge::poseidon_parameters_for_test_s;
