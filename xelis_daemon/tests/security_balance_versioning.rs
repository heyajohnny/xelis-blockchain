// Security test: Balance versioning and output_balance tracking
use xelis_daemon::core::storage::{
    sled::{SledStorage, SledConfig},
    BalanceProvider,
    VersionedBalanceProvider,
};
use xelis_common::crypto::{Hash, KeyPair};
use xelis_common::account::{VersionedBalance, BalanceType};
use xelis_common::serializer::Serializer;
use xelis_common::network::Network;
use tempfile::TempDir;

// Helper for tests (storage needs to be owned, not Arc, for mutation)
async fn with_test_storage<F, Fut>(test: F)
where
    F: FnOnce(SledStorage) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let temp_dir = TempDir::with_prefix("balance_test").unwrap();
    let config = SledConfig::default();
    let storage = SledStorage::new(
        temp_dir.path().to_str().unwrap().to_string(),
        if config.cache_size > 0 { Some(config.cache_size) } else { None },
        Network::Devnet,
        config.internal_cache_size,
        config.internal_db_mode,
        1
    ).unwrap();
    test(storage).await;
}

#[tokio::test]
async fn test_balance_versioning_tracks_output_balance() {
    // Test that VersionedBalance properly tracks output_balance field
    with_test_storage(|mut storage| async move {
        use xelis_daemon::core::storage::AssetProvider;
        use xelis_common::asset::{AssetData, MaxSupplyMode, AssetOwner};
        use xelis_common::versioned::Versioned;
    
        // Create a test account
        let keypair = KeyPair::new();
        let account = keypair.get_public_key().compress();
        let asset = Hash::zero();
        
        // Register the asset first
        let data = Versioned::new(AssetData::new(8, "Test".to_string(), "TST".to_string(), MaxSupplyMode::Fixed(1000000), AssetOwner::None), None);
        storage.add_asset(&asset, 0, data).await.unwrap();
    
        // Insert a versioned balance at topoheight 0
        let balance = VersionedBalance::zero();
        
        storage.set_last_balance_to(&account, &asset, 0, &balance).await.unwrap();
    
        // Retrieve and verify
        let result = storage.get_last_balance(&account, &asset).await;
        assert!(result.is_ok(), "Should retrieve balance");
        
        let (topo, stored_balance) = result.unwrap();
        assert_eq!(topo, 0, "Topoheight should match");
        // Zero balance has a zero ciphertext
        assert!(stored_balance.has_output_balance() == false, "Zero balance should not have output balance initially");
    }).await;
}

#[tokio::test]
async fn test_balance_versioning_across_reorg() {
    // Test that balance versioning survives reorgs (V1 hardfork feature)
    with_test_storage(|mut storage| async move {
        use xelis_daemon::core::storage::AssetProvider;
    
        let keypair = KeyPair::new();
        let account = keypair.get_public_key().compress();
        let asset = Hash::zero();
        
        // Register the asset
        use xelis_common::asset::{AssetData, MaxSupplyMode, AssetOwner};
        use xelis_common::versioned::Versioned;
        let data = Versioned::new(AssetData::new(8, "Test".to_string(), "TST".to_string(), MaxSupplyMode::Fixed(1000000), AssetOwner::None), None);
        storage.add_asset(&asset, 0, data).await.unwrap();
    
        // Simulate a balance at topoheight 10 with previous topoheight
        let mut balance1 = VersionedBalance::zero();
    balance1.set_previous_topoheight(Some(5));
    storage.set_last_balance_to(&account, &asset, 10, &balance1).await.unwrap();
    
    // Simulate a different balance at topoheight 15 (after some blocks)
    let mut balance2 = VersionedBalance::zero();
    balance2.set_previous_topoheight(Some(10));
    storage.set_last_balance_to(&account, &asset, 15, &balance2).await.unwrap();
    
        // Verify we can get the latest balance
        let (topo, final_balance) = storage.get_last_balance(&account, &asset).await.unwrap();
        assert_eq!(topo, 15, "Should have latest topoheight");
        assert_eq!(final_balance.get_previous_topoheight(), Some(10), "Should track previous topoheight");
    }).await;
}

#[tokio::test]
async fn test_delete_versioned_balances_above_topoheight() {
    // Test pruning of versioned balance data above a topoheight
    with_test_storage(|mut storage| async move {
        use xelis_daemon::core::storage::AssetProvider;
    
        let keypair = KeyPair::new();
        let account = keypair.get_public_key().compress();
        let asset = Hash::zero();
        
        // Register the asset
        use xelis_common::asset::{AssetData, MaxSupplyMode, AssetOwner};
        use xelis_common::versioned::Versioned;
        let data = Versioned::new(AssetData::new(8, "Test".to_string(), "TST".to_string(), MaxSupplyMode::Fixed(1000000), AssetOwner::None), None);
        storage.add_asset(&asset, 0, data).await.unwrap();
    
        // Create balance at topoheight 15
    let balance = VersionedBalance::zero();
    storage.set_last_balance_to(&account, &asset, 15, &balance).await.unwrap();
    
    // Get initial balance
    let (topo_before, _) = storage.get_last_balance(&account, &asset).await.unwrap();
    assert_eq!(topo_before, 15, "Balance should exist at topoheight 15");
    
        // Delete all versions above topoheight 10 (removes all account data)
        storage.delete_versioned_balances_above_topoheight(10).await.unwrap();
        
        // Note: This deletes ALL versioned balance data above topoheight 10 across all accounts
        // This is used during chain reorgs to prune invalidated state
    }).await;
}

#[test]
fn test_versioned_balance_serialization() {
    use xelis_common::serializer::{Reader, Writer};
    
    // Test that VersionedBalance serializes correctly (important for storage)
    let mut balance = VersionedBalance::zero();
    balance.set_previous_topoheight(Some(5));
    balance.set_balance_type(BalanceType::Both);
    
    let mut bytes = Vec::new();
    {
        let mut writer = Writer::new(&mut bytes);
        balance.write(&mut writer);
    }
    
    assert!(!bytes.is_empty(), "Serialized balance should not be empty");
    
    // Deserialize and verify
    let mut reader = Reader::new(&bytes);
    let deserialized = VersionedBalance::read(&mut reader).unwrap();
    assert_eq!(deserialized.get_previous_topoheight(), Some(5));
    assert_eq!(deserialized.get_balance_type(), BalanceType::Both);
}

#[test]
fn test_versioned_balance_output_balance_field() {
    // Test output_balance field specifically (used in tx validation)
    let mut balance = VersionedBalance::zero();
    
    // Initially no output balance
    assert!(!balance.has_output_balance(), "Should not have output balance initially");
    
    // Set output balance
    let zero_cache = balance.get_balance().clone();
    balance.set_output_balance(Some(zero_cache));
    
    // Verify it's tracked separately
    assert!(balance.has_output_balance(), "Output balance should be set");
    
    // Test balance type tracking
    balance.set_balance_type(BalanceType::Output);
    assert!(balance.contains_output(), "Should contain output");
    assert!(!balance.contains_input(), "Should not contain input for Output type");
}
