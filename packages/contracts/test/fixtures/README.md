# Test Fixtures

This directory contains test fixtures for verifying email authentication proofs in the zk-email system. These fixtures are used to test different types of email-based commands through the EmailSigner and Verifier contracts.

## Directory Structure

```
fixtures/
├── case1_signHash/       # First sign hash command test case
├── case2_signHash/       # Second sign hash command test case
├── case3_sendEth/        # Send ETH command test case
├── case4_acceptGuardian/ # Guardian acceptance test case
├── EmailAuthMsgFixtures.sol  # Solidity wrapper for test cases
├── Groth16Verifier.sol      # Auto-generated zk proof verifier
└── README.md
```

Each case directory contains:
- `raw.eml` - Original email file used to generate the proof
- `EmailAuthMsg.json` - Generated proof data for Solidity contract verification

## Test Cases

The fixtures cover four main test scenarios:

1. **Sign Hash (Case 1)** - Tests email-based signing of a specific hash value
   - Verified through EmailSigner contract
   - Located in `case1_signHash/`

2. **Sign Hash (Case 2)** - Tests email-based signing with a different hash value
   - Verified through EmailSigner contract
   - Located in `case2_signHash/`

3. **Send ETH (Case 3)** - Tests email-based ETH transfer command
   - Verified through Verifier contract
   - Located in `case3_sendEth/`

4. **Accept Guardian (Case 4)** - Tests email-based guardian acceptance
   - Verified through Verifier contract
   - Located in `case4_acceptGuardian/`

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

### Verification Process

The fixtures are verified using two different contracts based on the command type:

1. **EmailSigner Contract** - Verifies sign hash commands (Case 1 & 2)
   ```solidity
   EmailSigner(signerAddr).verifyEmail(emailAuthMsg);
   ```

2. **Verifier Contract** - Verifies send ETH and guardian commands (Case 3 & 4)
   ```solidity
   Verifier(verifierAddr).verifyEmailProof(emailAuthMsg.proof);
   ```

## ZK Proof Verification

The `Groth16Verifier.sol` is an auto-generated contract using snarkjs that handles the zero-knowledge proof verification. It's used internally by the EmailSigner and Verifier contracts to validate the proofs contained in the test fixtures.

For examples of how these fixtures are used in tests, refer to `test/Fixtures.t.sol`.
