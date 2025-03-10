// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "../src/EmailSignerFactory.sol";
import "@openzeppelin/contracts/proxy/Clones.sol";

contract EmailSignerFactoryTest is Test {
    EmailSignerFactory factory;
    EmailSigner implementation;
    bytes32 constant TEST_ACCOUNT_SALT =
        keccak256(abi.encodePacked("test@example.com", "123secret"));
    address deployer = makeAddr("deployer");
    address dkim = makeAddr("dkim");
    address verifier = makeAddr("verifier");

    function setUp() public {
        vm.startPrank(deployer);

        // Deploy implementation
        implementation = new EmailSigner();

        // Deploy factory
        factory = new EmailSignerFactory(
            address(implementation),
            address(dkim),
            address(verifier)
        );

        vm.stopPrank();
    }

    function testConstructorRevertInvalidImplementation() public {
        vm.expectRevert(EmailSignerFactory.InvalidImplementation.selector);
        new EmailSignerFactory(address(0), address(dkim), address(verifier));
    }

    function testConstructorRevertInvalidDkimRegistry() public {
        vm.expectRevert(EmailSignerFactory.InvalidDKIMRegistry.selector);
        new EmailSignerFactory(
            address(implementation),
            address(0),
            address(verifier)
        );
    }

    function testConstructorRevertInvalidVerifier() public {
        vm.expectRevert(EmailSignerFactory.InvalidVerifier.selector);
        new EmailSignerFactory(
            address(implementation),
            address(dkim),
            address(0)
        );
    }

    function testPredictAddress() public view {
        // Get the predicted address from the factory
        address predicted = factory.predictAddress(TEST_ACCOUNT_SALT);

        // Use the OpenZeppelin Clones library directly to calculate the expected address
        address expected = Clones.predictDeterministicAddress(
            address(implementation),
            TEST_ACCOUNT_SALT,
            address(factory)
        );

        // Assert that the predicted address matches the expected address
        assertEq(
            predicted,
            expected,
            "Predicted address does not match expected address"
        );
    }

    function testDeployMatchesPredicted() public {
        address predicted = factory.predictAddress(TEST_ACCOUNT_SALT);
        address deployed = factory.deploy(TEST_ACCOUNT_SALT);
        assertEq(deployed, predicted);
    }

    function testDeployInitializesCorrectly() public {
        address deployed = factory.deploy(TEST_ACCOUNT_SALT);
        EmailSigner signer = EmailSigner(deployed);

        assertEq(signer.accountSalt(), TEST_ACCOUNT_SALT);
        assertEq(signer.dkimRegistryAddr(), address(dkim));
        assertEq(signer.verifierAddr(), address(verifier));
        uint256 expectedTemplateId = uint256(
            keccak256(abi.encodePacked("EMAIL-SIGNER", TEST_ACCOUNT_SALT))
        );
        assertEq(signer.templateId(), expectedTemplateId);
    }

    function testDeployEmitsEvent() public {
        bytes32 expectedSalt = keccak256(abi.encodePacked(TEST_ACCOUNT_SALT));

        vm.expectEmit(true, true, false, true);
        emit EmailSignerFactory.EmailSignerDeployed(
            factory.predictAddress(TEST_ACCOUNT_SALT),
            expectedSalt
        );

        factory.deploy(TEST_ACCOUNT_SALT);
    }

    function testMultipleDeploysAreDifferent() public {
        address first = factory.deploy("first@example.com");
        address second = factory.deploy("second@example.com");
        assertTrue(first != second);
    }

    function testSameAccountSaltGivesSameAddress() public {
        address first = factory.deploy(TEST_ACCOUNT_SALT);
        vm.expectRevert(); // Should revert on second deploy of same salt
        address second = factory.deploy(TEST_ACCOUNT_SALT);
    }
}
