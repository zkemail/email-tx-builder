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

    event EmailSignerDeployed(
        address indexed emailSigner,
        bytes32 indexed accountSalt
    );

    error InvalidImplementation();
    error InvalidDKIMRegistry();
    error InvalidVerifier();

    /// @notice Constructor that sets the implementation contract and initialization parameters
    /// @param _implementation Address of the EmailSigner implementation contract
    /// @param _dkimRegistry Address of the DKIM registry contract
    /// @param _verifier Address of the verifier contract
    constructor(
        address _implementation,
        address _dkimRegistry,
        address _verifier
    ) {
        if (_implementation == address(0)) revert InvalidImplementation();
        if (_dkimRegistry == address(0)) revert InvalidDKIMRegistry();
        if (_verifier == address(0)) revert InvalidVerifier();

        implementation = _implementation;
        dkimRegistry = _dkimRegistry;
        verifier = _verifier;
    }

    /// @notice Deploys a new EmailSigner clone
    /// @param accountSalt The account salt used to generate the clone's salt
    /// @return The address of the deployed EmailSigner clone
    function deploy(bytes32 accountSalt) external returns (address) {
        // salt is not strictly necessary to deploy the account with this given accountSalt,
        // but it is nice to have for deterministic deployments. The accountSalt itself is ultimately
        // derived from a hash of a secret random code and an email address.
        bytes32 salt = keccak256(abi.encodePacked(accountSalt));
        address clone = Clones.cloneDeterministic(implementation, accountSalt);

        // Generate templateId dynamically based on accountSalt
        uint256 templateId = uint256(
            keccak256(abi.encodePacked("EMAIL-SIGNER", accountSalt))
        );

        EmailSigner(clone).initialize(
            accountSalt,
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
