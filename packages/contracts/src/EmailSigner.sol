// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {EmailProof} from "./utils/Verifier.sol";
import {IDKIMRegistry} from "@zk-email/contracts/DKIMRegistry.sol";
import {IVerifier, EmailProof} from "./interfaces/IVerifier.sol";
import {CommandUtils} from "./libraries/CommandUtils.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {EmailAuthMsg} from "./interfaces/IEmailTypes.sol";
import {IERC1271} from "./interfaces/IERC1271.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

/// @title Email Authentication/Authorization Contract for Signature-like Usage
/// @notice This contract provides a signature-like authentication mechanism using emails.
/// Similar to how ECDSA signatures work, this contract only verifies the authenticity
/// of an email command without handling replay protection or nullifiers - these should
/// be implemented at the application level.
/// @dev Unlike EmailAuth.sol which handles nullifiers internally, this contract is designed
/// to be used like a signature verification mechanism where the calling contract manages
/// its own replay protection.
contract EmailSigner is IERC1271, Initializable {
    /// @notice ERC-1271 magic value returned on valid signatures.
    /// @dev Value is derived from `bytes4(keccak256("isValidSignature(bytes32,bytes)")`.
    bytes4 constant MAGIC_VALUE = 0x1626ba7e;

    /// @notice Legacy EIP-1271 magic value returned on valid signatures.
    /// @dev Value is derived from `bytes4(keccak256("isValidSignature(bytes,bytes)")`.
    bytes4 constant LEGACY_MAGIC_VALUE = 0x20c13b0b;

    /// Unique identifier for owner of this contract defined as a hash of an email address and an account code.
    bytes32 public accountSalt;
    /// An instance of the DKIM registry contract.
    address public dkimRegistryAddr;
    /// An instance of the Verifier contract.
    address public verifierAddr;
    /// The templateId of the sign hash command.
    uint256 public templateId;

    /// @notice Thrown when the DKIM registry address is zero
    error InvalidDKIMRegistryAddress();
    /// @notice Thrown when the verifier address is zero
    error InvalidVerifierAddress();
    /// @notice Thrown when the template ID doesn't match
    error InvalidTemplateId();
    /// @notice Thrown when the DKIM public key hash verification fails
    error InvalidDKIMPublicKeyHash();
    /// @notice Thrown when the account salt doesn't match
    error InvalidAccountSalt();
    /// @notice Thrown when the masked command length exceeds the maximum
    error InvalidMaskedCommandLength();
    /// @notice Thrown when the skipped command prefix size is invalid
    error InvalidSkippedCommandPrefixSize();
    /// @notice Thrown when the command format is invalid
    error InvalidCommand();
    /// @notice Thrown when the email proof verification fails
    error InvalidEmailProof();

    constructor() {
        _disableInitializers();
    }

    /// @notice Initialize the contract with an initial owner, account salt, DKIM registry address, and verifier address.
    /// @param _accountSalt The account salt to derive CREATE2 address of this contract.
    /// @param _dkimRegistryAddr The address of the DKIM registry contract.
    /// @param _verifierAddr The address of the verifier contract.
    /// @param _templateId The templateId of the sign hash command.
    function initialize(
        bytes32 _accountSalt,
        address _dkimRegistryAddr,
        address _verifierAddr,
        uint256 _templateId
    ) public initializer {
        if (_accountSalt == bytes32(0)) revert InvalidAccountSalt();
        if (_templateId == 0) revert InvalidTemplateId();
        if (_dkimRegistryAddr == address(0))
            revert InvalidDKIMRegistryAddress();
        if (_verifierAddr == address(0)) revert InvalidVerifierAddress();

        accountSalt = _accountSalt;
        templateId = _templateId;
        dkimRegistryAddr = _dkimRegistryAddr;
        verifierAddr = _verifierAddr;
    }

    /// @notice Authenticate the email sender and authorize the message in the email command.
    /// @dev This function only verifies the authenticity of the email and command, without
    /// handling replay protection. The calling contract should implement its own mechanisms
    /// to prevent replay attacks, similar to how nonces are used with ECDSA signatures.
    /// @param emailAuthMsg The email auth message containing all necessary information for authentication.
    function verifyEmail(EmailAuthMsg memory emailAuthMsg) public view {
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
        string memory trimmedMaskedCommand = CommandUtils.removePrefix(
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

        verifyEmail(emailAuthMsg); // reverts if invalid
        bytes32 signedHash = abi.decode(
            emailAuthMsg.commandParams[0],
            (bytes32)
        );

        // signature is valid
        if (signedHash == _hash) return MAGIC_VALUE;

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
        if (isValidSignature(keccak256(_data), _signature) == MAGIC_VALUE)
            return LEGACY_MAGIC_VALUE;
        return bytes4(0);
    }
}
