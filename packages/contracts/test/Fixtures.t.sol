// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

/**
 * @title Email Authentication Fixture Tests
 * @notice Test suite for verifying fixture data and basic functionality using test cases
 * @dev Tests email authentication message fixtures through EmailSigner and Verifier.
 *      The fixtures are used to test different command types:
 *      - Sign hash commands are tested via EmailSigner
 *      - Send ETH and guardian commands are tested via Verifier
 */

import "forge-std/Test.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {Verifier} from "../src/utils/Verifier.sol";
import {ECDSAOwnedDKIMRegistry} from "../src/utils/ECDSAOwnedDKIMRegistry.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {EmailSigner} from "../src/EmailSigner.sol";
import {EmailAuthMsg} from "../src/interfaces/IEmailTypes.sol";
import {EmailAuthMsgFixtures} from "./fixtures/EmailAuthMsgFixtures.sol";
import {Groth16Verifier} from "./fixtures/Groth16Verifier.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

contract FixturesTest is Test {
    // ============ Test Environment ============

    /// @dev Contract instances needed for fixture testing
    ECDSAOwnedDKIMRegistry dkimRegistry;
    Verifier verifier;
    address signerImpl;

    /// @dev Test account for signing operations
    address testSigner;
    uint256 testSignerPrivateKey;

    // ============ Setup ============

    /// @notice Sets up the test environment with necessary contracts
    /// @dev Deploys minimal contract setup needed to test fixtures
    function setUp() public {
        // Setup test signer for DKIM registry
        testSignerPrivateKey = uint256(
            keccak256(abi.encodePacked("test signer key"))
        );
        testSigner = vm.addr(testSignerPrivateKey);

        // Deploy minimal contract setup for fixture testing
        signerImpl = address(new EmailSigner());

        // Setup verifier with test configuration
        verifier = Verifier(
            address(
                new ERC1967Proxy(
                    address(new Verifier()),
                    abi.encodeWithSelector(
                        Verifier.initialize.selector,
                        testSigner,
                        address(new Groth16Verifier())
                    )
                )
            )
        );

        // Setup DKIM registry with test configuration
        dkimRegistry = ECDSAOwnedDKIMRegistry(
            address(
                new ERC1967Proxy(
                    address(new ECDSAOwnedDKIMRegistry()),
                    abi.encodeWithSelector(
                        ECDSAOwnedDKIMRegistry.initialize.selector,
                        address(this),
                        testSigner
                    )
                )
            )
        );
    }

    // ============ Fixture Tests ============

    /// @notice Tests fixture data for sign hash command (case 1)
    /// @dev Verifies the first sign hash command fixture works with EmailSigner
    function testSignHashCommand1() public {
        _testSignHashCommand(EmailAuthMsgFixtures.getCase1());
    }

    /// @notice Tests fixture data for sign hash command (case 2)
    /// @dev Verifies the second sign hash command fixture works with EmailSigner
    function testSignHashCommand2() public {
        _testSignHashCommand(EmailAuthMsgFixtures.getCase2());
    }

    /// @notice Tests fixture data for send ETH command
    /// @dev Verifies the send ETH command fixture works with Verifier
    function testSendEthCommand() public view {
        Verifier(verifier).verifyEmailProof(
            EmailAuthMsgFixtures.getCase3().proof
        );
    }

    /// @notice Tests fixture data for guardian request
    /// @dev Verifies the guardian request fixture works with Verifier
    function testAcceptGuardianRequst() public view {
        Verifier(verifier).verifyEmailProof(
            EmailAuthMsgFixtures.getCase4().proof
        );
    }

    // ============ Helper Functions ============

    /// @notice Helper to test sign hash command fixtures
    /// @param emailAuthMsg The fixture email authentication message to test
    function _testSignHashCommand(EmailAuthMsg memory emailAuthMsg) internal {
        EmailSigner(_deployEmailSignerProxyBasedOnEmailAuthMsg(emailAuthMsg))
            .verifyEmail(emailAuthMsg);
    }

    /// @notice Sets up an EmailSigner instance for testing a fixture
    /// @param emailAuthMsg The fixture email authentication message
    /// @return emailSignerProxyAddr The address of the configured EmailSigner
    function _deployEmailSignerProxyBasedOnEmailAuthMsg(
        EmailAuthMsg memory emailAuthMsg
    ) internal returns (address emailSignerProxyAddr) {
        // Configure DKIM registry for the fixture if needed
        if (
            !dkimRegistry.isDKIMPublicKeyHashValid(
                emailAuthMsg.proof.domainName,
                emailAuthMsg.proof.publicKeyHash
            )
        ) {
            _setupDKIMRegistry(emailAuthMsg);
        }

        // Setup EmailSigner for the fixture
        return
            address(
                new ERC1967Proxy(
                    signerImpl,
                    abi.encodeWithSelector(
                        EmailSigner.initialize.selector,
                        emailAuthMsg.proof.accountSalt,
                        address(dkimRegistry),
                        verifier,
                        emailAuthMsg.templateId
                    )
                )
            );
    }

    /// @notice Configures DKIM registry for a fixture
    /// @param emailAuthMsg The fixture containing DKIM data to register
    function _setupDKIMRegistry(EmailAuthMsg memory emailAuthMsg) internal {
        string memory signedMsg = dkimRegistry.computeSignedMsg(
            dkimRegistry.SET_PREFIX(),
            emailAuthMsg.proof.domainName,
            emailAuthMsg.proof.publicKeyHash
        );

        bytes32 digest = MessageHashUtils.toEthSignedMessageHash(
            bytes(signedMsg)
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(testSignerPrivateKey, digest);
        bytes memory signature = abi.encodePacked(r, s, v);

        dkimRegistry.setDKIMPublicKeyHash(
            "DEFAULT", // Registry identifier (not validated in current implementation)
            emailAuthMsg.proof.domainName,
            emailAuthMsg.proof.publicKeyHash,
            signature
        );
    }
}
