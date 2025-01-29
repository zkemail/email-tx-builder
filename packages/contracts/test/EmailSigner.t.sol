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

contract EmailSignerTest is SignerStructHelper, IEmailAuthErrors {
    function setUp() public override {
        super.setUp();

        vm.startPrank(deployer);
        // TODO: expect emit
        emailSigner.initialize(
            deployer,
            accountSalt,
            address(dkim),
            address(verifier),
            templateId
        );
        vm.stopPrank();
    }

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

    function testAuthEmail() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg();

        vm.expectEmit(true, true, true, true);
        emit IEmailAuth.EmailAuthed(
            emailAuthMsg.proof.emailNullifier,
            emailAuthMsg.proof.accountSalt,
            emailAuthMsg.proof.isCodeExist,
            emailAuthMsg.templateId
        );
        emailSigner.authEmail(emailAuthMsg);
    }

    function testExpectRevertAuthEmailInvalidDkimPublicKeyHash() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg();
        emailAuthMsg.proof.domainName = "invalid.com";

        vm.expectRevert(InvalidDKIMPublicKeyHash.selector);
        emailSigner.authEmail(emailAuthMsg);
    }

    function testExpectRevertAuthEmailInvalidAccountSalt() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg();
        emailAuthMsg.proof.accountSalt = bytes32(uint256(1234));

        vm.expectRevert(InvalidAccountSalt.selector);
        emailSigner.authEmail(emailAuthMsg);
    }

    function testExpectRevertAuthEmailInvalidCommand() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg();
        emailAuthMsg.commandParams[0] = abi.encode(2 ether);

        vm.expectRevert(InvalidCommand.selector);
        emailSigner.authEmail(emailAuthMsg);
    }

    function testExpectRevertAuthEmailInvalidEmailProof() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg();

        vm.mockCall(
            address(verifier),
            abi.encodeWithSelector(
                Verifier.verifyEmailProof.selector,
                emailAuthMsg.proof
            ),
            abi.encode(false)
        );
        vm.expectRevert(InvalidEmailProof.selector);
        emailSigner.authEmail(emailAuthMsg);
    }

    function testExpectRevertAuthEmailInvalidMaskedCommandLength() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg();

        // Set masked command length to 606, which should be 605 or less defined in the verifier.
        emailAuthMsg.proof.maskedCommand = string(new bytes(606));

        vm.expectRevert(InvalidMaskedCommandLength.selector);
        emailSigner.authEmail(emailAuthMsg);
    }

    function testExpectRevertAuthEmailInvalidSizeOfTheSkippedCommandPrefix()
        public
    {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg();

        // Set skipped command prefix length to 605, it should be less than 605.
        emailAuthMsg.skippedCommandPrefix = 605;

        vm.expectRevert(InvalidSkippedCommandPrefixSize.selector);
        emailSigner.authEmail(emailAuthMsg);
    }

    function testUpgradeEmailAuth() public {
        vm.startPrank(deployer);

        // Deploy new implementation
        EmailSigner newImplementation = new EmailSigner();

        // Execute upgrade using proxy
        // Upgrade implementation through proxy contract
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(emailSigner),
            abi.encodeCall(
                emailSigner.initialize,
                (
                    deployer,
                    accountSalt,
                    address(dkim),
                    address(verifier),
                    templateId
                )
            )
        );
        EmailSigner emailSignerProxy = EmailSigner(payable(proxy));
        bytes32 beforeAccountSalt = emailSignerProxy.accountSalt();

        // Upgrade to new implementation through proxy
        emailSignerProxy.upgradeToAndCall(
            address(newImplementation),
            new bytes(0)
        );

        bytes32 afterAccountSalt = emailSignerProxy.accountSalt();

        // Verify the upgrade
        assertEq(beforeAccountSalt, afterAccountSalt);

        vm.stopPrank();
    }

    function testExpectRevertAuthEmailInvalidTemplateId() public {
        EmailAuthMsg memory emailAuthMsg = buildEmailAuthMsg();
        emailAuthMsg.templateId = uint256(1234); // Different template ID than the one set in initialization

        vm.expectRevert(InvalidTemplateId.selector);
        emailSigner.authEmail(emailAuthMsg);
    }
}
