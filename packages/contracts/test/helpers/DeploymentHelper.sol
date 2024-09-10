// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {EmailAuth, EmailAuthMsg} from "../../src/EmailAuth.sol";
import {Verifier, EmailProof} from "../../src/utils/Verifier.sol";
import {ECDSAOwnedDKIMRegistry} from "../../src/utils/ECDSAOwnedDKIMRegistry.sol";
import {ZKSyncCreate2Factory} from "../../src/utils/ZKSyncCreate2Factory.sol";
// import "../../src/utils/ForwardDKIMRegistry.sol";
import {UserOverrideableDKIMRegistry} from "@zk-email/contracts/UserOverrideableDKIMRegistry.sol";
import {SimpleWallet} from "./SimpleWallet.sol";
import {RecoveryController, EmailAccountRecovery} from "./RecoveryController.sol";
import {RecoveryControllerZkSync, EmailAccountRecoveryZkSync} from "./RecoveryControllerZkSync.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeploymentHelper is Test {
    using ECDSA for *;

    EmailAuth emailAuth;
    Verifier verifier;
    ECDSAOwnedDKIMRegistry dkim;
    UserOverrideableDKIMRegistry overrideableDkim;
    RecoveryController recoveryController;
    RecoveryControllerZkSync recoveryControllerZkSync;
    SimpleWallet simpleWalletImpl;
    SimpleWallet simpleWallet;

    address deployer = vm.addr(1);
    address receiver = vm.addr(2);
    address guardian;
    address newSigner = vm.addr(4);
    address someRelayer = vm.addr(5);

    bytes32 accountSalt;
    uint templateId;
    string[] subjectTemplate;
    string[] newSubjectTemplate;
    bytes mockProof = abi.encodePacked(bytes1(0x01));

    string selector = "12345";
    string domainName = "gmail.com";
    bytes32 publicKeyHash =
        0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788;
    bytes32 emailNullifier =
        0x00a83fce3d4b1c9ef0f600644c1ecc6c8115b57b1596e0e3295e2c5105fbfd8a;

    function setUp() public virtual {
        vm.startPrank(deployer);
        address signer = deployer;

        // Create DKIM registry
        {
            ECDSAOwnedDKIMRegistry ecdsaDkimImpl = new ECDSAOwnedDKIMRegistry();
            ERC1967Proxy ecdsaDkimProxy = new ERC1967Proxy(
                address(ecdsaDkimImpl),
                abi.encodeCall(ecdsaDkimImpl.initialize, (deployer, signer))
            );
            dkim = ECDSAOwnedDKIMRegistry(address(ecdsaDkimProxy));
        }
        string memory signedMsg = dkim.computeSignedMsg(
            dkim.SET_PREFIX(),
            selector,
            domainName,
            publicKeyHash
        );
        bytes32 digest = MessageHashUtils.toEthSignedMessageHash(
            bytes(signedMsg)
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        dkim.setDKIMPublicKeyHash(
            selector,
            domainName,
            publicKeyHash,
            signature
        );

        // Create userOverrideable dkim registry
        overrideableDkim = new UserOverrideableDKIMRegistry(deployer, deployer);
        overrideableDkim.setDKIMPublicKeyHash(
            domainName,
            publicKeyHash,
            deployer,
            new bytes(0)
        );

        // Create Verifier
        {
            Verifier verifierImpl = new Verifier();
            console.log(
                "Verifier implementation deployed at: %s",
                address(verifierImpl)
            );
            ERC1967Proxy verifierProxy = new ERC1967Proxy(
                address(verifierImpl),
                abi.encodeCall(verifierImpl.initialize, (msg.sender))
            );
            verifier = Verifier(address(verifierProxy));
        }
        accountSalt = 0x2c3abbf3d1171bfefee99c13bf9c47f1e8447576afd89096652a34f27b297971;

        // Create EmailAuth implementation
        EmailAuth emailAuthImpl = new EmailAuth();
        emailAuth = emailAuthImpl;

        // Create zkSync Factory implementation
        ZKSyncCreate2Factory factoryImpl = new ZKSyncCreate2Factory();

        uint templateIdx = 0;
        templateId = uint256(keccak256(abi.encodePacked("TEST", templateIdx)));
        subjectTemplate = ["Send", "{decimals}", "ETH", "to", "{ethAddr}"];
        newSubjectTemplate = ["Send", "{decimals}", "USDC", "to", "{ethAddr}"];

        // Create RecoveryController as EmailAccountRecovery implementation
        RecoveryController recoveryControllerImpl = new RecoveryController();
        ERC1967Proxy recoveryControllerProxy = new ERC1967Proxy(
            address(recoveryControllerImpl),
            abi.encodeCall(
                recoveryControllerImpl.initialize,
                (
                    signer,
                    address(verifier),
                    address(dkim),
                    address(emailAuthImpl)
                )
            )
        );
        recoveryController = RecoveryController(
            payable(address(recoveryControllerProxy))
        );

        // Create RecoveryControllerZkSync as EmailAccountRecovery implementation
        if (isZkSync()) {
            RecoveryControllerZkSync recoveryControllerZkSyncImpl = new RecoveryControllerZkSync();
            ERC1967Proxy recoveryControllerZkSyncProxy = new ERC1967Proxy(
                address(recoveryControllerZkSyncImpl),
                abi.encodeCall(
                    recoveryControllerZkSyncImpl.initialize,
                    (
                        signer,
                        address(verifier),
                        address(dkim),
                        address(emailAuthImpl),
                        address(factoryImpl)
                    )
                )
            );
            recoveryControllerZkSync = RecoveryControllerZkSync(
                payable(address(recoveryControllerZkSyncProxy))
            );
        }
        // Create SimpleWallet
        simpleWalletImpl = new SimpleWallet();
        address recoveryControllerAddress = address(recoveryController);
        if (isZkSync()) {
            recoveryControllerAddress = address(recoveryControllerZkSync);
        }


        ERC1967Proxy simpleWalletProxy = new ERC1967Proxy(
            address(simpleWalletImpl),
            abi.encodeCall(
                simpleWalletImpl.initialize,
                (signer, recoveryControllerAddress)
            )
        );
        simpleWallet = SimpleWallet(payable(address(simpleWalletProxy)));
        vm.deal(address(simpleWallet), 1 ether);

        // Set guardian address
        if (isZkSync()) {
            guardian = EmailAccountRecoveryZkSync(address(recoveryControllerZkSync))
                .computeProxyAddress(address(simpleWallet), accountSalt);
        } else {
            guardian = EmailAccountRecovery(address(recoveryController))
                .computeProxyAddress(address(simpleWallet), accountSalt);
        }
        vm.stopPrank();
    }

    function isZkSync() public view returns (bool) {
        return block.chainid == 324 || block.chainid == 300;
    }

    function skipIfZkSync() public {
        if (isZkSync()) {
            vm.skip(true);
        } else {
            vm.skip(false);
        }
    }

    function skipIfNotZkSync() public {
        if (!isZkSync()) {
            vm.skip(true);
        } else {
            vm.skip(false);
        }
    }
}
