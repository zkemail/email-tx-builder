// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "../src/utils/Verifier.sol";
import "../src/utils/ECDSAOwnedDKIMRegistry.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "./helpers/SignerStructHelper.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

contract EmailSignerTest is SignerStructHelper {
    string message = "message to sign";
    bytes32 msgHash = keccak256(abi.encode(message));

    function testDkimRegistryAddr() public view {
        address dkimAddr = emailSigner.dkimRegistryAddr();
        assertEq(dkimAddr, address(dkim));
    }

    function testVerifierAddr() public view {
        address verifierAddr = emailSigner.verifierAddr();
        assertEq(verifierAddr, address(verifier));
    }

    function testVerifyEmail() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailSigner.verifyEmail(emailAuthMsg); // should not revert
    }

    function testExpectRevertVerifyEmailInvalidDkimPublicKeyHash() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.proof.domainName = "invalid.com";

        vm.expectRevert(EmailSigner.InvalidDKIMPublicKeyHash.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidAccountSalt() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.proof.accountSalt = bytes32(uint256(1234));

        vm.expectRevert(EmailSigner.InvalidAccountSalt.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidCommand() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.commandParams[0] = abi.encode(2 ether);

        vm.expectRevert(EmailSigner.InvalidCommand.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidEmailProof() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);

        vm.mockCall(
            address(verifier),
            abi.encodeWithSelector(
                Verifier.verifyEmailProof.selector,
                emailAuthMsg.proof
            ),
            abi.encode(false)
        );
        vm.expectRevert(EmailSigner.InvalidEmailProof.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidMaskedCommandLength() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);

        // Set masked command length to 606, which should be 605 or less defined in the verifier.
        emailAuthMsg.proof.maskedCommand = string(new bytes(606));

        vm.expectRevert(EmailSigner.InvalidMaskedCommandLength.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidSkippedCommandPrefixGreaterThan()
        public
    {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);

        // Set skipped command prefix length to greater than command bytes (605)
        emailAuthMsg.skippedCommandPrefix = 606;

        vm.expectRevert(EmailSigner.InvalidSkippedCommandPrefixSize.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidSkippedCommandPrefixEqualTo()
        public
    {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);

        // Set skipped command prefix length equal to command bytes (605)
        emailAuthMsg.skippedCommandPrefix = 605;

        vm.expectRevert(EmailSigner.InvalidSkippedCommandPrefixSize.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidTemplateId() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.templateId = uint256(1234); // Different template ID than the one set in initialization

        vm.expectRevert(EmailSigner.InvalidTemplateId.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testIsValidSignature() public {
        // Create test data
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.commandParams[0] = abi.encode(msgHash); // Set the hash as the parameter

        // Encode the EmailAuthMsg as the signature
        bytes memory signature = abi.encode(emailAuthMsg);

        // Test valid signature
        bytes4 result = emailSigner.isValidSignature(msgHash, signature);
        assertEq(result, bytes4(0x1626ba7e));

        // Test invalid hash
        bytes32 wrongHash = keccak256("wrong message");
        result = emailSigner.isValidSignature(wrongHash, signature);
        assertEq(result, bytes4(0));
    }

    function testIsValidSignatureLegacy() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.commandParams[0] = abi.encode(msgHash); // Set the hash as the parameter

        // Encode the EmailAuthMsg as the signature
        bytes memory signature = abi.encode(emailAuthMsg);

        // Test valid signature
        bytes4 result = emailSigner.isValidSignature(
            abi.encode(message),
            signature
        );
        assertEq(result, bytes4(0x20c13b0b));

        // Test invalid data
        bytes memory wrongData = bytes("wrong message");
        result = emailSigner.isValidSignature(abi.encode(wrongData), signature);
        assertEq(result, bytes4(0));
    }

    function testExpectRevertIsValidSignatureInvalidEmailAuth() public {
        // Create test data with invalid EmailAuthMsg
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.proof.accountSalt = bytes32(uint256(1234)); // Invalid account salt

        // Encode the EmailAuthMsg as the signature
        bytes memory signature = abi.encode(emailAuthMsg);

        // Should revert with InvalidAccountSalt
        vm.expectRevert(EmailSigner.InvalidAccountSalt.selector);
        emailSigner.isValidSignature(msgHash, signature);
    }

    function testExpectRevertInitializeInvalidDKIMRegistryAddress() public {
        // Create new implementation
        EmailSigner emailSignerImpl = new EmailSigner();

        // Try to initialize with zero address for DKIM registry
        bytes memory initData = abi.encodeWithSelector(
            EmailSigner.initialize.selector,
            accountSalt,
            address(0), // Invalid DKIM registry address
            address(verifier),
            templateId
        );

        vm.expectRevert(EmailSigner.InvalidDKIMRegistryAddress.selector);
        new ERC1967Proxy(address(emailSignerImpl), initData);
    }

    function testExpectRevertInitializeInvalidVerifierAddress() public {
        // Create new implementation
        EmailSigner emailSignerImpl = new EmailSigner();

        // Try to initialize with zero address for verifier
        bytes memory initData = abi.encodeWithSelector(
            EmailSigner.initialize.selector,
            accountSalt,
            address(dkim),
            address(0), // Invalid verifier address
            templateId
        );

        vm.expectRevert(EmailSigner.InvalidVerifierAddress.selector);
        new ERC1967Proxy(address(emailSignerImpl), initData);
    }
}
