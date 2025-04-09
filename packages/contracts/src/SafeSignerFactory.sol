// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {SafeSigner} from "./SafeSigner.sol";
import {EmailSignerFactory} from "./EmailSignerFactory.sol";

/// @title SafeSignerFactory
/// @notice Factory contract for deploying minimal proxy clones of SafeSigner
contract SafeSignerFactory is EmailSignerFactory {
    /// @notice Constructor that sets the implementation contract and initialization parameters
    /// @param _implementation Address of the SafeSigner implementation contract
    /// @param _dkimRegistry Address of the DKIM registry contract
    /// @param _verifier Address of the verifier contract
    constructor(
        address _implementation,
        address _dkimRegistry,
        address _verifier
    ) EmailSignerFactory(_implementation, _dkimRegistry, _verifier) {}
}
