// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

struct PublicValuesStruct {
    bytes32 leaf;
    bytes32 root;
    bool is_valid;
}

/// @title MerkleTreeVerifier.
/// @author Succinct Labs
/// @notice This contract implements a simple example of verifying the proof of a Merkle tree
///         path verification computation.
contract MerkleTreeVerifier {
    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments
    address public verifier;

    /// @notice The verification key for the merkle tree program.
    bytes32 public merkleTreeProgramVKey;

    constructor(address _verifier, bytes32 _merkleTreeProgramVKey) {
        verifier = _verifier;
        merkleTreeProgramVKey = _merkleTreeProgramVKey;
    }

    /// @notice The entrypoint for verifying the proof of a Merkle tree path verification.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    /// @return leaf The leaf hash that was verified
    /// @return root The Merkle root that was used for verification
    /// @return isValid Whether the Merkle path verification was successful
    function verifyMerkleTreeProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (bytes32 leaf, bytes32 root, bool isValid)
    {
        ISP1Verifier(verifier).verifyProof(merkleTreeProgramVKey, _publicValues, _proofBytes);
        PublicValuesStruct memory publicValues = abi.decode(_publicValues, (PublicValuesStruct));
        return (publicValues.leaf, publicValues.root, publicValues.is_valid);
    }

    /// @notice Convenience function to verify a Merkle tree proof and ensure it's valid.
    /// @param _publicValues The encoded public values.
    /// @param _proofBytes The encoded proof.
    /// @dev This function will revert if the Merkle path verification failed.
    function verifyValidMerkleTreeProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (bytes32 leaf, bytes32 root)
    {
        (bytes32 _leaf, bytes32 _root, bool isValid) = verifyMerkleTreeProof(_publicValues, _proofBytes);
        require(isValid, "MerkleTreeVerifier: Invalid Merkle path");
        return (_leaf, _root);
    }
}
