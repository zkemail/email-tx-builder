// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./DeploymentHelper.sol";
import {EmailSigner} from "../../src/EmailSigner.sol";

contract SignerDeploymentHelper is DeploymentHelper {
    EmailSigner emailSigner;

    function setUp() public virtual override {
        super.setUp();

        vm.startPrank(deployer);

        // Create EmailSigner implementation
        EmailSigner emailSignerImpl = new EmailSigner();
        emailSigner = emailSignerImpl;

        // Override the command template for signer-specific needs
        templateId = uint256(keccak256(abi.encodePacked("TEST", uint256(0))));
        commandTemplate = ["signHash", "{uint}"];

        vm.stopPrank();
    }
}
