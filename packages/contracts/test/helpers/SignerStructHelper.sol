// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./SignerDeploymentHelper.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";

contract SignerStructHelper is SignerDeploymentHelper {
    function buildEmailAuthMsg(
        bytes32 hash
    ) public returns (EmailAuthMsg memory emailAuthMsg) {
        bytes[] memory commandParams = new bytes[](1);
        commandParams[0] = abi.encode(hash);

        EmailProof memory emailProof = EmailProof({
            domainName: "gmail.com",
            publicKeyHash: publicKeyHash,
            timestamp: 1694989812,
            maskedCommand: string.concat(
                "signHash ",
                Strings.toString(uint256(hash))
            ),
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
