// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/**
 * @notice This script is for development and testing purposes only
 * @dev Deploys mock versions of contracts with simplified validation
 * @dev DO NOT USE IN PRODUCTION
 */

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {BaseDeployScript} from "./BaseDeployScript.sol";
import {EmailSigner} from "../src/EmailSigner.sol";
import {EmailSignerFactory} from "../src/EmailSignerFactory.sol";
import {Groth16Verifier} from "../src/utils/Groth16Verifier110.sol";
import {IDKIMRegistry} from "@zk-email/contracts/DKIMRegistry.sol";
import {Verifier} from "../src/utils/Verifier.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @title Mock DKIM Registry
/// @notice A mock implementation of IDKIMRegistry that always returns true for validity checks
/// @dev Used for testing purposes only
contract MockDKIMRegistry is IDKIMRegistry {
    /// @notice Always returns true for any domain name and public key hash combinations
    /// @return bool Always returns true
    function isDKIMPublicKeyHashValid(
        string calldata,
        bytes32
    ) external pure returns (bool) {
        return true;
    }
}

contract DeployEmailSignerFactory is BaseDeployScript {
    uint256 constant TEMPLATE_ID = 1; // Template ID for sign hash command
    uint256 constant DKIM_REGISTRY_TIME_DELAY = 7 days; // Time delay for DKIM registry updates

    function run() public override {
        // Call parent run to set up deployment configuration
        super.run();

        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);

        // Deploy DKIM Registry with time delay
        address dkimRegistry = address(new MockDKIMRegistry());

        address verifierProxyAddress;
        Verifier verifierImpl = new Verifier();
        Groth16Verifier groth16Verifier = new Groth16Verifier();
        console.log(
            "Groth16Verifier deployed at: %s",
            address(groth16Verifier)
        );
        verifierProxyAddress = address(
            new ERC1967Proxy(
                address(verifierImpl),
                abi.encodeCall(
                    verifierImpl.initialize,
                    (initialOwner, address(groth16Verifier))
                )
            )
        );
        console.log("Verifier deployed at: %s", verifierProxyAddress);

        // Deploy EmailSigner implementation
        EmailSigner emailSignerImpl = new EmailSigner();
        console.log(
            "EmailSigner implementation deployed at: %s",
            address(emailSignerImpl)
        );

        // Deploy EmailSignerFactory
        EmailSignerFactory factory = new EmailSignerFactory(
            address(emailSignerImpl),
            dkimRegistry,
            verifierProxyAddress
        );
        console.log("EmailSignerFactory deployed at: %s", address(factory));

        vm.stopBroadcast();

        // Log deployment summary
        console.log("\nDeployment Summary:");
        console.log("-------------------");
        console.log("DKIM Registry: %s", dkimRegistry);
        console.log("Verifier: %s", verifierProxyAddress);
        console.log("EmailSigner Implementation: %s", address(emailSignerImpl));
        console.log("EmailSignerFactory: %s", address(factory));
    }
}
