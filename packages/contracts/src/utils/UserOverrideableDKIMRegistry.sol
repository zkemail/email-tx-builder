// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@zk-email/contracts/interfaces/IDKIMRegistry.sol";

/**
  A Registry that store the hash(dkim_public_key) for each domain
  The hash is calculated by taking Poseidon of DKIM key split into 9 chunks of 242 bits each

  https://zkrepl.dev/?gist=43ce7dce2466c63812f6efec5b13aa73 can be used to generate the public key hash. 
  The same code is used in EmailVerifier.sol
  Input is DKIM pub key split into 17 chunks of 121 bits. You can use `helpers` package to fetch/split DKIM keys
 */
contract UserOverrideableDKIMRegistry is IDKIMRegistry, Ownable {
    constructor(address _owner) Ownable(_owner) {}

    event DKIMPublicKeyHashRegistered(
        string domainName,
        bytes32 publicKeyHash,
        address register
    );
    event DKIMPublicKeyHashRevoked(bytes32 publicKeyHash, address register);

    // Mapping from domain name to DKIM public key hash
    mapping(string => mapping(bytes32 => mapping(address => bool)))
        public dkimPublicKeyHashes;

    // DKIM public that are revoked (eg: in case of private key compromise)
    mapping(bytes32 => mapping(address => bool))
        public revokedDKIMPublicKeyHashes;

    function _stringEq(
        string memory a,
        string memory b
    ) internal pure returns (bool) {
        return keccak256(abi.encodePacked(a)) == keccak256(abi.encodePacked(b));
    }

    function isDKIMPublicKeyHashValid(
        string memory domainName,
        bytes32 publicKeyHash
    ) public view returns (bool) {
        if (
            revokedDKIMPublicKeyHashes[publicKeyHash][address(0)] ||
            revokedDKIMPublicKeyHashes[publicKeyHash][msg.sender]
        ) {
            return false;
        }

        if (dkimPublicKeyHashes[domainName][publicKeyHash][address(0)]) {
            return true;
        }

        if (dkimPublicKeyHashes[domainName][publicKeyHash][msg.sender]) {
            return true;
        }

        return false;
    }

    function setDKIMPublicKeyHash(
        string memory domainName,
        bytes32 publicKeyHash,
        bool individual
    ) public {
        address register = msg.sender;
        if (!individual) {
            require(
                msg.sender == owner(),
                "only owner can set DKIM public key hash for all users"
            );
            register = address(0);
        }
        require(
            !revokedDKIMPublicKeyHashes[publicKeyHash][register],
            "cannot set revoked pubkey"
        );

        dkimPublicKeyHashes[domainName][publicKeyHash][register] = true;

        emit DKIMPublicKeyHashRegistered(domainName, publicKeyHash, register);
    }

    function setDKIMPublicKeyHashes(
        string memory domainName,
        bytes32[] memory publicKeyHashes,
        bool individual
    ) public {
        for (uint256 i = 0; i < publicKeyHashes.length; i++) {
            setDKIMPublicKeyHash(domainName, publicKeyHashes[i], individual);
        }
    }

    function revokeDKIMPublicKeyHash(
        bytes32 publicKeyHash,
        bool individual
    ) public {
        address register = msg.sender;
        if (!individual) {
            require(
                msg.sender == owner(),
                "only owner can revoke DKIM public key hash for all users"
            );
            register = address(0);
        }
        revokedDKIMPublicKeyHashes[publicKeyHash][register] = true;

        emit DKIMPublicKeyHashRevoked(publicKeyHash, register);
    }
}