// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {EmailProof} from "../utils/Verifier.sol";

/// @notice Custom errors for email authentication
interface IEmailAuthErrors {
    /// @notice Thrown when the DKIM registry address is zero
    error InvalidDKIMRegistryAddress();
    /// @notice Thrown when attempting to initialize DKIM registry that's already set
    error DKIMRegistryAlreadyInitialized();
    /// @notice Thrown when the verifier address is zero
    error InvalidVerifierAddress();
    /// @notice Thrown when attempting to initialize verifier that's already set
    error VerifierAlreadyInitialized();
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
}

/// @notice Struct to hold the email authentication/authorization message.
struct EmailAuthMsg {
    /// @notice The ID of the command template that the command in the email body should satisfy.
    uint templateId;
    /// @notice The parameters in the command of the email body, which should be taken according to the specified command template.
    bytes[] commandParams;
    /// @notice The number of skipped bytes in the command.
    uint skippedCommandPrefix;
    /// @notice The email proof containing the zk proof and other necessary information for the email verification by the verifier contract.
    EmailProof proof;
}

/// @title Interface for Email Authentication/Authorization Contract
interface IEmailAuth is IEmailAuthErrors {
    event DKIMRegistryUpdated(address indexed dkimRegistry);
    event VerifierUpdated(address indexed verifier);
    event EmailAuthed(
        bytes32 indexed emailNullifier,
        bytes32 indexed accountSalt,
        bool isCodeExist,
        uint templateId
    );

    function accountSalt() external view returns (bytes32);
    function dkimRegistryAddr() external view returns (address);
    function verifierAddr() external view returns (address);

    function updateDKIMRegistry(address _dkimRegistryAddr) external;
    function updateVerifier(address _verifierAddr) external;

    function authEmail(EmailAuthMsg memory emailAuthMsg) external;
}
