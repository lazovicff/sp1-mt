//! An end-to-end example of using the SP1 SDK to generate a proof of a program that verifies
//! a Merkle tree path.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::SolType;
use clap::Parser;
use merkle_tree_lib::{compute_leaf_hash, hash_pair, verify_merkle_path, PublicValuesStruct};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const MERKLE_TREE_ELF: &[u8] = include_elf!("merkle-tree-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "Hello, World!")]
    data: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Create a simple Merkle tree for demonstration
    // Tree structure:
    //        root
    //       /    \
    //   node1    node2
    //   /  \     /  \
    //  leaf1 leaf2 leaf3 leaf4

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

    println!("Data: {}", args.data);
    println!("Leaf hash: {:?}", hex::encode(leaf1));
    println!("Root hash: {:?}", hex::encode(root));
    println!("Proof length: {}", proof.len());

    // Verify the path locally first
    let is_valid = verify_merkle_path(leaf1, root, &proof, &indices);
    println!("Local verification result: {}", is_valid);

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

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(MERKLE_TREE_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = PublicValuesStruct::abi_decode(output.as_slice()).unwrap();
        let PublicValuesStruct {
            leaf,
            root: output_root,
            is_valid,
        } = decoded;

        println!("Output leaf: {:?}", hex::encode(leaf));
        println!("Output root: {:?}", hex::encode(output_root));
        println!("Is valid: {}", is_valid);

        // Verify the results match our expectations
        assert_eq!(leaf.0, leaf1);
        assert_eq!(output_root.0, root);
        assert_eq!(is_valid, true);
        println!("Merkle path verification successful!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(MERKLE_TREE_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
