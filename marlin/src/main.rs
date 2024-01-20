use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::{BigInteger, Field, PrimeField};
use ark_poly::univariate::DensePolynomial;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::{test_rng, UniformRand};
use ark_poly_commit::sonic_pc::SonicKZG10;
use ark_relations::lc;
use rand::{Error, RngCore};
use ark_sponge::{poseidon::PoseidonSponge, FieldBasedCryptographicSponge};
use crate::{Marlin, SimpleHashFiatShamirRng};

// Define a simple circuit to prove x * y = 42
struct MultiplicationCircuit<F: Field> {
    x: Option<F>,
    y: Option<F>,
    product: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for MultiplicationCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Ensure that x, y, and product are defined as witnesses
        let x = cs.new_witness_variable(|| self.x.ok_or(SynthesisError::AssignmentMissing))?;
        let y = cs.new_witness_variable(|| self.y.ok_or(SynthesisError::AssignmentMissing))?;
        let product = cs.new_witness_variable(|| self.product.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce_constraint(lc!(), lc!(),lc!())?;
        cs.enforce_constraint(lc!(), lc!(),lc!())?;
        cs.enforce_constraint(lc!(), lc!(),lc!())?;


        Ok(())
    }
}
struct FiatShamirPoseidonRng<F: PrimeField> {
    sponge: PoseidonSponge<F>,
}

impl<F: PrimeField> RngCore for FiatShamirPoseidonRng<F> {
    fn next_u32(&mut self) -> u32 {
        let random_field_element = self.sponge.squeeze_native_field_elements(1)[0];
        random_field_element.into_repr().as_ref()[0] as u32
    }

    fn next_u64(&mut self) -> u64 {
        let random_field_element = self.sponge.squeeze_native_field_elements(1)[0];
        random_field_element.into_repr().as_ref()[0]
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(F::BigInt::NUM_LIMBS * 8) {
            let random_field_elements = self.sponge.squeeze_native_field_elements(chunk.len() / (F::BigInt::NUM_LIMBS * 8));
            for (field_element, chunk) in random_field_elements.into_iter().zip(chunk.chunks_mut(F::BigInt::NUM_LIMBS * 8)) {
                chunk.copy_from_slice(&field_element.into_repr().to_bytes_le());
            }
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

fn main() {
    let rng = &mut test_rng();
    let max_degree = 3; // Set based on your circuit's requirements

    // Universal setup for SonicKZG10

// Create the Fiat-Shamir RNG using the hash function
    type PC = SonicKZG10<Bls12_381, DensePolynomial<Fr>>;
    let universal_srs = Marlin::<Fr, PC, SimpleHashFiatShamirRng<FiatShamirPoseidonRng<Fr>, PoseidonSponge<Fr>>>::universal_setup(max_degree, max_degree, max_degree, rng).unwrap();

    // Generate random secret values x and y
    let x = Fr::rand(rng);
    let y = Fr::rand(rng);

    // Calculate the product
    let product = x * y;

    // Create the circuit instance with the secret values
    let circuit = MultiplicationCircuit {
        x: Some(x),
        y: Some(y),
        product: Some(product),
    };

    // Prover generates a proof
    let (pk, vk) = Marlin::index(&universal_srs, circuit.clone()).unwrap();
    let proof = Marlin::prove(&pk, circuit, rng).unwrap();

    // Verifier checks the proof
    let public_input = vec![product];
    let verified = Marlin::verify(&vk, &public_input, &proof).unwrap();

    if verified {
        println!("Proof verified successfully!");
    } else {
        println!("Proof verification failed.");
    }
}

