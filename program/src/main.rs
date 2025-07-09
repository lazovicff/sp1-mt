//! A simple program that takes Merkle tree verification inputs and verifies a Merkle path.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use merkle_tree_lib::{verify_merkle_path, PublicValuesStruct};

pub fn main() {
    // Read the leaf hash (32 bytes)
    let leaf: [u8; 32] = sp1_zkvm::io::read::<[u8; 32]>();

    // Read the expected root hash (32 bytes)
    let root: [u8; 32] = sp1_zkvm::io::read::<[u8; 32]>();

    // Read the proof length
    let proof_len: u32 = sp1_zkvm::io::read::<u32>();

    // Read the proof (array of sibling hashes)
    let mut proof = Vec::new();
    for _ in 0..proof_len {
        let sibling: [u8; 32] = sp1_zkvm::io::read::<[u8; 32]>();
        proof.push(sibling);
    }

    // Read the indices (boolean array indicating left/right positions)
    let mut indices = Vec::new();
    for _ in 0..proof_len {
        let index: bool = sp1_zkvm::io::read::<bool>();
        indices.push(index);
    }

    // Verify the Merkle path
    let is_valid = verify_merkle_path(leaf, root, &proof, &indices);

    // Encode the public values of the program
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        leaf: leaf.into(),
        root: root.into(),
        is_valid,
    });

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
