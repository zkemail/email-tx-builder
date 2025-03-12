// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface ISafe {
    function approveHash(bytes32 hashToApprove) external;
}
