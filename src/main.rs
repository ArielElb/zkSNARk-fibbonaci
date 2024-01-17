use ark_r1cs_std::{
    prelude::{AllocVar, EqGadget, FieldVar, R1CSVar},
};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError, ConstraintSynthesizer};
use ark_ff::{ PrimeField};
use ark_groth16::{Groth16};
use ark_bls12_381::{Bls12_381, Fr};
use rand::rngs::StdRng;
use rand::SeedableRng;
use ark_snark::SNARK;
use ark_r1cs_std::fields::fp::FpVar;
use ark_std::cmp::Ordering;
use ark_std::fmt::Debug;
use ark_std::str::FromStr;

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

fn input_number<F: PrimeField>(message: &str) -> F  where <F as FromStr>::Err: Debug {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<F>().unwrap();
    n
}
fn should_verify_fibonacci_circuit_groth16() -> bool {
    let mut rng = StdRng::seed_from_u64(0u64);
    // enter the number of the fibonacci sequence to prove from the user:

    let n = input_number::<Fr>("Enter the number of the fibonacci sequence to prove: ");
    // use standard input to get the number of constraints from the user:
    
    println!("Enter the index of the N Fibonacci number - number of steps: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Parse the input into a usize
    let num_of_step: usize = input.trim().parse().unwrap();

    println!("The number of steps is: {}", num_of_step);
    // Create an instance of the FibonacciCircuit:
    let c = FibonacciCircuit::<Fr> {
        a: Some(Fr::from(1)), // Initial value for Fi_minus_one
        b: Some(Fr::from(0)), // Initial value for Fi_minus_two
        n: Some(n), // The number of the fibonacci sequence to prove
        numb_of_constraints: num_of_step,// Number of steps to perform in the sequence
    };
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(c.clone(), &mut rng).unwrap();
    let proof = Groth16::<Bls12_381>::prove(&pk, c.clone(), &mut rng).unwrap();
    if let Err(_err) = Groth16::<Bls12_381>::verify(&vk, &vec![c.n.unwrap()], &proof) {

        eprintln!("Verification failed: your circuit constraints are not satisfied.");
        return false;
    }    

    true
}

fn main() {
    let result = should_verify_fibonacci_circuit_groth16();
    if !result {
        eprintln!("Circuit constraints are not satisfied.");
    }
    else {
        println!("Circuit constraints are satisfied: your fibonacci can be calculated in the number of steps you entered.");
    }
    
}
