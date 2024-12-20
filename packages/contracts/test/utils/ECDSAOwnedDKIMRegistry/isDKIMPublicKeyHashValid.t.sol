// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "../../../src/utils/ECDSAOwnedDKIMRegistry.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract ECDSAOwnedDKIMRegistryTest_isDKIMPublicKeyHashValid is Test {
    ECDSAOwnedDKIMRegistry dkim;

    string public selector = "12345";
    string public domainName = "example.com";
    bytes32 public publicKeyHash = bytes32(uint256(1));

    function setUp() public {
        address signer = vm.addr(1);
        ECDSAOwnedDKIMRegistry dkimImpl = new ECDSAOwnedDKIMRegistry();
        ERC1967Proxy dkimProxy = new ERC1967Proxy(
            address(dkimImpl),
            abi.encodeCall(dkimImpl.initialize, (msg.sender, signer))
        );
        dkim = ECDSAOwnedDKIMRegistry(address(dkimProxy));
    }

    function test_IsDKIMPublicKeyHashValid_True() public {
        // Set a valid public key hash
        string memory signedMsg = dkim.computeSignedMsg(
            dkim.SET_PREFIX(),
            domainName,
            publicKeyHash
        );
        bytes32 digest = MessageHashUtils.toEthSignedMessageHash(
            bytes(signedMsg)
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        dkim.setDKIMPublicKeyHash(
            selector,
            domainName,
            publicKeyHash,
            signature
        );

        // Check if the public key hash is valid
        bool isValid = dkim.isDKIMPublicKeyHashValid(domainName, publicKeyHash);
        require(isValid, "Public key hash should be valid");
    }

    function test_IsDKIMPublicKeyHashValid_False() public {
        // Check if a non-set public key hash is invalid
        bytes32 nonExistentPublicKeyHash = bytes32(uint256(2));
        bool isValid = dkim.isDKIMPublicKeyHashValid(
            domainName,
            nonExistentPublicKeyHash
        );
        require(!isValid, "Public key hash should not be valid");
    }

    function test_IsDKIMPublicKeyHashValid_AfterRevoke() public {
        // Set and then revoke a public key hash
        string memory signedMsg = dkim.computeSignedMsg(
            dkim.SET_PREFIX(),
            domainName,
            publicKeyHash
        );
        bytes32 digest = MessageHashUtils.toEthSignedMessageHash(
            bytes(signedMsg)
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        dkim.setDKIMPublicKeyHash(
            selector,
            domainName,
            publicKeyHash,
            signature
        );

        // Revoke the public key hash
        string memory revokeMsg = dkim.computeSignedMsg(
            dkim.REVOKE_PREFIX(),
            domainName,
            publicKeyHash
        );
        bytes32 revokeDigest = MessageHashUtils.toEthSignedMessageHash(
            bytes(revokeMsg)
        );
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(1, revokeDigest);
        bytes memory revokeSig = abi.encodePacked(r1, s1, v1);
        dkim.revokeDKIMPublicKeyHash(
            selector,
            domainName,
            publicKeyHash,
            revokeSig
        );

        // Check if the public key hash is invalid after revocation
        bool isValid = dkim.isDKIMPublicKeyHashValid(domainName, publicKeyHash);
        require(
            !isValid,
            "Public key hash should not be valid after revocation"
        );
    }
}
