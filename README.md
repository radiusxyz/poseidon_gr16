# Example for ZKP of POSEIDON using Groth16

This repo contains an example of building zero-knowlege prover-verifier instances using Arkworks' zkSNARK implementation for: 

*  Sponge-based Poseidon hash function



To choose one of the curves (Bls12-377, Bls12-381, Bn_254), change the default ```[features]``` in the Cargo.toml file in each example; 

To change the size of input, change the value of ```SIZEOFINPUT``` in lib.rs.

To change the size of output for Sponge-based Poseidon hash function, change the value of ```SIZEOFOUTPUT``` in the lib.rs. 

To benchmark, run:
``` bash 
cargo bench 
```
