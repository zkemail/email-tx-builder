// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {EmailSigner} from "./EmailSigner.sol";
import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";

/// @title Email Signer Factory
/// @notice Factory contract for deploying minimal proxy clones of EmailSigner
/// @dev Uses EIP-1167 minimal proxy pattern for gas efficient deployment
contract EmailSignerFactory {
    /// The address of the EmailSigner implementation contract
    address public immutable implementation;
    /// The DKIM registry contract address used by all clones
    address public immutable dkimRegistry;
    /// The verifier contract address used by all clones
    address public immutable verifier;
    /// The template ID used by all clones
    uint256 public immutable templateId;

    event EmailSignerDeployed(
        address indexed emailSigner,
        bytes32 indexed accountSalt
    );

    /// @notice Constructor that sets the implementation contract and initialization parameters
    /// @param _implementation Address of the EmailSigner implementation contract
    /// @param _dkimRegistry Address of the DKIM registry contract
    /// @param _verifier Address of the verifier contract
    /// @param _templateId Template ID for the sign hash command
    constructor(
        address _implementation,
        address _dkimRegistry,
        address _verifier,
        uint256 _templateId
    ) {
        if (_implementation == address(0)) revert("Invalid implementation");
        if (_dkimRegistry == address(0)) revert("Invalid DKIM registry");
        if (_verifier == address(0)) revert("Invalid verifier");

        implementation = _implementation;
        dkimRegistry = _dkimRegistry;
        verifier = _verifier;
        templateId = _templateId;
    }

    /// @notice Deploys a new EmailSigner clone
    /// @param accountSalt The account salt used to generate the clone's salt
    /// @return The address of the deployed EmailSigner clone
    function deploy(bytes32 accountSalt) external returns (address) {
        // @dev This salt is not strictly necessary to deploy the account with this given accountSalt,
        // but it is nice to have for deterministic deployments. The accountSalt itself is ultimately
        // derived from a hash of a secret random code.
        bytes32 salt = keccak256(abi.encodePacked(accountSalt));
        address clone = Clones.cloneDeterministic(implementation, accountSalt);

        EmailSigner(clone).initialize(
            accountSalt, // secret identifier for the account owner
            dkimRegistry,
            verifier,
            templateId
        );

        emit EmailSignerDeployed(clone, salt);
        return clone;
    }

    /// @notice Predicts the address where an EmailSigner clone would be deployed
    /// @param accountSalt The account salt used to generate the clone's salt
    /// @return The predicted address of the EmailSigner clone
    function predictAddress(
        bytes32 accountSalt
    ) external view returns (address) {
        return Clones.predictDeterministicAddress(implementation, accountSalt);
    }
}
