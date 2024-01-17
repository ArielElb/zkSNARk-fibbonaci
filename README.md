# Fibonacci Sequence Proof using Groth16

This program implements a proof of the Fibonacci sequence using the Groth16 zk-SNARK (Zero-Knowledge Succinct Non-Interactive Argument of Knowledge) system. It allows you to prove the correctness of a specific Fibonacci number within the sequence.

## Prerequisites

- [Rust](https://www.rust-lang.org/) programming language and Cargo package manager.
- Dependencies are specified in the `Cargo.toml` file.

## Usage

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/fibonacci-proof.git
    cd fibonacci-proof
    ```

2. Build the program:

    ```bash
    cargo build --release
    ```

3. Run the program:

    ```bash
    cargo run --release
    ```

4. Follow the prompts to enter the index of the Fibonacci number and the number of steps to perform in the sequence.

## Input

The program prompts the user to enter the following:

- Index of the Fibonacci number to prove.
- Number of steps to perform in the Fibonacci sequence.

## Output

The program generates a proof using the Groth16 zk-SNARK system and verifies the proof's correctness.

## Example

```bash
Enter the index of the N Fibonacci number - number of steps:
9
The number of steps is: 9
Verification successful: the Fibonacci number at index 9 is correctly computed.
