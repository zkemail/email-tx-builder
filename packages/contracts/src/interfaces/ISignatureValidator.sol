// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ISignatureValidator {
    // Magic value returned by older versions of EIP1271 when validating data and signature
    // bytes4(keccak256("isValidSignature(bytes,bytes)")). Used by Gnosis Safe and others.
    bytes4 internal constant EIP1271_MAGIC_VALUE_DATA = 0x20c13b0b;

    // Magic value returned by newer versions of EIP1271 when validating hash and signature
    // bytes4(keccak256("isValidSignature(bytes32,bytes)"))
    bytes4 internal constant EIP1271_MAGIC_VALUE_HASH = 0x1626ba7e;

    /**
     * @dev Validates if a signature is valid for the provided data. Used in older EIP1271 versions.
     * @param _data Raw data that was signed
     * @param _signature Signature bytes to validate against the data
     * @return Magic value 0x20c13b0b if signature is valid for the data, 0x0 otherwise
     *
     * MUST NOT modify state (using STATICCALL for solc < 0.5, view modifier for solc > 0.5)
     * MUST allow external calls
     */
    function isValidSignature(
        bytes calldata _data,
        bytes calldata _signature
    ) public view returns (bytes4);

    /**
     * @dev Validates if a signature is valid for the provided hash. Used in newer EIP1271 versions.
     * @param _hash Hash of the data that was signed
     * @param _signature Signature bytes to validate against the hash
     * @return magicValue Magic value 0x1626ba7e if signature is valid for the hash, 0x0 otherwise
     *
     * MUST NOT modify state (using STATICCALL for solc < 0.5, view modifier for solc > 0.5)
     * MUST allow external calls
     */
    function isValidSignature(
        bytes32 _hash,
        bytes memory _signature
    ) public view returns (bytes4 magicValue);
}
