use std::time::Instant;
use ark_r1cs_std::{fields::fp::FpVar, prelude::AllocVar};
use ark_sponge::{ CryptographicSponge, FieldBasedCryptographicSponge, poseidon::PoseidonSponge};

//use ark_crypto_primitives::SNARK;
//use ark_groth16::*;
//use ark_serialize::CanonicalSerialize;
//use ark_serialize::CanonicalDeserialize;
//use ark_serialize::CanonicalDeserialize;
//use std::io::Write; // bring trait into scope
//use std::fs::*;

// added by zeroknight
use poseidongr16::{SIZEOFINPUT, SIZEOFOUTPUT, Fr,poseidon_parameters_for_test_s };
use poseidongr16::{groth_verify_s,groth_proof_gen_s, groth_param_gen_s, SPNGCircuit };    // Fr
use ark_relations::r1cs::ConstraintSystem;
use ark_std::{test_rng};
use ark_relations::*;
use ark_sponge::poseidon::constraints::PoseidonSpongeVar;
use ark_ff::UniformRand;
use ark_sponge::constraints::CryptographicSpongeVar;
use ark_r1cs_std::R1CSVar;

#[test]
fn test_circuit_working() {     // zeroknight
    let mut rng = ark_std::test_rng();
        let cs = ConstraintSystem::new_ref();

        let absorb1: Vec<_> = (0..SIZEOFINPUT).map(|_| Fr::rand(&mut rng)).collect();
        let absorb1_var: Vec<_> = absorb1
            .iter()
            .map(|v| FpVar::new_input(ns!(cs, "absorb1"), || Ok(*v)).unwrap())
            .collect(); 

        let sponge_params = poseidon_parameters_for_test_s();

        let mut native_sponge = PoseidonSponge::<Fr>::new(&sponge_params);
        let mut constraint_sponge = PoseidonSpongeVar::<Fr>::new(cs.clone(), &sponge_params);

        native_sponge.absorb(&absorb1);
        constraint_sponge.absorb(&absorb1_var).unwrap();

        let squeeze1 = native_sponge.squeeze_native_field_elements(SIZEOFOUTPUT);
        let squeeze2 = constraint_sponge.squeeze_field_elements(SIZEOFOUTPUT).unwrap();
       
        let c =squeeze2.value().unwrap();

        assert_eq!(c, squeeze1);
        assert!(cs.is_satisfied().unwrap());


}


fn main() {
	let start_param_gen = Instant::now();

    //let seed =  &[32u8; 32];
    //let mut rng = ChaCha20Rng::from_seed(*seed);
    const INPTEXT:&str ="Input ...";
    const LEN : usize=  INPTEXT.len();
    let input = [
        INPTEXT.as_ref(),
        [0u8; SIZEOFINPUT - LEN].as_ref(),
    ]
   .concat();
    let inp =input;
    //let inp =[1u8; SIZEOFINPUT].to_vec();
   // let inp: Vec<_> = (0..4).map(|_| Fr::rand(&mut rng)).collect();
	//output
    let  parameter = poseidon_parameters_for_test_s();
    //let spongparams= <PoseidonSponge<Fr> as CryptographicSponge>::new(&parameter);
    let mut native_sponge = PoseidonSponge::< >::new(&parameter);
    native_sponge.absorb(&inp);
	//let out = inp.to_sponge_field_elements_as_vec();
    let out=native_sponge.squeeze_native_field_elements(SIZEOFOUTPUT);
	println!("out ={:?}",out);

    // temp



    // build the circuit
    let circuit = SPNGCircuit {
        param: parameter.clone(),
        input: inp,
        output: out.clone(),
    };
    let elapsed_param_gen = start_param_gen.elapsed();
    println!("time to generate public paremeters and comm: {:?}", elapsed_param_gen);

   // Codes to use serialization and  Deserialization
/*
    let start2 = Instant::now();
    // generate a zkp parameters
    let zk_param = groth_param_gen_s(parameter);
    //let elapse2 = start2.elapsed();

    let mut kpar2 : Vec<u8> = vec![];

    zk_param.serialize_unchecked(&mut kpar2).unwrap() ;
    println!("size of zk_param = {:?}",kpar2.len());
    #[cfg(feature="bls12_377")]
    let mut buffer2 = File::create("poseidon_zk_param_bls12_377.txt").unwrap();
    #[cfg(feature="bls12_381")]
    let mut buffer2 = File::create("poseidon_zk_param_bls12_381.txt").unwrap();
    #[cfg(feature="bn254c")]
    let mut buffer2 = File::create("poseidon_zk_param_bn254c.txt").unwrap();
    buffer2.write_fmt(format_args!("{:.?}", &kpar2)).unwrap();

  */

   // load zk_param from file 
   /* 
    let zk_param1 = std::fs::read_to_string("./poseidon_zk_param_bn254c_p.txt").unwrap();

     
    let numbers2: Vec<u8> = zk_param1
     .split_whitespace()
     .map(|s| s.parse().expect("parse error"))
     .collect(); 


    struct Parameter<'a> {
        data: &'a [u8],
           }
    let zk_param2 :Parameter = Parameter {
            data: &numbers2,
        };
    // let zkpa :Parameter = Parameter {
    //         data: &kpar2,
    //     };
        //let ped    
    let start2 = Instant::now();      
    let zk_param =
    <Groth16<CurveTypeG> as SNARK<Fr>>::ProvingKey::deserialize(zk_param2.data)
    .unwrap();  
    
    let elapse2 = start2.elapsed();
   

    let mut kpar3 : Vec<u8> = vec![];
    zk_param.serialize(&mut kpar3).unwrap() ;
    println!("size of zk_param = {:?}",kpar3.len());
    */

    //load zk_param (In case of using (de)serialization)
    //let start_load_zk_param = Instant::now();
    // let zk_param =
    //     <Groth16<CurveTypeG> as SNARK<Fr>>::ProvingKey::deserialize_unchecked(ZK_PARAM.data)
    //     .unwrap(); 
    //let elapsedload = start_load_zk_param.elapsed();
    //println!("time to load deser. = {:?}", elapsedload);

    // generate ZK_param
    let start_zk_param = Instant::now();
    let zk_param = groth_param_gen_s(parameter);
    let elapsed_zk_param = start_zk_param.elapsed();
    println!("time to gen zk_param: {:?}", elapsed_zk_param);

    let start_proof = Instant::now();
    
    let proof = groth_proof_gen_s(&zk_param, circuit, &[32u8; 32]);

    let elapse_proof = start_proof.elapsed();
    println!("time to gen proof: {:?}", elapse_proof);

    let start_verify = Instant::now();
    assert!(groth_verify_s(&zk_param, &proof, &out));
    let elapse_verify = start_verify.elapsed();
    println!("time to verify proof: {:?}", elapse_verify);

}
    
