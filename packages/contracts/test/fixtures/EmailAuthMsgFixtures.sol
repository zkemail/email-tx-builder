// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {EmailAuthMsg} from "../../src/interfaces/IEmailTypes.sol";
import {EmailProof} from "../../src/interfaces/IVerifier.sol";

library EmailAuthMsgFixtures {
    /**
     * @dev Case 1: Sign Hash with a specific hash value
     * This fixture represents an email command to sign a hash value.
     * The command format is "signHash {uint}"
     * Used for testing basic hash signing functionality
     */
    function getCase1() internal pure returns (EmailAuthMsg memory) {
        EmailAuthMsg memory emailAuthMsg;

        emailAuthMsg.commandParams = new bytes[](1);
        emailAuthMsg.commandParams[
            0
        ] = hex"da6c88e5c11236cd71d4733f45851c91257ba8d4c7f99145f72848e326126152";

        emailAuthMsg.proof = EmailProof({
            accountSalt: hex"0ea4525e6e02fdbb98062196b1130f73f6f36b460bb3fcb1e062e8722da28c4f",
            domainName: "gmail.com",
            emailNullifier: hex"154b7529c9ee0651a9c8a585644fdf514ada243bb3a13f439226f5f81acb0480",
            isCodeExist: true,
            maskedCommand: "signHash 98795965305811853593942095979598763998273224478639454298374304707044663517522",
            proof: hex"2f5a59c14c784cc1a1372afd21321219f67b14d12f037f8e54e60d856b304cae030b497830f7680ab884bb9234182b76e497abc0799b971ff98284e8a9efed5b2f1286fb8dabbff86906bb61d93586040acac5a3a287d050087fbe5c7a703d501a43e86fff29aac12cb3515253e9cbcea55f5b20c519b8c379fe5cd0a950ed2c1e8ea2c1dbb5ee4a6a8ad92235f0873692a9647d5fbf2b1f1b3f078111d9609619025ca1dd38ce32d3ae908b8043f3fe159ceef2a5751374a7566906f4475b9c2f681a64aea3ca4aa1e24b3ca0644727f774b9963f6f88949aca93b72008811c03d2ea773c4af8c654f0ce4c8cab80faf7e6074d57cd44daef2b088ebe4b7f03",
            publicKeyHash: hex"0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            timestamp: 1744555398
        });

        emailAuthMsg.skippedCommandPrefix = 0;
        emailAuthMsg
            .templateId = 0x1bd88348ccb7396aa7a29d6f7107c793b5b24b8cec1ccfdd9de3f1d61ab6c1dd;

        return emailAuthMsg;
    }

    /**
     * @dev Case 2: Sign Hash with a different hash value
     * This fixture represents another email command to sign a different hash value.
     * The command format is "signHash {uint}"
     * Used for testing multiple hash signing scenarios
     */
    function getCase2() internal pure returns (EmailAuthMsg memory) {
        EmailAuthMsg memory emailAuthMsg;

        emailAuthMsg.commandParams = new bytes[](1);
        emailAuthMsg.commandParams[
            0
        ] = hex"8ae164ca3498628c6d5c2bcd904b80c4f93d886feb7da1bca6d74a59e9c7a4cb";

        emailAuthMsg.proof = EmailProof({
            accountSalt: hex"23c274a6e4d88ded1bf05d44762b855545ddc134f54750adfa25e3386ba979cd",
            domainName: "gmail.com",
            emailNullifier: hex"1a350f11b783eae6a5fddbf960c9bf7ca55b04727a8c4b61d77a3df1cce8f9d1",
            isCodeExist: true,
            maskedCommand: "signHash 62817409320148730591830218376583920457123489321048213478932100011234567890123",
            proof: hex"2cbec20c8c6bdda378100088a7a5705aec7247d13b06cb4544bcba7f53d5170922c84a56f91e478cdf26ec642e855fbeaafe8bc8f78b30509274f8954078cfd5115957f6bc25cd50e4181b0c22e83e34f47456c424762c8bcf3b838972fcc2092d7a71b4285e7a23b826a6bc99e35dd1bb1bd9fcf1c2801941a0564753f5b35d2790ba0c9866085c647ae741886164617236584346d852b3e0b335833e35faea009fd83613b3971ab3410cdb0ae4b51098dc30b7bae91aaa0829ecaa9ef4b3aa0338d4faba1cb22c367811a6e52a3e6113e4f6574c27ba6b30d2a12edfa3975a140c8a4261a3cec8c0ff4b0ee3cf76941cb3d9b8c21d13a5f99f97d2a71c301a",
            publicKeyHash: hex"0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            timestamp: 1744555928
        });

        emailAuthMsg.skippedCommandPrefix = 0;
        emailAuthMsg
            .templateId = 0xc093a1009d021018079012b25b827ac3754eb0993625879a8b8ec3dd5ff8b3a7;

        return emailAuthMsg;
    }

    /**
     * @dev Case 3: Send ETH Transaction
     * This fixture represents an email command to send ETH to a recipient.
     * The command format is "Send {uint} ETH to {ethAddr}"
     * Used for testing ETH transfer functionality
     */
    function getCase3() internal pure returns (EmailAuthMsg memory) {
        EmailAuthMsg memory emailAuthMsg;

        emailAuthMsg.commandParams = new bytes[](2);
        emailAuthMsg.commandParams[
            0
        ] = hex"000000000000000000000000000000000000000000000000016345785d8a0000";
        emailAuthMsg.commandParams[
            1
        ] = hex"000000000000000000000000afbd210c60dd651892a61804a989eef7bd63cba0";

        emailAuthMsg.proof = EmailProof({
            accountSalt: hex"03ae56d1cac90dc5febde08736e6ce7f7e5d9db03239bff41f4ac929446f5f83",
            domainName: "gmail.com",
            emailNullifier: hex"0088956aa41bd520408c38c6e1749c7e7921d71af752f8360aa09ac808ef0100",
            isCodeExist: true,
            maskedCommand: "Send 0.1 ETH to 0xafBD210c60dD651892a61804A989eEF7bD63CBA0",
            proof: hex"26c108b983baeb89b98e1c6dc2e6b8cdfcb51840c1a8fe066e731a6636b3276b29fc2fb2ab56bd26dee30daaebb45330d3c6a20176b38884488a772eb6c727da22c6b1ce7335547c18fd62684e8f8af915775c2e78e15fc869e15893911bba761d15863f0372f800d855d99742b44ef0bfb1de0565997cf155fc56afce7c4eca21744162e9b6293d500af7e13023274a75d057f1f875b90054b97d026157a56d12fe5b2cbf31f4c1cd82f510d8a5bf8ff46a142810a03f8f9aca71db1b97485a0ee7ccf3cd75ebff97fb7fa5d7dfbbed4b2e1508906ed179054596b67f83cde105d0d61ced5b447c7218906fff48588bd2ea2ed1d952dd56107cda7b156fb666",
            publicKeyHash: hex"0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            timestamp: 1744556343
        });

        emailAuthMsg.skippedCommandPrefix = 0;
        emailAuthMsg
            .templateId = 0xe986edec44591c33db673df2346f92142660eaa9c3e88e6bf744317d2b43516a;

        return emailAuthMsg;
    }

    /**
     * @dev Case 4: Accept Guardian Request
     * This fixture represents an email command to accept a guardian request.
     * The command format is "Accept guardian request for {ethAddr}"
     * Used for testing guardian management functionality
     */
    function getCase4() internal pure returns (EmailAuthMsg memory) {
        EmailAuthMsg memory emailAuthMsg;

        emailAuthMsg.commandParams = new bytes[](1);
        emailAuthMsg.commandParams[
            0
        ] = hex"000000000000000000000000afbd210c60dd651892a61804a989eef7bd63cba0";

        emailAuthMsg.proof = EmailProof({
            accountSalt: hex"03ae56d1cac90dc5febde08736e6ce7f7e5d9db03239bff41f4ac929446f5f83",
            domainName: "gmail.com",
            emailNullifier: hex"2d40a9b0e331940f1b3df6f79833f591b4cbe88bcd6fab3d2033c8f0256d6a94",
            isCodeExist: true,
            maskedCommand: "Accept guardian request for 0xafBD210c60dD651892a61804A989eEF7bD63CBA0",
            proof: hex"256ba69feff9fbf8567081e16e97d5e6a1e08582c2091c338cd7db713aa06baf1cd76c9f756fcee010af5c105f3dac267fd7c913e988309dc830b023b890856e0306c6f1a097f40c7d2207e0f08e2f12141e8a58c9dc0aeb44dea7a64aef7f092222e924a089c363d53a80c46ec7373a38e724c2c17fe90388a0f8fbc0274c6e29afdd61a8aa790014506233793f11955fab2a5ac233e15fcea7722b162d3927007f89769cdbcbb401ba7aeaa4e642e81c3ae0df3bc8e14dba1cbd8660d947d91f23af06fac604ab75090e1bec608478c9990b5832800d3bbfd0657b2b3ae53d230e90f3b30f5cbdbc725e00b2336594cc37172301b083644dc44eb10e60874a",
            publicKeyHash: hex"0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            timestamp: 1744556534
        });

        emailAuthMsg.skippedCommandPrefix = 0;
        emailAuthMsg
            .templateId = 0x462ab7844474673315f0e17f86abf5dd680a9086a372a8b8c8c0a3c7fae198f3;

        return emailAuthMsg;
    }
}
