use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        bytes32 leaf;
        bytes32 root;
        bool is_valid;
    }
}

/// Compute the Keccak256 hash of two 32-byte values concatenated together.
pub fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(&left);
    hasher.update(&right);
    let result = hasher.finalize();
    result.into()
}

/// Verify a Merkle tree path.
///
/// # Arguments
/// * `leaf` - The leaf node hash
/// * `root` - The expected Merkle root
/// * `proof` - Array of sibling hashes in the path from leaf to root
/// * `indices` - Array of boolean values indicating whether to hash as (sibling, current) or (current, sibling)
///
/// # Returns
/// * `bool` - True if the path is valid, false otherwise
pub fn verify_merkle_path(
    leaf: [u8; 32],
    root: [u8; 32],
    proof: &[[u8; 32]],
    indices: &[bool],
) -> bool {
    if proof.len() != indices.len() {
        return false;
    }

    let mut current_hash = leaf;

    for (i, &sibling) in proof.iter().enumerate() {
        if indices[i] {
            // Current hash goes on the right, sibling on the left
            current_hash = hash_pair(sibling, current_hash);
        } else {
            // Current hash goes on the left, sibling on the right
            current_hash = hash_pair(current_hash, sibling);
        }
    }

    current_hash == root
}

/// Compute the hash of arbitrary data to create a leaf node.
pub fn compute_leaf_hash(data: &[u8]) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_pair() {
        let left = [1u8; 32];
        let right = [2u8; 32];
        let result = hash_pair(left, right);

        // The result should be deterministic
        let result2 = hash_pair(left, right);
        assert_eq!(result, result2);

        // Different order should give different result
        let result3 = hash_pair(right, left);
        assert_ne!(result, result3);
    }

    #[test]
    fn test_simple_merkle_path() {
        // Create a simple 2-level tree
        let leaf1 = compute_leaf_hash(b"data1");
        let leaf2 = compute_leaf_hash(b"data2");
        let root = hash_pair(leaf1, leaf2);

        // Verify path for leaf1 (left child)
        let proof = vec![leaf2];
        let indices = vec![false]; // leaf1 is left child, so leaf2 goes on right
        assert!(verify_merkle_path(leaf1, root, &proof, &indices));

        // Verify path for leaf2 (right child)
        let proof = vec![leaf1];
        let indices = vec![true]; // leaf2 is right child, so leaf1 goes on left
        assert!(verify_merkle_path(leaf2, root, &proof, &indices));

        // Invalid proof should fail
        let wrong_proof = vec![leaf1];
        let wrong_indices = vec![false];
        assert!(!verify_merkle_path(
            leaf1,
            root,
            &wrong_proof,
            &wrong_indices
        ));
    }

    #[test]
    fn test_three_level_merkle_tree() {
        // Create a 3-level tree with 4 leaves
        let leaf1 = compute_leaf_hash(b"data1");
        let leaf2 = compute_leaf_hash(b"data2");
        let leaf3 = compute_leaf_hash(b"data3");
        let leaf4 = compute_leaf_hash(b"data4");

        let node1 = hash_pair(leaf1, leaf2);
        let node2 = hash_pair(leaf3, leaf4);
        let root = hash_pair(node1, node2);

        // Verify path for leaf1 (left-left position)
        let proof = vec![leaf2, node2];
        let indices = vec![false, false]; // both times current goes left
        assert!(verify_merkle_path(leaf1, root, &proof, &indices));

        // Verify path for leaf4 (right-right position)
        let proof = vec![leaf3, node1];
        let indices = vec![true, true]; // both times current goes right
        assert!(verify_merkle_path(leaf4, root, &proof, &indices));
    }
}
