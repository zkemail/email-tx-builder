// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IERC1271 {
    /**
     * @dev Validates if a signature is valid for the provided data. Used in older EIP1271 versions.
     * @param _data Raw data that was signed
     * @param _signature Signature of the data
     * @return Magic value 0x20c13b0b if signature is valid for the data, 0x0 otherwise
     *
     * MUST NOT modify state (using STATICCALL for solc < 0.5, view modifier for solc > 0.5)
     * MUST allow external calls
     */
    function isValidSignature(
        bytes calldata _data,
        bytes calldata _signature
    ) external view returns (bytes4);

    /**
     * @dev Validates if a signature is valid for the provided hash. Used in newer EIP1271 versions.
     * @param _hash Hash of the data that was signed
     * @param _signature Signature of the hash
     * @return magicValue Magic value 0x1626ba7e if signature is valid for the hash, 0x0 otherwise
     *
     * MUST NOT modify state (using STATICCALL for solc < 0.5, view modifier for solc > 0.5)
     * MUST allow external calls
     */
    function isValidSignature(
        bytes32 _hash,
        bytes calldata _signature
    ) external view returns (bytes4);
}
