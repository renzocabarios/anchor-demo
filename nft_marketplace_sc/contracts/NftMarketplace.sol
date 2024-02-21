// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

struct NFTListing {
    uint256 price;
    address seller;
}

contract NftMarketplace is ERC721URIStorage, Ownable {
    uint256 _tokenId = 0;
    mapping(uint256 => NFTListing) private _listings;

    constructor() ERC721("My MarketPlace", "MMP") Ownable(msg.sender) {}

    event NFTTransfer(
        uint256 tokenId,
        address from,
        address to,
        string tokenURI,
        uint256 price
    );

    function createNFT(string calldata tokenURI) public onlyOwner {
        _tokenId += 1;
        uint256 currentID = _tokenId;
        _safeMint(msg.sender, currentID);
        _setTokenURI(currentID, tokenURI);
        emit NFTTransfer(currentID, address(0), msg.sender, tokenURI, 0);
    }

    function listNFT(uint256 tokenId, uint256 price) public {
        require(price > 0, "NFTMarket: price must be greater than 0");
        _listings[tokenId] = NFTListing(price, msg.sender);
        transferFrom(msg.sender, address(this), tokenId);
        emit NFTTransfer(tokenId, address(0), msg.sender, "", 0);
    }

    function buyNFT(uint256 tokenId) public payable {
        NFTListing memory listing = _listings[tokenId];
        require(listing.price > 0, "NFTMarket: nft not listed for sale");
        require(msg.value == listing.price, "NFTMarket: incorrect price");
        transferFrom(address(this), msg.sender, tokenId);
        clearListing(tokenId);
        payable(listing.seller).transfer(listing.price);
        emit NFTTransfer(tokenId, address(this), msg.sender, "", 0);
    }

    function deListNFT(uint256 tokenId) public {
        NFTListing memory listing = _listings[tokenId];
        require(listing.price > 0, "NFTMarket: nft not listed for sale");
        require(
            listing.seller == msg.sender,
            "NFTMarket: you're not the seller"
        );
        transferFrom(address(this), msg.sender, tokenId);
        clearListing(tokenId);
        emit NFTTransfer(tokenId, address(this), msg.sender, "", 0);
    }

    function clearListing(uint256 tokenId) private {
        _listings[tokenId].price = 0;
        _listings[tokenId].seller = address(0);
    }
}
