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
import {IEmailAuth} from "../src/interfaces/IEmailAuth.sol";
import {IEmailAuthErrors} from "../src/interfaces/IEmailAuth.sol";
import {ERC1271} from "../src/libraries/ERC1271.sol";
contract EmailSignerTest is SignerStructHelper, IEmailAuthErrors {
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

    function testUpdateDKIMRegistryToECDSA() public {
        assertEq(emailSigner.dkimRegistryAddr(), address(dkim));

        vm.startPrank(deployer);
        ECDSAOwnedDKIMRegistry newDKIM;
        {
            ECDSAOwnedDKIMRegistry dkimImpl = new ECDSAOwnedDKIMRegistry();
            ERC1967Proxy dkimProxy = new ERC1967Proxy(
                address(dkimImpl),
                abi.encodeCall(dkimImpl.initialize, (msg.sender, msg.sender))
            );
            newDKIM = ECDSAOwnedDKIMRegistry(address(dkimProxy));
        }
        vm.expectEmit(true, false, false, false);
        emit IEmailAuth.DKIMRegistryUpdated(address(newDKIM));
        emailSigner.updateDKIMRegistry(address(newDKIM));
        vm.stopPrank();

        assertEq(emailSigner.dkimRegistryAddr(), address(newDKIM));
    }

    function testExpectRevertUpdateDKIMRegistryInvalidDkimRegistryAddress()
        public
    {
        assertEq(emailSigner.dkimRegistryAddr(), address(dkim));

        vm.startPrank(deployer);
        vm.expectRevert(InvalidDKIMRegistryAddress.selector);
        emailSigner.updateDKIMRegistry(address(0));
        vm.stopPrank();
    }

    function testUpdateVerifier() public {
        assertEq(emailSigner.verifierAddr(), address(verifier));

        vm.startPrank(deployer);
        Verifier newVerifier = new Verifier();
        vm.expectEmit(true, false, false, false);
        emit IEmailAuth.VerifierUpdated(address(newVerifier));
        emailSigner.updateVerifier(address(newVerifier));
        vm.stopPrank();

        assertEq(emailSigner.verifierAddr(), address(newVerifier));
    }

    function testExpectRevertUpdateVerifierInvalidVerifierAddress() public {
        assertEq(emailSigner.verifierAddr(), address(verifier));

        vm.startPrank(deployer);
        vm.expectRevert(InvalidVerifierAddress.selector);
        emailSigner.updateVerifier(address(0));
        vm.stopPrank();
    }

    function testVerifyEmail() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailSigner.verifyEmail(emailAuthMsg); // should not revert
    }

    function testExpectRevertVerifyEmailInvalidDkimPublicKeyHash() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.proof.domainName = "invalid.com";

        vm.expectRevert(InvalidDKIMPublicKeyHash.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidAccountSalt() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.proof.accountSalt = bytes32(uint256(1234));

        vm.expectRevert(InvalidAccountSalt.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidCommand() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.commandParams[0] = abi.encode(2 ether);

        vm.expectRevert(InvalidCommand.selector);
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
        vm.expectRevert(InvalidEmailProof.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidMaskedCommandLength() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);

        // Set masked command length to 606, which should be 605 or less defined in the verifier.
        emailAuthMsg.proof.maskedCommand = string(new bytes(606));

        vm.expectRevert(InvalidMaskedCommandLength.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testExpectRevertVerifyEmailInvalidSizeOfTheSkippedCommandPrefix()
        public
    {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);

        // Set skipped command prefix length to 605, it should be less than 605.
        emailAuthMsg.skippedCommandPrefix = 605;

        vm.expectRevert(InvalidSkippedCommandPrefixSize.selector);
        emailSigner.verifyEmail(emailAuthMsg);
    }

    function testUpgradeEmailAuth() public {
        vm.startPrank(deployer);

        // Deploy new implementation
        EmailSigner newImplementation = new EmailSigner();

        bytes32 beforeAccountSalt = emailSigner.accountSalt();

        // Upgrade to new implementation through proxy
        emailSigner.upgradeToAndCall(address(newImplementation), new bytes(0));

        bytes32 afterAccountSalt = emailSigner.accountSalt();

        // Verify the upgrade
        assertEq(beforeAccountSalt, afterAccountSalt);

        vm.stopPrank();
    }

    function testExpectRevertUpgradeEmailAuthNotDeployer() public {
        // Deploy new implementation
        EmailSigner newImplementation = new EmailSigner();
        address alice = makeAddr("alice");
        // Try to upgrade from non-deployer account
        vm.startPrank(alice);
        vm.expectRevert();
        emailSigner.upgradeToAndCall(address(newImplementation), new bytes(0));
        vm.stopPrank();
    }

    function testExpectRevertVerifyEmailInvalidTemplateId() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg(msgHash);
        emailAuthMsg.templateId = uint256(1234); // Different template ID than the one set in initialization

        vm.expectRevert(InvalidTemplateId.selector);
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
        assertEq(result, ERC1271.MAGIC_VALUE);

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
        assertEq(result, ERC1271.LEGACY_MAGIC_VALUE);

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
        vm.expectRevert(InvalidAccountSalt.selector);
        emailSigner.isValidSignature(msgHash, signature);
    }
}
