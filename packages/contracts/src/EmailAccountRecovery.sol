// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./EmailAuth.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @title Email Account Recovery Contract
/// @notice Provides mechanisms for email-based account recovery, leveraging guardians and template-based email verification.
/// @dev This contract is abstract and requires implementation of several methods for configuring a new guardian and recovering a wallet.
abstract contract EmailAccountRecovery {
    uint8 constant EMAIL_ACCOUNT_RECOVERY_VERSION_ID = 1;
    address public verifierAddr;
    address public dkimAddr;
    address public emailAuthImplementationAddr;

    /// @notice Returns the address of the verifier contract.
    /// @dev This function is virtual and can be overridden by inheriting contracts.
    /// @return address The address of the verifier contract.
    function verifier() public view virtual returns (address) {
        return verifierAddr;
    }

    /// @notice Returns the address of the DKIM contract.
    /// @dev This function is virtual and can be overridden by inheriting contracts.
    /// @return address The address of the DKIM contract.
    function dkim() public view virtual returns (address) {
        return dkimAddr;
    }

    /// @notice Returns the address of the email auth contract implementation.
    /// @dev This function is virtual and can be overridden by inheriting contracts.
    /// @return address The address of the email authentication contract implementation.
    function emailAuthImplementation() public view virtual returns (address) {
        return emailAuthImplementationAddr;
    }

    /// @notice Returns a two-dimensional array of strings representing the subject templates for an acceptance by a new guardian's.
    /// @dev This function is virtual and should be implemented by inheriting contracts to define specific acceptance subject templates.
    /// @return string[][] A two-dimensional array of strings, where each inner array represents a set of fixed strings and matchers for a subject template.
    function acceptanceSubjectTemplates()
        public
        view
        virtual
        returns (string[][] memory);

    /// @notice Returns a two-dimensional array of strings representing the subject templates for email recovery.
    /// @dev This function is virtual and should be implemented by inheriting contracts to define specific recovery subject templates.
    /// @return string[][] A two-dimensional array of strings, where each inner array represents a set of fixed strings and matchers for a subject template.
    function recoverySubjectTemplates()
        public
        view
        virtual
        returns (string[][] memory);

    function acceptGuardian(
        address guardian,
        uint templateIdx,
        bytes[] memory subjectParams,
        bytes32 emailNullifier
    ) internal virtual;

    function processRecovery(
        address guardian,
        uint templateIdx,
        bytes[] memory subjectParams,
        bytes32 emailNullifier
    ) internal virtual;

    /// @notice Completes the recovery process.
    /// @dev This function must be implemented by inheriting contracts to finalize the recovery process.
    function completeRecovery() external virtual;

    /// @notice Computes the address for email auth contract using the CREATE2 opcode.
    /// @dev This function utilizes the `Create2` library to compute the address. The computation uses a provided account salt
    /// and the hash of the encoded ERC1967Proxy creation code concatenated with the encoded email auth contract implementation
    /// address and the initialization call data. This ensures that the computed address is deterministic and unique per account salt.
    /// @param accountSalt A bytes32 salt value, which is assumed to be unique to a pair of the guardian's email address and the wallet address to be recovered.
    /// @return address The computed address.
    function computeEmailAuthAddress(
        bytes32 accountSalt
    ) public view returns (address) {
        return
            Create2.computeAddress(
                accountSalt,
                keccak256(
                    abi.encodePacked(
                        type(ERC1967Proxy).creationCode,
                        abi.encode(
                            emailAuthImplementation(),
                            abi.encodeCall(
                                EmailAuth.initialize,
                                (address(this), accountSalt)
                            )
                        )
                    )
                )
            );
    }

    /// @notice Calculates a unique subject template ID for an acceptance subject template using its index.
    /// @dev Encodes the email account recovery version ID, "ACCEPTANCE", and the template index,
    /// then uses keccak256 to hash these values into a uint ID.
    /// @param templateIdx The index of the acceptance subject template.
    /// @return uint The computed uint ID.
    function computeAcceptanceTemplateId(
        uint templateIdx
    ) public pure returns (uint) {
        return
            uint256(
                keccak256(
                    abi.encode(
                        EMAIL_ACCOUNT_RECOVERY_VERSION_ID,
                        "ACCEPTANCE",
                        templateIdx
                    )
                )
            );
    }

    /// @notice Calculates a unique ID for a recovery subject template using its index.
    /// @dev Encodes the email account recovery version ID, "RECOVERY", and the template index,
    /// then uses keccak256 to hash these values into a uint256 ID.
    /// @param templateIdx The index of the recovery subject template.
    /// @return uint The computed uint ID.
    function computeRecoveryTemplateId(
        uint templateIdx
    ) public pure returns (uint) {
        return
            uint256(
                keccak256(
                    abi.encode(
                        EMAIL_ACCOUNT_RECOVERY_VERSION_ID,
                        "RECOVERY",
                        templateIdx
                    )
                )
            );
    }

    /// @notice Handles an acceptance by a new guardian.
    /// @dev This function validates the email auth message, deploys a new EmailAuth contract as a proxy if validations pass and initializes the contract.
    /// @param emailAuthMsg The email auth message for the email send from the guardian.
    /// @param templateIdx The index of the subject template for acceptance, which should match with the subject in the given email auth message.
    function handleAcceptance(
        EmailAuthMsg memory emailAuthMsg,
        uint templateIdx
    ) external {
        address guardian = computeEmailAuthAddress(
            emailAuthMsg.proof.accountSalt
        );
        require(
            address(guardian).code.length == 0,
            "guardian is already deployed"
        );
        uint templateId = computeAcceptanceTemplateId(templateIdx);
        require(templateId == emailAuthMsg.templateId, "invalid template id");
        require(emailAuthMsg.proof.isCodeExist == true, "isCodeExist is false");

        // Deploy proxy of the guardian's EmailAuth contract
        ERC1967Proxy proxy = new ERC1967Proxy{
            salt: emailAuthMsg.proof.accountSalt
        }(
            emailAuthImplementation(),
            abi.encodeCall(
                EmailAuth.initialize,
                (address(this), emailAuthMsg.proof.accountSalt)
            )
        );
        EmailAuth guardianEmailAuth = EmailAuth(address(proxy));
        guardianEmailAuth.updateDKIMRegistry(dkim());
        guardianEmailAuth.updateVerifier(verifier());
        for (uint idx = 0; idx < acceptanceSubjectTemplates().length; idx++) {
            guardianEmailAuth.insertSubjectTemplate(
                computeAcceptanceTemplateId(idx),
                acceptanceSubjectTemplates()[idx]
            );
        }
        for (uint idx = 0; idx < recoverySubjectTemplates().length; idx++) {
            guardianEmailAuth.insertSubjectTemplate(
                computeRecoveryTemplateId(idx),
                recoverySubjectTemplates()[idx]
            );
        }

        // An assertion to confirm that the authEmail function is executed successfully
        // and does not return an error.
        guardianEmailAuth.authEmail(emailAuthMsg);

        acceptGuardian(
            guardian,
            templateIdx,
            emailAuthMsg.subjectParams,
            emailAuthMsg.proof.emailNullifier
        );
    }

    /// @notice Processes the recovery based on an email from the guardian.
    /// @dev Verify the provided email auth message for a deployed guardian's EmailAuth contract and a specific subject template for recovery.
    /// Requires that the guardian is already deployed, and the template ID corresponds to the `templateId` in the given email auth message. Once validated.
    /// @param emailAuthMsg The email auth message for recovery.
    /// @param templateIdx The index of the subject template for recovery, which should match with the subject in the given email auth message.
    function handleRecovery(
        EmailAuthMsg memory emailAuthMsg,
        uint templateIdx
    ) external {
        address guardian = computeEmailAuthAddress(
            emailAuthMsg.proof.accountSalt
        );
        require(address(guardian).code.length > 0, "guardian is not deployed");
        uint templateId = uint256(
            keccak256(
                abi.encode(
                    EMAIL_ACCOUNT_RECOVERY_VERSION_ID,
                    "RECOVERY",
                    templateIdx
                )
            )
        );
        require(templateId == emailAuthMsg.templateId, "invalid template id");

        EmailAuth guardianEmailAuth = EmailAuth(payable(address(guardian)));

        // An assertion to confirm that the authEmail function is executed successfully
        // and does not return an error.
        guardianEmailAuth.authEmail(emailAuthMsg);

        processRecovery(
            guardian,
            templateIdx,
            emailAuthMsg.subjectParams,
            emailAuthMsg.proof.emailNullifier
        );
    }
}
