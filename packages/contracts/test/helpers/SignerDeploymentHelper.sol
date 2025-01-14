// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./DeploymentHelper.sol";
import {EmailAuthSigner} from "../../src/EmailAuthSigner.sol";

contract SignerDeploymentHelper is DeploymentHelper {
    EmailAuthSigner emailAuthSigner;

    function setUp() public virtual override {
        super.setUp();

        vm.startPrank(deployer);

        // Create EmailAuthSigner implementation
        EmailAuthSigner emailAuthSignerImpl = new EmailAuthSigner();
        emailAuthSigner = emailAuthSignerImpl;

        // Override the command template for signer-specific needs
        templateId = uint256(keccak256(abi.encodePacked("TEST", uint256(0))));
        commandTemplate = ["signHash", "{uint}"];

        vm.stopPrank();
    }
}
