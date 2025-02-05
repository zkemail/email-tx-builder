// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./DeploymentHelper.sol";
import {EmailSigner} from "../../src/EmailSigner.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract SignerDeploymentHelper is DeploymentHelper {
    EmailSigner emailSigner;

    function setUp() public virtual override {
        super.setUp();

        vm.startPrank(deployer);

        // Create EmailSigner implementation
        EmailSigner emailSignerImpl = new EmailSigner();

        // Deploy proxy pointing to implementation
        bytes memory initData = abi.encodeWithSelector(
            EmailSigner.initialize.selector,
            accountSalt,
            address(dkim),
            address(verifier),
            templateId
        );

        ERC1967Proxy proxy = new ERC1967Proxy(
            address(emailSignerImpl),
            initData
        );

        emailSigner = EmailSigner(address(proxy));

        // Override the command template for signer-specific needs
        templateId = uint256(keccak256(abi.encodePacked("TEST", uint256(0))));
        commandTemplate = ["signHash", "{uint}"];

        vm.stopPrank();
    }
}
