use ark_r1cs_std::{
    prelude::{AllocVar, Boolean, EqGadget, FieldVar, UInt8, R1CSVar},
};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError, ConstraintSynthesizer};
use ark_ff::{Field, PrimeField};
use ark_groth16::{Groth16};
use ark_bls12_381::{Bls12_381, Fr};
use rand::rngs::StdRng;
use rand::SeedableRng;
use ark_snark::SNARK;
use ark_r1cs_std::fields::fp::FpVar;
use ark_std::cmp::Ordering;

/// Defines FibonacciCircuit
#[derive(Clone)]
struct FibonacciCircuit<F: PrimeField> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub n: Option<F>,
    pub numb_of_constraints: usize,
}

impl<F: PrimeField > ConstraintSynthesizer<F> for FibonacciCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // let n the number of the fibonacci sequence to prove:
        let n = FpVar::<F>::new_input(cs.clone(), || self.n.ok_or(SynthesisError::AssignmentMissing))?;
        // Allocate the first two numbers of the sequence as witness variables:
        let mut fi_minus_one = FpVar::<F>::new_witness(cs.clone(), || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let mut fi_minus_two = FpVar::<F>::new_witness(cs.clone(), || self.b.ok_or(SynthesisError::AssignmentMissing))?;
        // allocate fi as an input variable - this is the number we're interested in computing
        let mut fi = FpVar::<F>::constant(F::zero());
    
        for _i in 0..self.numb_of_constraints - 1 {
            // Allocate the next number in the sequence
            fi = fi_minus_one.clone() + &fi_minus_two;
            
            // Enforce the constraint fi = fi_minus_one + fi_minus_two
            fi.enforce_equal(&(&fi_minus_one + &fi_minus_two))?;

            // Update the previous two numbers in the sequence
            fi_minus_two = fi_minus_one;
            fi_minus_one = fi.clone();
            println!("fi: {:?}", fi.value());

        }

        // Check if the computed Fibonacci number is less than the input number n:
        fi.enforce_cmp(&n, Ordering::Greater, true)?;
        Ok(())
    }
}

fn should_verify_fibonacci_circuit_groth16() -> bool {
    let mut rng = StdRng::seed_from_u64(0u64);
    let c = FibonacciCircuit::<Fr> {
        a: Some(Fr::from(1)), // Initial value for Fi_minus_one
        b: Some(Fr::from(0)), // Initial value for Fi_minus_two
        n: Some(Fr::from(21)), // the number of the fibonacci sequence to prove
        numb_of_constraints: 8,// Number of steps to perform in the sequence
    };
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(c.clone(), &mut rng).unwrap();
    let proof = Groth16::<Bls12_381>::prove(&pk, c.clone(), &mut rng).unwrap();
    if let Err(err) = Groth16::<Bls12_381>::verify(&vk, &vec![c.n.unwrap()], &proof) {
        eprintln!("Error verifying proof: {:?}", err);
        return false;
    }    

    true
}

fn main() {
    let result = should_verify_fibonacci_circuit_groth16();
    if !result {
        eprintln!("Circuit constraints are not satisfied.");
    }
}
