// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./SignerDeploymentHelper.sol";

contract SignerStructHelper is SignerDeploymentHelper {
    function buildEmailAuthMsg()
        public
        returns (EmailAuthMsg memory emailAuthMsg)
    {
        bytes[] memory commandParams = new bytes[](1);
        commandParams[0] = abi.encode(1234567890);

        EmailProof memory emailProof = EmailProof({
            domainName: "gmail.com",
            publicKeyHash: publicKeyHash,
            timestamp: 1694989812,
            maskedCommand: "signHash 1234567890",
            emailNullifier: emailNullifier,
            accountSalt: accountSalt,
            isCodeExist: true,
            proof: mockProof
        });

        emailAuthMsg = EmailAuthMsg({
            templateId: templateId,
            commandParams: commandParams,
            skippedCommandPrefix: 0,
            proof: emailProof
        });

        vm.mockCall(
            address(verifier),
            abi.encodeCall(Verifier.verifyEmailProof, (emailProof)),
            abi.encode(true)
        );
    }
}
