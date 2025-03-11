// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {BaseDeployScript} from "./BaseDeployScript.sol";
import {EmailSigner} from "../src/EmailSigner.sol";
import {EmailSignerFactory} from "../src/EmailSignerFactory.sol";

contract DeployEmailSignerFactory is BaseDeployScript {
    uint256 constant TEMPLATE_ID = 1; // Template ID for sign hash command
    uint256 constant DKIM_REGISTRY_TIME_DELAY = 7 days; // Time delay for DKIM registry updates

    function run() public override {
        // Call parent run to set up deployment configuration
        super.run();

        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);

        // Deploy DKIM Registry with time delay
        address dkimRegistry = deployUserOverrideableDKIMRegistry(
            initialOwner,
            vm.envAddress("DKIM_SIGNER"), // DKIM signer from environment variable
            DKIM_REGISTRY_TIME_DELAY
        );

        // Deploy Verifier with Groth16
        address verifier = deployVerifier(initialOwner);

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
            verifier
        );
        console.log("EmailSignerFactory deployed at: %s", address(factory));

        vm.stopBroadcast();

        // Log deployment summary
        console.log("\nDeployment Summary:");
        console.log("-------------------");
        console.log("DKIM Registry: %s", dkimRegistry);
        console.log("Verifier: %s", verifier);
        console.log("EmailSigner Implementation: %s", address(emailSignerImpl));
        console.log("EmailSignerFactory: %s", address(factory));
    }
}
