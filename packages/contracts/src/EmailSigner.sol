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
import {IERC1271} from "./interfaces/IERC1271.sol";
import {ERC1271} from "./libraries/ERC1271.sol";

/// @title Email Authentication/Authorization Contract for Signature-like Usage
/// @notice This contract provides a signature-like authentication mechanism using emails.
/// Similar to how ECDSA signatures work, this contract only verifies the authenticity
/// of an email command without handling replay protection or nullifiers - these should
/// be implemented at the application level.
/// @dev Unlike EmailAuth.sol which handles nullifiers internally, this contract is designed
/// to be used like a signature verification mechanism where the calling contract manages
/// its own replay protection.
contract EmailSigner is OwnableUpgradeable, UUPSUpgradeable, IEmailAuth {
    /// The CREATE2 salt of this contract defined as a hash of an email address and an account code.
    bytes32 public accountSalt;
    /// An instance of the DKIM registry contract.
    address public dkimRegistryAddr;
    /// An instance of the Verifier contract.
    address public verifierAddr;
    /// The templateId of the sign hash command.
    uint256 public templateId;

    constructor() {
        _disableInitializers();
    }

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
        if (_dkimRegistryAddr == address(0))
            revert InvalidDKIMRegistryAddress();
        if (address(dkimRegistryAddr) != address(0))
            revert DKIMRegistryAlreadyInitialized();
        dkimRegistryAddr = _dkimRegistryAddr;
        emit DKIMRegistryUpdated(_dkimRegistryAddr);

        if (_verifierAddr == address(0)) revert InvalidVerifierAddress();
        if (address(verifierAddr) != address(0))
            revert VerifierAlreadyInitialized();
        verifierAddr = _verifierAddr;
        emit VerifierUpdated(_verifierAddr);
    }

    /// @notice Updates the address of the DKIM registry contract.
    /// @param _dkimRegistryAddr The new address of the DKIM registry contract.
    function updateDKIMRegistry(address _dkimRegistryAddr) public onlyOwner {
        if (_dkimRegistryAddr == address(0))
            revert InvalidDKIMRegistryAddress();
        dkimRegistryAddr = _dkimRegistryAddr;
        emit DKIMRegistryUpdated(_dkimRegistryAddr);
    }

    /// @notice Updates the address of the verifier contract.
    /// @param _verifierAddr The new address of the verifier contract.
    function updateVerifier(address _verifierAddr) public onlyOwner {
        if (_verifierAddr == address(0)) revert InvalidVerifierAddress();
        verifierAddr = _verifierAddr;
        emit VerifierUpdated(_verifierAddr);
    }

    /// @notice Authenticate the email sender and authorize the message in the email command.
    /// @dev This function only verifies the authenticity of the email and command, without
    /// handling replay protection. The calling contract should implement its own mechanisms
    /// to prevent replay attacks, similar to how nonces are used with ECDSA signatures.
    /// @param emailAuthMsg The email auth message containing all necessary information for authentication.
    function authEmail(EmailAuthMsg memory emailAuthMsg) public view {
        if (templateId != emailAuthMsg.templateId) revert InvalidTemplateId();
        string[] memory signHashTemplate = new string[](2);
        signHashTemplate[0] = "signHash";
        signHashTemplate[1] = "{uint}";
        if (
            !IDKIMRegistry(dkimRegistryAddr).isDKIMPublicKeyHashValid(
                emailAuthMsg.proof.domainName,
                emailAuthMsg.proof.publicKeyHash
            )
        ) revert InvalidDKIMPublicKeyHash();

        if (accountSalt != emailAuthMsg.proof.accountSalt)
            revert InvalidAccountSalt();

        if (
            bytes(emailAuthMsg.proof.maskedCommand).length >
            IVerifier(verifierAddr).commandBytes()
        ) revert InvalidMaskedCommandLength();

        if (
            emailAuthMsg.skippedCommandPrefix >=
            IVerifier(verifierAddr).commandBytes()
        ) revert InvalidSkippedCommandPrefixSize();

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
                revert InvalidCommand();
            }
        }

        if (!IVerifier(verifierAddr).verifyEmailProof(emailAuthMsg.proof))
            revert InvalidEmailProof();
    }

    /// @notice Validates a signature of an EmailAuthMsg. ERC1271 compatible.
    /// @param _hash The hash of the EmailAuthMsg.
    /// @param _signature The signature of the EmailAuthMsg.
    /// @return bytes4(0) if the signature is invalid, ERC1271.MAGIC_VALUE if the signature is valid.
    function isValidSignature(
        bytes32 _hash,
        bytes calldata _signature
    ) public view returns (bytes4) {
        // signature is a serialized EmailAuthMsg
        EmailAuthMsg memory emailAuthMsg = abi.decode(
            _signature,
            (EmailAuthMsg)
        );

        authEmail(emailAuthMsg); // reverts if invalid
        bytes32 signedHash = abi.decode(
            emailAuthMsg.commandParams[0],
            (bytes32)
        );

        // signature is valid
        if (signedHash == _hash) return ERC1271.MAGIC_VALUE;

        // signature is invalid
        return bytes4(0);
    }

    /// @notice Validates a signature of an EmailAuthMsg. Legacy ERC1271 compatible.
    /// @param _data The data to validate the signature against.
    /// @param _signature The signature to validate.
    /// @return bytes4(0) if the signature is invalid, ERC1271.MAGIC_VALUE_LEGACY if the signature is valid.
    function isValidSignature(
        bytes calldata _data,
        bytes calldata _signature
    ) external view returns (bytes4) {
        if (
            isValidSignature(keccak256(_data), _signature) ==
            ERC1271.MAGIC_VALUE
        ) return ERC1271.LEGACY_MAGIC_VALUE;
        return bytes4(0);
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
        if (numBytes > bytes(str).length)
            revert InvalidSkippedCommandPrefixSize();

        bytes memory strBytes = bytes(str);
        bytes memory result = new bytes(strBytes.length - numBytes);

        for (uint i = numBytes; i < strBytes.length; i++) {
            result[i - numBytes] = strBytes[i];
        }

        return string(result);
    }
}
