//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can have an
//! EVM-Compatible proof generated which can be verified on-chain.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release --bin evm -- --system groth16
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release --bin evm -- --system plonk
//! ```

use alloy_sol_types::SolType;
use clap::{Parser, ValueEnum};
use merkle_tree_lib::{compute_leaf_hash, hash_pair, PublicValuesStruct};
use serde::{Deserialize, Serialize};
use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};
use std::path::PathBuf;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const MERKLE_TREE_ELF: &[u8] = include_elf!("merkle-tree-program");

/// The arguments for the EVM command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct EVMArgs {
    #[arg(long, default_value = "Hello, World!")]
    data: String,
    #[arg(long, value_enum, default_value = "groth16")]
    system: ProofSystem,
}

/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    Plonk,
    Groth16,
}

/// A fixture that can be used to test the verification of SP1 zkVM proofs inside Solidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1MerkleTreeProofFixture {
    leaf: String,
    root: String,
    is_valid: bool,
    vkey: String,
    public_values: String,
    proof: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = EVMArgs::parse();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the program.
    let (pk, vk) = client.setup(MERKLE_TREE_ELF);

    // Create a simple Merkle tree for demonstration
    let leaf1 = compute_leaf_hash(args.data.as_bytes());
    let leaf2 = compute_leaf_hash(b"data2");
    let leaf3 = compute_leaf_hash(b"data3");
    let leaf4 = compute_leaf_hash(b"data4");

    let node1 = hash_pair(leaf1, leaf2);
    let node2 = hash_pair(leaf3, leaf4);
    let root = hash_pair(node1, node2);

    // We'll verify the path for leaf1 (position 0: left-left)
    let proof = vec![leaf2, node2]; // Sibling hashes needed for verification
    let indices = vec![false, false]; // false = current goes left, true = current goes right

    // Setup the inputs for the zkVM program
    let mut stdin = SP1Stdin::new();
    stdin.write(&leaf1);
    stdin.write(&root);
    stdin.write(&(proof.len() as u32));

    for sibling in &proof {
        stdin.write(sibling);
    }

    for &index in &indices {
        stdin.write(&index);
    }

    println!("Data: {}", args.data);
    println!("Leaf: 0x{}", hex::encode(leaf1));
    println!("Root: 0x{}", hex::encode(root));
    println!("Proof System: {:?}", args.system);

    // Generate the proof based on the selected proof system.
    let proof = match args.system {
        ProofSystem::Plonk => client.prove(&pk, &stdin).plonk().run(),
        ProofSystem::Groth16 => client.prove(&pk, &stdin).groth16().run(),
    }
    .expect("failed to generate proof");

    create_proof_fixture(&proof, &vk, args.system);
}

/// Create a fixture for the given proof.
fn create_proof_fixture(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
    system: ProofSystem,
) {
    // Deserialize the public values.
    let bytes = proof.public_values.as_slice();
    let PublicValuesStruct {
        leaf,
        root,
        is_valid,
    } = PublicValuesStruct::abi_decode(bytes).unwrap();

    // Create the testing fixture so we can test things end-to-end.
    let fixture = SP1MerkleTreeProofFixture {
        leaf: format!("0x{}", hex::encode(leaf.0)),
        root: format!("0x{}", hex::encode(root.0)),
        is_valid,
        vkey: vk.bytes32().to_string(),
        public_values: format!("0x{}", hex::encode(bytes)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
    };

    // The verification key is used to verify that the proof corresponds to the execution of the
    // program on the given input.
    //
    // Note that the verification key stays the same regardless of the input.
    println!("Verification Key: {}", fixture.vkey);

    // The public values are the values which are publicly committed to by the zkVM.
    //
    // If you need to expose the inputs or outputs of your program, you should commit them in
    // the public values.
    println!("Public Values: {}", fixture.public_values);

    // The proof proves to the verifier that the program was executed with some inputs that led to
    // the give public values.
    println!("Proof Bytes: {}", fixture.proof);

    // Save the fixture to a file.
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures");
    std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");
    std::fs::write(
        fixture_path.join(format!("{:?}-fixture.json", system).to_lowercase()),
        serde_json::to_string_pretty(&fixture).unwrap(),
    )
    .expect("failed to write fixture");
}
