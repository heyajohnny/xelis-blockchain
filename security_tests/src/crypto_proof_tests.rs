// Security Tests: Cryptographic Proof Validation
// Tests for ZK proofs, signature verification, and cryptographic edge cases

#[cfg(test)]
mod tests {
    use xelis_common::{
        crypto::{
            elgamal::{KeyPair, Ciphertext, PublicKey, PedersenCommitment},
            proofs::{
                CommitmentEqProof, CiphertextValidityProof, BalanceProof,
                BatchCollector, ProofVerificationError,
            },
            Signature,
        },
        transaction::{Transaction, builder::TransactionBuilder},
    };
    use curve25519_dalek::{
        RistrettoPoint, Scalar,
        traits::Identity,
        ristretto::CompressedRistretto,
    };
    use merlin::Transcript;

    #[test]
    fn test_identity_point_rejection_in_signature() {
        // TEST: Identity point (point at infinity) should be rejected
        // This was historically fixed in commit aaab636e
        
        let message = b"test message";
        let identity = RistrettoPoint::identity();
        let identity_compressed = identity.compress();
        
        // Try to create public key from identity point
        // Signature verification should reject it
        
        // Note: This tests the fix from commit aaab636e
        todo!("Implement test: identity point rejection");
    }

    #[test]
    fn test_non_canonical_scalar_rejection() {
        // TEST: Non-canonical scalars should be rejected
        // Scalar value >= group order should fail
        
        // Create scalar with value >= l (group order)
        // Proof verification should reject it
        
        todo!("Implement test: non-canonical scalar");
    }

    #[test]
    fn test_non_canonical_point_rejection() {
        // TEST: Non-canonical compressed points should be rejected
        
        // Create CompressedRistretto with invalid bytes
        // Decompression should fail gracefully
        
        let invalid_bytes = [255u8; 32]; // Invalid compressed point
        let compressed = CompressedRistretto(invalid_bytes);
        
        assert!(compressed.decompress().is_none(), "Should reject invalid point");
    }

    #[test]
    fn test_zero_amount_transfer() {
        // TEST: Transfer of 0 amount should be handled correctly
        // Should not break proofs or create value
        
        let sender = KeyPair::new();
        let receiver = KeyPair::new();
        
        // Create transfer of 0 amount
        // Verify proofs are still valid
        
        todo!("Implement test: zero amount transfer");
    }

    #[test]
    fn test_self_transfer() {
        // TEST: Sender == Receiver should be rejected or handled safely
        
        let keypair = KeyPair::new();
        
        // Try to create TX from keypair to keypair
        // Should be rejected in validation
        
        todo!("Implement test: self-transfer rejection");
    }

    #[test]
    fn test_commitment_eq_proof_batch_verification() {
        // TEST: Batch verification should detect one bad proof among many good ones
        
        let mut batch_collector = BatchCollector::default();
        let mut transcript = Transcript::new(b"test");
        
        // Create 10 valid proofs
        // Add 1 invalid proof
        // Batch verification should fail
        
        todo!("Implement test: batch verification detects bad proof");
    }

    #[test]
    fn test_range_proof_boundary_values() {
        // TEST: Range proof should handle boundary values correctly
        // 0, 2^64-1, and values just outside range
        
        // Test amount = 0
        // Test amount = 2^64 - 1
        // Test amount = 2^64 (should fail)
        
        todo!("Implement test: range proof boundaries");
    }

    #[test]
    fn test_transcript_domain_separation() {
        // TEST: Different proof types should have different transcript domains
        // Prevents cross-protocol attacks
        
        // Create commitment eq proof with transcript T1
        // Try to verify as ciphertext validity proof with same values
        // Should fail due to domain separation
        
        todo!("Implement test: transcript domain separation");
    }

    #[test]
    fn test_malformed_proof_rejection() {
        // TEST: Proofs with malformed structure should be rejected
        
        // Create proof with:
        // - Invalid field sizes
        // - Missing components
        // - Extra components
        
        todo!("Implement test: malformed proof rejection");
    }

    #[test]
    fn test_proof_reuse_across_transactions() {
        // TEST: Proof from TX1 should not be valid for TX2
        // Even with same amount
        
        // Create TX1 with proof
        // Try to reuse proof in TX2
        // Should fail due to transcript binding
        
        todo!("Implement test: proof non-reusability");
    }

    #[test]
    fn test_bulletproof_aggregation() {
        // TEST: Aggregated range proof should be valid
        // for multiple outputs
        
        // Create TX with 5 outputs
        // Generate single aggregated range proof
        // Verify proof is valid for all outputs
        
        todo!("Implement test: bulletproof aggregation");
    }

    #[test]
    fn test_ciphertext_validity_proof_soundness() {
        // TEST: Ciphertext validity proof should prevent invalid ciphertexts
        
        // Try to create proof for ciphertext that doesn't match commitment
        // Proof generation or verification should fail
        
        todo!("Implement test: ciphertext validity soundness");
    }

    #[test]
    fn test_balance_proof_with_wrong_amount() {
        // TEST: Balance proof should fail if claimed amount is wrong
        
        let keypair = KeyPair::new();
        let actual_balance = 1000u64;
        let claimed_balance = 1500u64;
        
        let ct = keypair.get_public_key().encrypt(actual_balance);
        
        // Try to create balance proof claiming wrong amount
        // Should fail verification
        
        todo!("Implement test: balance proof wrong amount");
    }

    #[test]
    fn test_equality_proof_cross_cancellation() {
        // TEST: Multiple equality proofs should not allow cross-cancellation
        // This was tested in existing code but verify it's robust
        
        // Related to the test_tx_rejects_legacy_commitment_eq_cross_equation_cancellation
        // Ensure TxVersion::V2+ rejects this attack
        
        todo!("Implement test: no cross-cancellation");
    }

    #[test]
    fn test_signature_malleability() {
        // TEST: Signature should not be malleable
        
        // Create valid signature (r, s)
        // Try to create (-r, -s) or other variations
        // Should be invalid
        
        todo!("Implement test: signature malleability");
    }
}
