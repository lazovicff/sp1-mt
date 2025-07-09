# SP1 Merkle Tree Path Verification

This project demonstrates how to use the [SP1 RISC-V zkVM](https://github.com/succinctlabs/sp1) to generate zero-knowledge proofs for Merkle tree path verification. The project includes a complete implementation with Rust libraries, zkVM program, verification scripts, and Solidity contracts for on-chain verification.

## Overview

The project verifies Merkle tree paths using zero-knowledge proofs, allowing you to prove that a specific leaf exists in a Merkle tree without revealing the entire tree structure. This is useful for:

- Privacy-preserving membership proofs
- Scalable blockchain state verification
- Decentralized identity systems
- Private voting systems

## Project Structure

```
sp1-mt/
├── lib/                    # Merkle tree library with verification logic
├── program/               # SP1 zkVM program for proof generation
├── script/                # Scripts for execution and proof generation
├── contracts/             # Solidity contracts for on-chain verification
├── Cargo.toml            # Workspace configuration
└── README.md
```

## Features

- **Merkle Tree Path Verification**: Verify that a leaf belongs to a Merkle tree given the root and proof path
- **Zero-Knowledge Proofs**: Generate SP1 proofs for Merkle tree verification
- **EVM Compatibility**: Generate Groth16 and PLONK proofs for on-chain verification
- **Comprehensive Testing**: Unit tests for Merkle tree logic and Solidity contract tests
- **Flexible Input**: Support for arbitrary data as leaf nodes

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [SP1 toolchain](https://docs.succinct.xyz/getting-started/install.html)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) (for contract testing)
- [Docker](https://www.docker.com/) (for EVM proof generation)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd sp1-mt
```

2. Install SP1 toolchain:
```bash
curl -L https://sp1.succinct.xyz | bash
sp1up
```

3. Build the project:
```bash
cargo build --release
```

### Running Examples

#### Execute the Merkle Tree Verification Program

```bash
cd script
RUST_LOG=info cargo run --release --bin merkle-tree -- --execute --data "Hello, World!"
```

This will:
1. Create a simple 4-leaf Merkle tree
2. Generate a proof path for the specified data
3. Execute the zkVM program to verify the path
4. Display the verification results

#### Generate a Zero-Knowledge Proof

```bash
cd script
RUST_LOG=info cargo run --release --bin merkle-tree -- --prove --data "Hello, World!"
```

This generates and verifies a complete SP1 proof for the Merkle tree verification.

#### Generate EVM-Compatible Proofs

For Groth16 (requires Docker):
```bash
cd script
RUST_LOG=info cargo run --release --bin evm -- --system groth16 --data "Hello, World!"
```

For PLONK (requires Docker):
```bash
cd script
RUST_LOG=info cargo run --release --bin evm -- --system plonk --data "Hello, World!"
```

#### Get the Program Verification Key

```bash
cd script
cargo run --release --bin vkey
```

## Library Usage

The `merkle-tree-lib` crate provides core functionality for Merkle tree operations:

```rust
use merkle_tree_lib::{compute_leaf_hash, hash_pair, verify_merkle_path};

// Compute leaf hash from data
let leaf = compute_leaf_hash(b"my-data");

// Create a simple tree
let leaf1 = compute_leaf_hash(b"data1");
let leaf2 = compute_leaf_hash(b"data2");
let root = hash_pair(leaf1, leaf2);

// Verify path for leaf1
let proof = vec![leaf2];
let indices = vec![false]; // left child
let is_valid = verify_merkle_path(leaf1, root, &proof, &indices);
```

## Smart Contract Integration

The project includes Solidity contracts for on-chain verification:

```solidity
// Deploy the contract
MerkleTreeVerifier verifier = new MerkleTreeVerifier(
    SP1_VERIFIER_ADDRESS,
    PROGRAM_VERIFICATION_KEY
);

// Verify a proof
(bytes32 leaf, bytes32 root, bool isValid) = verifier.verifyMerkleTreeProof(
    publicValues,
    proofBytes
);

// Or use the convenience function that reverts on invalid paths
(bytes32 leaf, bytes32 root) = verifier.verifyValidMerkleTreeProof(
    publicValues,
    proofBytes
);
```

## Testing

### Run Rust Tests

```bash
# Test the Merkle tree library
cargo test -p merkle-tree-lib

# Test all components
cargo test
```

### Run Solidity Tests

```bash
cd contracts
forge test
```

## How It Works

### Merkle Tree Structure

The implementation uses a binary Merkle tree where:
- Leaf nodes contain hashes of data
- Internal nodes contain hashes of their children
- The root represents the entire tree

### Proof Verification

To verify a leaf belongs to the tree:

1. **Input**: Leaf hash, root hash, proof path (sibling hashes), and indices (left/right positions)
2. **Process**: Iteratively hash the current node with siblings, following the path to the root
3. **Output**: Boolean indicating if the computed root matches the expected root

### Zero-Knowledge Proof Generation

The SP1 zkVM program:
1. Reads the verification inputs (leaf, root, proof path, indices)
2. Performs the Merkle path verification
3. Commits the results as public values
4. Generates a succinct proof of correct execution

### On-Chain Verification

The Solidity contract:
1. Accepts SP1 proof bytes and public values
2. Calls the SP1 verifier to validate the proof
3. Decodes the public values to extract verification results
4. Returns the leaf, root, and validity status

## Configuration

### Environment Variables

- `SP1_PROVER`: Set to `network` for remote proving (default: `cpu`)
- `RUST_LOG`: Set logging level (e.g., `info`, `debug`)

### Customization

- **Hash Function**: Uses Keccak256 (can be changed in `lib/src/lib.rs`)
- **Tree Structure**: Currently supports binary trees (can be extended)
- **Proof Format**: Compatible with standard Merkle proof formats

## Production Considerations

### Security

- The hash function (Keccak256) is cryptographically secure
- SP1 proofs provide computational integrity guarantees
- Smart contracts should validate all inputs properly

### Performance

- Proof generation time depends on tree depth and SP1 configuration
- On-chain verification is efficient (constant time regardless of tree size)
- Consider proof caching for frequently accessed paths

### Scalability

- Tree depth affects proof size and verification time
- Consider using commitment schemes for very large trees
- Batch verification can improve throughput

## Troubleshooting

### Common Issues

1. **Docker not running**: EVM proof generation requires Docker
2. **SP1 toolchain**: Ensure SP1 is properly installed and in PATH
3. **Forge dependencies**: Run `forge install` in the contracts directory

### Performance Tips

- Use `--release` flag for better performance
- Set `SP1_PROVER=network` for faster proving (requires API key)
- Consider proof caching for repeated operations

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

- [Succinct Labs](https://succinct.xyz/) for the SP1 zkVM
- [Ethereum Foundation](https://ethereum.org/) for EVM compatibility
- [Foundry](https://github.com/foundry-rs/foundry) for Solidity tooling