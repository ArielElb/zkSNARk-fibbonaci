# Fibonacci Sequence Proof using Groth16

This program implements a proof of the Fibonacci sequence using the Groth16 zk-SNARK (Zero-Knowledge Succinct Non-Interactive Argument of Knowledge) system. It allows you to prove that we can know the Fibonacci number in the place 2^n steps when we get n from the user.
## Prerequisites

- [Rust](https://www.rust-lang.org/) programming language and Cargo package manager.
- Dependencies are specified in the `Cargo.toml` file.

## Usage

1. Clone the repository:

    ```bash
    git clone https://github.com/ArielElb/zkSNARk-fibbonaci.git
    cd zkSNARk-fibbonaci
    ```

2. Build and run the program:
    ```
    cargo run 0 1 5
     
    ```

## Input

The input to the program is received through the command line argument.
- first argument - the first number in the Fibonacci sequence.
- second argument - the second number in the Fibonacci sequence.
- third argument - Number of steps to perform in the Fibonacci sequence.

## Output

The program generates a proof using the Groth16 zk-SNARK system and verifies the proof's correctness.
the program will some important information about the proof-
- the size of the proof in bytes.
- Verification time.
- Proving time.
- Total time taken.

## Example

```bash
will be continue.
