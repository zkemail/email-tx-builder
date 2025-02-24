// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {EmailSigner} from "./EmailSigner.sol";
import {ISafe} from "./interfaces/ISafe.sol";

/// @title SafeSigner
/// @notice A contract that allows a user to approve a hash using a signature
/// @dev This contract is a wrapper around the EmailSigner contract
/// and allows a user to approve a hash using a signature
/// @dev This contract is a wrapper around the EmailSigner contract
/// and allows a user to approve a hash using a signature
/// @dev This contract is a wrapper around the EmailSigner contract
/// and allows a user to approve a hash using a signature
contract SafeSigner is EmailSigner {
    event HashApproved(bytes32 hashToApprove, address safe);

    function approveHash(
        bytes32 hashToApprove,
        bytes calldata signature,
        address safe
    ) external {
        if (safe == address(0)) revert("Invalid safe");
        if (safe == address(this)) revert("Invalid safe");

        if (isValidSignature(hashToApprove, signature) == MAGIC_VALUE) {
            ISafe(safe).approveHash(hashToApprove);
            emit HashApproved(hashToApprove, safe);
        }
    }
}
