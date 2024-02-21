import {
  time,
  loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("Lock", function () {
  async function deployContract() {
    const [owner, otherAccount] = await ethers.getSigners();

    const NftMarketplace = await ethers.getContractFactory("NftMarketplace");
    const nftMarketplace = await NftMarketplace.deploy();

    return { nftMarketplace, owner, otherAccount };
  }

  describe("Deployment", function () {
    it("Should create an NFT", async function () {
      const { nftMarketplace, owner } = await loadFixture(deployContract);
      const tx = await nftMarketplace.createNFT("");
      await tx.wait();
      expect(tx.from).to.equal(owner.address);
    });

    it("Should create an NFT and List NFT", async function () {
      const { nftMarketplace, owner } = await loadFixture(deployContract);

      const tx = await nftMarketplace.createNFT("");
      const receipt = await tx.wait();

      if (receipt) {
        console.log(receipt.logs[0]);
      }
    });
  });
});
