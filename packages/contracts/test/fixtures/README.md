# Test Fixtures

This directory contains test fixtures for verifying the EmailAuth(includes EmailSigner and recovery) circuit proofs.

## Directory Structure

The `Groth16Verifier.sol` file has been auto-generated using snarkjs and can be used to verify the proofs listed here.



Each subdirectory represents a test case with:
- `raw.eml`: The original email file
- `EmailAuthMsg.json`: proof generated from the raw email file that can be verified in solidity contracts

For convenience, there is also an `EmailAuthMsgFixtures.sol` file that wraps all the cases in a Solidity library. This library provides functions to retrieve the `EmailAuthMsg` for each test case:

- `getCase1()`: Sign Hash with a specific hash value
- `getCase2()`: Sign Hash with a different hash value
- `getCase3()`: Send ETH Transaction
- `getCase4()`: Accept Guardian Request

You can use these functions to easily access the test cases in your Solidity tests.
