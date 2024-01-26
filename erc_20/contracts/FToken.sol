// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract FToken is ERC20 {
    constructor() ERC20("FungibleToken", "FToken") {}
}
