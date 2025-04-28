# Test Fixtures

This directory contains test fixtures for `EmailAuthMsg` proofs. These fixtures can be used to verify `EmailAuthMsg` in any context. For example, in these tests, we use both the `EmailSigner` contract and the lower-level primitive, `Verifier`, to validate the proofs. JSON-encoded versions of the fixtures are also provided, along with a Solidity library to easily import and use the data in smart contracts.


## Directory Structure

```
fixtures/
├── case1_signHash/         # First sign hash command test case
│   ├── raw.eml             # Email sent by the relayer to the user (reference only, not used in verification)
│   └── EmailAuthMsg.json   # JSON-encoded `EmailAuthMsg` generated from the user's response to raw.eml
├── case2_signHash/         # Second sign hash command test case
│   ├── raw.eml
│   └── EmailAuthMsg.json
├── case3_sendEth/          # Send ETH command test case
│   ├── raw.eml
│   └── EmailAuthMsg.json
├── case4_acceptGuardian/   # Guardian acceptance test case
│   ├── raw.eml
│   └── EmailAuthMsg.json
├── EmailAuthMsgFixtures.sol  # Solidity wrapper for test cases
├── Groth16Verifier.sol       # Auto-generated zk proof verifier
└── README.md


```

## Test Cases

The fixtures cover four main test scenarios:

1. **Sign Hash (Case 1)** - Tests signing of a specific hash value
   - Verified through EmailSigner contract
   - Command Template: `signHash {uint}`
   - Command: `signHash 98795965305811853593942095979598763998273224478639454298374304707044663517522`

3. **Sign Hash (Case 2)** - Tests signing with a different hash value
   - Same as Case 1
   - Command: `signHash 62817409320148730591830218376583920457123489321048213478932100011234567890123`

4. **Send ETH (Case 3)** - Tests ETH transfer command
   - Verified through Verifier contract
   - Command Template: `Send {decimals} ETH to {ethAddr}`
   - Command: `Send 0.1 ETH to 0xafBD210c60dD651892a61804A989eEF7bD63CBA0`

5. **Accept Guardian (Case 4)** - Tests guardian acceptance
   - Verified through Verifier contract
   - Command Template: `Accept guardian request for {ethAddr}`
   - Command: `Accept guardian request for 0xafBD210c60dD651892a61804A989eEF7bD63CBA0`

## Usage

### Solidity Interface

The `EmailAuthMsgFixtures.sol` provides a convenient Solidity interface to access these test cases:

```solidity
import {EmailAuthMsgFixtures} from "./fixtures/EmailAuthMsgFixtures.sol";

// Get test cases
EmailAuthMsg memory case1 = EmailAuthMsgFixtures.getCase1(); // Sign Hash
EmailAuthMsg memory case2 = EmailAuthMsgFixtures.getCase2(); // Sign Hash
EmailAuthMsg memory case3 = EmailAuthMsgFixtures.getCase3(); // Send ETH
EmailAuthMsg memory case4 = EmailAuthMsgFixtures.getCase4(); // Accept Guardian
```

you can also import `EmailAuthMsg.json` in hardhat tests or anywhere really

### Verification Process

For refrence the fixtures are verified using two different contracts based on the command type:

1. **EmailSigner Contract** - Verifies sign hash commands (Case 1 & 2)
   ```solidity
   EmailSigner(signerAddr).verifyEmail(emailAuthMsg);
   ```

2. **Verifier Contract** - Verifies send ETH and guardian commands (Case 3 & 4)
   ```solidity
   Verifier(verifierAddr).verifyEmailProof(emailAuthMsg.proof);
   ```

## Gorth16Verifier

The `Groth16Verifier.sol` is an auto-generated contract using snarkjs that handles the zero-knowledge proof verification. It's used internally by the EmailSigner and Verifier contracts to validate the proofs contained in the test fixtures.

For examples of how these fixtures are used in tests, refer to `Fixtures.t.sol`.
