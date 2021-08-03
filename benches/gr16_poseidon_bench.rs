#[macro_use]
extern crate criterion;
extern crate poseidongr16;

use criterion::{Criterion, black_box};
use ark_sponge::{ CryptographicSponge, FieldBasedCryptographicSponge, poseidon::PoseidonSponge};
use poseidongr16::*;

criterion_group!(
	sponge_bench,
    spongeg16tests
);
criterion_main!(sponge_bench);

fn spongeg16tests(c: &mut Criterion) {
    println!("/////////////////////////////////");
    const REPETITION : usize = 50;
    #[cfg(feature="bls12_381")]
    println!("Benchmark for Poseidon sponge using Bls12_381 curve");
    #[cfg(feature="bls12_377")]
    println!("Benchmark for Poseidon sponge using Bls12_377 curve");
    #[cfg(feature="bn254c")]
    println!("Benchmark for Poseidon sponge using Bn254 curve");
    
    println!("Input size is: {:?} u8 elements", SIZEOFINPUT);
	println!("Output size is: {:?} Fp256<FrParameters> = {:?} bytes", SIZEOFOUTPUT, SIZEOFOUTPUT*4*8 );
    println!("/////////////////////////////////");

    // Generating input for the benchmark. A way to provide random input can be:
    //   let seed =  &[32u8; 32];
    //   let mut rng = ChaCha20Rng::from_seed(*seed);
    //   let inp: Vec<_> = (0..4).map(|_| Fr::rand(&mut rng)).collect();
    // or fixed input like :
    //   let inp =[1u8; SIZEOFINPUT].to_vec();
    // However, we use the following input, assuming length of INPTEXT is less than SIZEOFINPUT

    const INPTEXT:&str ="This is the input ...";
    const LEN : usize=  INPTEXT.len();
    let input = [
        INPTEXT.as_ref(),
        [0u8; SIZEOFINPUT - LEN].as_ref(),
    ]
   .concat();
    let inp =input;

    // The parameters for Poseidon 
    let  parameter = poseidon_parameters_for_test_s();
    let  parameter2 = parameter.clone();

    // Defining a sponge based on Poseidon 
    let mut native_sponge = PoseidonSponge::< >::new(&parameter);
    // Feeding the input to the sponge 
    native_sponge.absorb(&inp);

    // Computing the output of the sponge, producing SIZEOFOUTPUT numbers of field elements for output
    let out=native_sponge.squeeze_native_field_elements(SIZEOFOUTPUT);
    //println!("out = {:?}", out);

    // build the circuit
    let circuit = SPNGCircuit {
        param: parameter.clone(),
        input: inp,
        output: out.clone(),
    };
    


    // generate a zkp parameters
    println!("start benchmarking grooth parameter generator");
	let mut bench_group = c.benchmark_group("ZKP_param");
    bench_group.sample_size(REPETITION);
	let bench_str = format!("ZKP Parameter");
	bench_group.bench_function(bench_str, move |b| {
		b.iter(|| black_box(groth_param_gen_s(parameter2.clone())))
	});

    let zk_param = groth_param_gen_s(parameter);
    let zk_param2=zk_param.clone();
    let circuit2 = circuit.clone();

    
    
    println!("start benchmarking proof generator");
    bench_group.sample_size(REPETITION);
	let bench_str = format!("ZKP proof");
	bench_group.bench_function(bench_str, move |b| {
		b.iter(|| black_box(groth_proof_gen_s(&zk_param2, circuit2.clone(), &black_box([32u8; 32]))))
	});

    let proof = groth_proof_gen_s(&zk_param, circuit, &[32u8; 32]);
    let proof2 = proof.clone();

    let zk_param3=zk_param.clone();
    let out2=out.clone();
   
    
    // Benchmark the verification if verification is successful 
    assert!(groth_verify_s(&zk_param, &proof, &out));

    println!("start benchmarking verification");
    bench_group.sample_size(REPETITION);
	let bench_str = format!("ZKP Verification");
	bench_group.bench_function(bench_str, move |b| {
		b.iter(||     black_box(groth_verify_s(&zk_param3, &proof2, &out2))
    )
	});
	bench_group.finish();
}