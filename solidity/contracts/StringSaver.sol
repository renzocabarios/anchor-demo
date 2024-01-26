// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract StringSaver {
    string public data;

    function setData(string memory _data) public payable {
        data = _data;
    }
}
