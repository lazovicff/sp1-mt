// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {MerkleTreeVerifier} from "../src/Fibonacci.sol";
import {SP1VerifierGateway} from "@sp1-contracts/SP1VerifierGateway.sol";

contract MerkleTreeVerifierTest is Test {
    address verifier;
    MerkleTreeVerifier public merkleTreeVerifier;

    // Mock verification key for testing
    bytes32 constant MOCK_VKEY = 0x004438d60c1749b851c995fea9f150c3c28d9a72cb19d3c49007a553f234f482;

    function setUp() public {
        verifier = address(new SP1VerifierGateway(address(1)));
        merkleTreeVerifier = new MerkleTreeVerifier(verifier, MOCK_VKEY);
    }

    function test_ValidMerkleTreeProof() public {
        // Create mock public values (leaf, root, isValid)
        bytes32 mockLeaf = keccak256("test-leaf");
        bytes32 mockRoot = keccak256("test-root");
        bool isValid = true;

        bytes memory mockPublicValues = abi.encode(mockLeaf, mockRoot, isValid);
        bytes memory mockProof = hex"1234567890abcdef"; // Mock proof bytes

        // Mock the SP1 verifier to return success
        vm.mockCall(
            verifier,
            abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector, MOCK_VKEY, mockPublicValues, mockProof),
            abi.encode(true)
        );

        (bytes32 leaf, bytes32 root, bool valid) = merkleTreeVerifier.verifyMerkleTreeProof(mockPublicValues, mockProof);

        assertEq(leaf, mockLeaf);
        assertEq(root, mockRoot);
        assertTrue(valid);
    }

    function test_ValidMerkleTreeProofConvenience() public {
        // Create mock public values (leaf, root, isValid = true)
        bytes32 mockLeaf = keccak256("test-leaf");
        bytes32 mockRoot = keccak256("test-root");
        bool isValid = true;

        bytes memory mockPublicValues = abi.encode(mockLeaf, mockRoot, isValid);
        bytes memory mockProof = hex"1234567890abcdef"; // Mock proof bytes

        // Mock the SP1 verifier to return success
        vm.mockCall(
            verifier,
            abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector, MOCK_VKEY, mockPublicValues, mockProof),
            abi.encode(true)
        );

        (bytes32 leaf, bytes32 root) = merkleTreeVerifier.verifyValidMerkleTreeProof(mockPublicValues, mockProof);

        assertEq(leaf, mockLeaf);
        assertEq(root, mockRoot);
    }

    function testRevert_InvalidMerkleTreeProof() public {
        bytes memory mockPublicValues = abi.encode(bytes32(0), bytes32(0), false);
        bytes memory fakeProof = hex"deadbeef";

        // Mock the SP1 verifier to revert (simulating invalid proof)
        vm.mockCallRevert(
            verifier,
            abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector, MOCK_VKEY, mockPublicValues, fakeProof),
            "Invalid proof"
        );

        vm.expectRevert("Invalid proof");
        merkleTreeVerifier.verifyMerkleTreeProof(mockPublicValues, fakeProof);
    }

    function testRevert_InvalidMerkleTreeProofConvenience() public {
        // Create mock public values with isValid = false
        bytes32 mockLeaf = keccak256("test-leaf");
        bytes32 mockRoot = keccak256("test-root");
        bool isValid = false;

        bytes memory mockPublicValues = abi.encode(mockLeaf, mockRoot, isValid);
        bytes memory mockProof = hex"1234567890abcdef";

        // Mock the SP1 verifier to return success (proof is valid but Merkle path is invalid)
        vm.mockCall(
            verifier,
            abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector, MOCK_VKEY, mockPublicValues, mockProof),
            abi.encode(true)
        );

        vm.expectRevert("MerkleTreeVerifier: Invalid Merkle path");
        merkleTreeVerifier.verifyValidMerkleTreeProof(mockPublicValues, mockProof);
    }

    function test_ContractInitialization() public {
        assertEq(merkleTreeVerifier.verifier(), verifier);
        assertEq(merkleTreeVerifier.merkleTreeProgramVKey(), MOCK_VKEY);
    }
}
