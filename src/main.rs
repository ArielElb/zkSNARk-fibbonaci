use std::any::type_name;
use ark_r1cs_std::{
    prelude::{AllocVar, EqGadget, R1CSVar},
};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError, ConstraintSynthesizer};
use ark_ff::PrimeField;
use ark_groth16::Groth16;
use ark_bls12_381::{Bls12_381, Fr};
use rand::rngs::StdRng;
use rand::SeedableRng;
use ark_r1cs_std::fields::fp::FpVar;
use std::time::Instant;
use std::mem;
use std::env;
use ark_snark::SNARK;
use ark_std::str::FromStr;
use std::sync::Mutex;
use lazy_static::lazy_static;
//static mut GLOBAL_STRING: &str = "Your global string here";
lazy_static! {
    static ref GLOBAL_STRING: Mutex<String> = Mutex::new(String::new());
}
//static mut GLOBAL_VARIABLE:Option<Fr>  = Option::from(Fr::from_str("2").unwrap());
#[derive(Clone)]
struct FibonacciCircuit<F: PrimeField> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub numb_of_steps: usize,
    pub result: Option<F>,
}
fn fibonacci_steps(a: u64, b: u64, steps: u32) -> u64 {
    let mut x = a;
    let mut y = b;

    for _ in 0..steps {
        let next = x + y;
        x = y;
        y = next;
    }

    x
}
static mut START_TIME: Option<Instant> = None;
impl<F: PrimeField> ConstraintSynthesizer<F> for FibonacciCircuit<F> {
    fn generate_constraints(mut self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let mut fi_minus_one =
            FpVar::<F>::new_witness(cs.clone(), || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let mut fi_minus_two =
            FpVar::<F>::new_witness(cs.clone(), || self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let saved_result = FpVar::<F>::new_witness(cs.clone(), || self.result.ok_or(SynthesisError::AssignmentMissing))?;

        // initialize fi as public input
        let mut fi = FpVar::<F>::new_input(cs.clone(), || Ok(F::zero()))?;
        // do the loop only when verifying the circuit
        for _i in 0..self.numb_of_steps {
            fi = fi_minus_one.clone() + &fi_minus_two;
            fi.enforce_equal(&(&fi_minus_one + &fi_minus_two))?;
            fi_minus_two = fi_minus_one;
            fi_minus_one = fi.clone();
        }
        match fi.value() {
            Ok(val) => unsafe {
                // Do something with the value
                println!("Value of fi: {:?}", val.to_string());
                let val_str = val.to_string();
                let mut global_str = GLOBAL_STRING.lock().unwrap();
                *global_str = val_str;
                
                //GLOBAL_STRING = &val.to_string().clone();
                //println!("{}",val[0]);
                //println!("{}",fi.value().unwrap().type_name());
                //GLOBAL_VARIABLE = fi.value().unwrap();
            },
            Err(e) => {
                if e == SynthesisError::AssignmentMissing {
                    // Handle the AssignmentMissing error
                } else {
                    // Handle other types of errors
                }
            }
        }
        fi.enforce_equal(&(&saved_result))?;
        Ok(())
    }
}

fn input_number<F: PrimeField>(message: &str) -> F
where
    <F as FromStr>::Err: std::fmt::Debug,
{
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<F>().unwrap();
    n
}

fn should_verify_fibonacci_circuit_groth16(a: Fr, b: Fr, numb_of_steps: usize) -> bool {
    // set the seed for the random number generator as the security parameter :
    // 32 bytes for 256-bit security level, 48 bytes for 384-bit security level, and 64 bytes for 512-bit security level:
    let seed = [0u8; 32];
    let mut rng = StdRng::from_seed(seed);

    unsafe {
        START_TIME = Some(Instant::now());
    }
    let c = FibonacciCircuit::<Fr> {
        a: Some(a),
        b: Some(b),
        numb_of_steps,
        result: fibonacci_steps(a,b,num_of_steps), // Initialize fi as None
    };

    // Proving
    let prove_start_time = Instant::now();
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(c.clone(), &mut rng).unwrap();
    let proof = Groth16::<Bls12_381>::prove(&pk, c.clone(), &mut rng).unwrap();
    let prove_elapsed_time = prove_start_time.elapsed();
    
    eprintln!(
        "Proving time: {}.{:03} seconds",
        prove_elapsed_time.as_secs(),
        prove_elapsed_time.subsec_millis()
    );
    eprintln!("The size of the proof is: {} bytes", mem::size_of_val(&proof));
    // print the size of the bls12_381:

    
    // Verifying
    let verify_start_time = Instant::now();
    // let the inputs be num of steps
    let mut inputs = Vec::new();
    inputs.push(Fr::from_str(&numb_of_steps.to_string()).unwrap());
    let pvk = Groth16::<Bls12_381>::process_vk(&vk).unwrap();
    if let Err(_err) = Groth16::<Bls12_381>::verify_with_processed_vk(&pvk, &inputs, &proof) {
        eprintln!("Verification failed: your circuit constraints are not satisfied.");
        println!("Error: {:?}", _err);
        return false;
    }
    let verify_elapsed_time = verify_start_time.elapsed();
    eprintln!(
        "Verification time: {}.{:03} seconds",
        verify_elapsed_time.as_secs(),
        verify_elapsed_time.subsec_millis()
    );

    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <a> <b> <num_of_steps>", args[0]);
        return;
    }
    let a = Fr::from_str(&args[1]).unwrap();
    let b = Fr::from_str(&args[2]).unwrap();
    let power_from_user: u32 = args[3].parse().unwrap();
    let num_of_steps = 2u32.pow(power_from_user);

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("num_of_steps: {:?}", num_of_steps);

    let result = should_verify_fibonacci_circuit_groth16(a, b, num_of_steps as usize);
    let elapsed_time = unsafe { START_TIME.unwrap().elapsed() };
    if !result {
        eprintln!("Circuit constraints are not satisfied.");
    } else {
        println!("Circuit constraints are satisfied: your fibonacci can be calculated in the number of steps you entered.");
    }
    result.
    println!(
        "Total time taken: {}.{:03} seconds",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
    let global_str = GLOBAL_STRING.lock().unwrap();
    println!("{}", *global_str);
}
