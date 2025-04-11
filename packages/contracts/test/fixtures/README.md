# Test Fixtures

This directory contains test fixtures for verifying the EmailAuth(includes EmailSigner and recovery) circuit proofs.

## Directory Structure

The `Groth16Verifier.sol` file has been auto-generated using snarkjs and can be used to verify the proofs listed here.



Each subdirectory represents a test case with:
- `email.eml`: The original email file
- `input.json`: Generated input from the email
- `public.json`: Public inputs for the proof
- `proof.json`: The actual proof data

## Note

Only `proof.json` and `public.json` are needed for the final verification of the proof. The `email.eml` and `input.json` files are provided just for reference and to understand the context of the test case.


## Usage
