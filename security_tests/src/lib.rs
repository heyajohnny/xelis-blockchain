// XELIS Security Test Suite
// Comprehensive security and correctness tests for XELIS BlockDAG protocol
//
// This test suite covers:
// 1. DAG reorgs and ordering determinism
// 2. Balance versioning and supply conservation
// 3. Nonce and replay protection
// 4. Cryptographic proof validation
// 5. Difficulty and timestamp manipulation
// 6. P2P DoS protection
// 7. Smart contract gas accounting and value creation

// Note: Some tests require the full XELIS dependency tree to compile.
// If dependencies fail to resolve, the test structure and logic
// remain valuable for security review and future implementation.

mod dag_reorg_tests;
mod balance_versioning_tests;
mod nonce_replay_tests;
mod crypto_proof_tests;
mod difficulty_timestamp_tests;
mod p2p_dos_tests;
mod smart_contract_tests;

// Re-export test modules for external access
pub use dag_reorg_tests::*;
pub use balance_versioning_tests::*;
pub use nonce_replay_tests::*;
pub use crypto_proof_tests::*;
pub use difficulty_timestamp_tests::*;
pub use p2p_dos_tests::*;
pub use smart_contract_tests::*;
