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
import {SafeSigner} from "../src/SafeSigner.sol";
import {SafeSignerFactory} from "../src/SafeSignerFactory.sol";
import {Groth16Verifier} from "../src/utils/Groth16Verifier110.sol";
import {IDKIMRegistry} from "@zk-email/contracts/DKIMRegistry.sol";
import {Verifier} from "../src/utils/Verifier.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeploySafeSignerFactory is BaseDeployScript {
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
            address(0),
            DKIM_REGISTRY_TIME_DELAY
        );

        address verifier = deployVerifier(initialOwner);

        // Deploy EmailSigner implementation
        SafeSigner safeSignerImpl = new SafeSigner();
        console.log(
            "SafeSigner implementation deployed at: %s",
            address(safeSignerImpl)
        );

        // Deploy EmailSignerFactory
        SafeSignerFactory factory = new SafeSignerFactory(
            address(safeSignerImpl),
            dkimRegistry,
            verifier
        );
        console.log("SafeSignerFactory deployed at: %s", address(factory));

        vm.stopBroadcast();

        // Log deployment summary
        console.log("\nDeployment Summary:");
        console.log("-------------------");
        console.log("DKIM Registry: %s", dkimRegistry);
        console.log("Verifier: %s", verifier);
        console.log("SafeSigner Implementation: %s", address(safeSignerImpl));
        console.log("SafeSignerFactory: %s", address(factory));
    }
}
