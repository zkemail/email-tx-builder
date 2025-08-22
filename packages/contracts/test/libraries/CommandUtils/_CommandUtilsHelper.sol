// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {Test} from "forge-std/Test.sol";
import {CommandUtils} from "../../../src/libraries/CommandUtils.sol";

contract CommandUtilsHelper is Test {
    function callExtractCommandParamByIndex(string[] memory template, string memory command, uint256 paramIndex)
        public
        pure
        returns (string memory)
    {
        return CommandUtils.extractCommandParamByIndex(template, command, paramIndex);
    }
}
