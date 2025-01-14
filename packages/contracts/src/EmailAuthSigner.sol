// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {EmailProof} from "./utils/Verifier.sol";
import {IDKIMRegistry} from "@zk-email/contracts/DKIMRegistry.sol";
import {IVerifier, EmailProof} from "./interfaces/IVerifier.sol";
import {CommandUtils} from "./libraries/CommandUtils.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {IEmailAuth, EmailAuthMsg} from "./interfaces/IEmailAuth.sol";

/// @title Email Authentication/Authorization Contract for Signature-like Usage
/// @notice This contract provides a signature-like authentication mechanism using emails.
/// Similar to how ECDSA signatures work, this contract only verifies the authenticity
/// of an email command without handling replay protection or nullifiers - these should
/// be implemented at the application level.
/// @dev Unlike EmailAuth.sol which handles nullifiers internally, this contract is designed
/// to be used like a signature verification mechanism where the calling contract manages
/// its own replay protection.
contract EmailAuthSigner is OwnableUpgradeable, UUPSUpgradeable, IEmailAuth {
    /// The CREATE2 salt of this contract defined as a hash of an email address and an account code.
    bytes32 public accountSalt;
    /// An instance of the DKIM registry contract.
    address public dkimRegistryAddr;
    /// An instance of the Verifier contract.
    address public verifierAddr;
    /// The templateId of the sign hash command.
    uint256 public templateId;

    constructor() {}

    /// @notice Initialize the contract with an initial owner, account salt, DKIM registry address, and verifier address.
    /// @param _initialOwner The address of the initial owner.
    /// @param _accountSalt The account salt to derive CREATE2 address of this contract.
    /// @param _dkimRegistryAddr The address of the DKIM registry contract.
    /// @param _verifierAddr The address of the verifier contract.
    /// @param _templateId The templateId of the sign hash command.
    function initialize(
        address _initialOwner,
        bytes32 _accountSalt,
        address _dkimRegistryAddr,
        address _verifierAddr,
        uint256 _templateId
    ) public initializer {
        __Ownable_init(_initialOwner);
        accountSalt = _accountSalt;
        templateId = _templateId;
        require(
            _dkimRegistryAddr != address(0),
            "invalid dkim registry address"
        );
        require(
            address(dkimRegistryAddr) == address(0),
            "dkim registry already initialized"
        );
        dkimRegistryAddr = _dkimRegistryAddr;
        emit DKIMRegistryUpdated(_dkimRegistryAddr);

        require(_verifierAddr != address(0), "invalid verifier address");
        require(
            address(verifierAddr) == address(0),
            "verifier already initialized"
        );
        verifierAddr = _verifierAddr;
        emit VerifierUpdated(_verifierAddr);
    }

    /// @notice Updates the address of the DKIM registry contract.
    /// @param _dkimRegistryAddr The new address of the DKIM registry contract.
    function updateDKIMRegistry(address _dkimRegistryAddr) public onlyOwner {
        require(
            _dkimRegistryAddr != address(0),
            "invalid dkim registry address"
        );
        dkimRegistryAddr = _dkimRegistryAddr;
        emit DKIMRegistryUpdated(_dkimRegistryAddr);
    }

    /// @notice Updates the address of the verifier contract.
    /// @param _verifierAddr The new address of the verifier contract.
    function updateVerifier(address _verifierAddr) public onlyOwner {
        require(_verifierAddr != address(0), "invalid verifier address");
        verifierAddr = _verifierAddr;
        emit VerifierUpdated(_verifierAddr);
    }

    /// @notice Authenticate the email sender and authorize the message in the email command.
    /// @dev This function only verifies the authenticity of the email and command, without
    /// handling replay protection. The calling contract should implement its own mechanisms
    /// to prevent replay attacks, similar to how nonces are used with ECDSA signatures.
    /// @param emailAuthMsg The email auth message containing all necessary information for authentication.
    function authEmail(EmailAuthMsg memory emailAuthMsg) public {
        require(templateId == emailAuthMsg.templateId, "invalid template id");
        string[] memory signHashTemplate = new string[](2);
        signHashTemplate[0] = "signHash";
        signHashTemplate[1] = "{uint}";
        require(
            IDKIMRegistry(dkimRegistryAddr).isDKIMPublicKeyHashValid(
                emailAuthMsg.proof.domainName,
                emailAuthMsg.proof.publicKeyHash
            ) == true,
            "invalid dkim public key hash"
        );
        require(
            accountSalt == emailAuthMsg.proof.accountSalt,
            "invalid account salt"
        );
        require(
            bytes(emailAuthMsg.proof.maskedCommand).length <=
                IVerifier(verifierAddr).commandBytes(),
            "invalid masked command length"
        );
        require(
            emailAuthMsg.skippedCommandPrefix <
                IVerifier(verifierAddr).commandBytes(),
            "invalid size of the skipped command prefix"
        );

        // Construct an expectedCommand from template and the values of emailAuthMsg.commandParams.
        string memory trimmedMaskedCommand = removePrefix(
            emailAuthMsg.proof.maskedCommand,
            emailAuthMsg.skippedCommandPrefix
        );
        string memory expectedCommand = "";
        for (uint stringCase = 0; stringCase < 3; stringCase++) {
            expectedCommand = CommandUtils.computeExpectedCommand(
                emailAuthMsg.commandParams,
                signHashTemplate,
                stringCase
            );
            if (Strings.equal(expectedCommand, trimmedMaskedCommand)) {
                break;
            }
            if (stringCase == 2) {
                revert("invalid command");
            }
        }

        require(
            IVerifier(verifierAddr).verifyEmailProof(emailAuthMsg.proof) ==
                true,
            "invalid email proof"
        );

        emit EmailAuthed(
            emailAuthMsg.proof.emailNullifier,
            emailAuthMsg.proof.accountSalt,
            emailAuthMsg.proof.isCodeExist,
            emailAuthMsg.templateId
        );
    }

    /// @notice Upgrade the implementation of the proxy.
    /// @param newImplementation Address of the new implementation.
    function _authorizeUpgrade(
        address newImplementation
    ) internal override onlyOwner {}

    /// @notice Remove a prefix from a string.
    /// @param str The original string.
    /// @param numBytes The number of bytes to remove from the start of the string.
    /// @return The string with the prefix removed.
    function removePrefix(
        string memory str,
        uint numBytes
    ) private pure returns (string memory) {
        require(
            numBytes <= bytes(str).length,
            "Invalid size of the removed bytes"
        );

        bytes memory strBytes = bytes(str);
        bytes memory result = new bytes(strBytes.length - numBytes);

        for (uint i = numBytes; i < strBytes.length; i++) {
            result[i - numBytes] = strBytes[i];
        }

        return string(result);
    }
}
