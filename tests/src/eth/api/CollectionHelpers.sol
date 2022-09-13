// SPDX-License-Identifier: OTHER
// This code is automatically generated

pragma solidity >=0.8.0 <0.9.0;

/// @dev common stubs holder
interface Dummy {

}

interface ERC165 is Dummy {
	function supportsInterface(bytes4 interfaceID) external view returns (bool);
}

/// @dev inlined interface
interface CollectionHelpersEvents {
	event CollectionCreated(address indexed owner, address indexed collectionId);
}

/// @title Contract, which allows users to operate with collections
/// @dev the ERC-165 identifier for this interface is 0xf62c7aa9
interface CollectionHelpers is Dummy, ERC165, CollectionHelpersEvents {
	/// Create an NFT collection
	/// @param name Name of the collection
	/// @param description Informative description of the collection
	/// @param tokenPrefix Token prefix to represent the collection tokens in UI and user applications
	/// @return address Address of the newly created collection
	/// @dev EVM selector for this function is: 0x844af658,
	///  or in textual repr: createNFTCollection(string,string,string)
	function createNFTCollection(
		string memory name,
		string memory description,
		string memory tokenPrefix
	) external payable returns (address);

	/// Create an NFT collection
	/// @param name Name of the collection
	/// @param description Informative description of the collection
	/// @param tokenPrefix Token prefix to represent the collection tokens in UI and user applications
	/// @return address Address of the newly created collection
	/// @dev EVM selector for this function is: 0xe34a6844,
	///  or in textual repr: createNonfungibleCollection(string,string,string)
	function createNonfungibleCollection(
		string memory name,
		string memory description,
		string memory tokenPrefix
	) external payable returns (address);

	/// @dev EVM selector for this function is: 0xd1df968c,
	///  or in textual repr: createERC721MetadataNFTCollection(string,string,string,string)
	function createERC721MetadataNFTCollection(
		string memory name,
		string memory description,
		string memory tokenPrefix,
		string memory baseUri
	) external payable returns (address);

	/// @dev EVM selector for this function is: 0xab173450,
	///  or in textual repr: createRFTCollection(string,string,string)
	function createRFTCollection(
		string memory name,
		string memory description,
		string memory tokenPrefix
	) external payable returns (address);

	/// @dev EVM selector for this function is: 0x44a68ad5,
	///  or in textual repr: createRefungibleCollection(string,string,string)
	function createRefungibleCollection(
		string memory name,
		string memory description,
		string memory tokenPrefix
	) external payable returns (address);

	/// @dev EVM selector for this function is: 0xbea6a299,
	///  or in textual repr: createERC721MetadataRFTCollection(string,string,string,string)
	function createERC721MetadataRFTCollection(
		string memory name,
		string memory description,
		string memory tokenPrefix,
		string memory baseUri
	) external payable returns (address);

	/// Check if a collection exists
	/// @param collectionAddress Address of the collection in question
	/// @return bool Does the collection exist?
	/// @dev EVM selector for this function is: 0xc3de1494,
	///  or in textual repr: isCollectionExist(address)
	function isCollectionExist(address collectionAddress) external view returns (bool);

	/// @dev EVM selector for this function is: 0xd23a7ab1,
	///  or in textual repr: collectionCreationFee()
	function collectionCreationFee() external view returns (uint256);
}
