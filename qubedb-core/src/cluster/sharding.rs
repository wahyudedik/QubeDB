//! Sharding module for QubeDB
//! Implements horizontal partitioning and data distribution

use crate::error::QubeResult;
use crate::types::{Row, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Sharding strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShardingStrategy {
    Hash,           // Hash-based sharding
    Range,          // Range-based sharding
    Consistent,     // Consistent hashing
    Directory,      // Directory-based sharding
}

/// Shard key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardKey {
    pub table: String,
    pub key: String,
    pub hash: u64,
    pub shard_id: u32,
}

/// Shard manager
pub struct ShardManager {
    strategy: ShardingStrategy,
    shards: HashMap<u32, ShardInfo>,
    shard_count: u32,
    replication_factor: usize,
}

/// Shard information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardInfo {
    pub id: u32,
    pub range_start: String,
    pub range_end: String,
    pub nodes: Vec<String>, // Node IDs hosting this shard
    pub leader: Option<String>,
    pub status: ShardStatus,
    pub size_bytes: u64,
    pub record_count: u64,
}

/// Shard status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShardStatus {
    Active,
    Migrating,
    Recovering,
    Failed,
    ReadOnly,
}

impl ShardManager {
    pub fn new(strategy: ShardingStrategy, shard_count: u32, replication_factor: usize) -> Self {
        let mut manager = Self {
            strategy,
            shards: HashMap::new(),
            shard_count,
            replication_factor,
        };
        
        manager.initialize_shards();
        manager
    }

    /// Initialize shards
    fn initialize_shards(&mut self) {
        println!("📊 Initializing {} shards with {} replication factor", 
            self.shard_count, self.replication_factor);

        for i in 0..self.shard_count {
            let shard = ShardInfo {
                id: i,
                range_start: self.calculate_range_start(i),
                range_end: self.calculate_range_end(i),
                nodes: Vec::new(),
                leader: None,
                status: ShardStatus::Active,
                size_bytes: 0,
                record_count: 0,
            };
            
            self.shards.insert(i, shard);
        }
    }

    /// Calculate range start for a shard
    fn calculate_range_start(&self, shard_id: u32) -> String {
        match self.strategy {
            ShardingStrategy::Hash => {
                let range_size = u64::MAX / self.shard_count as u64;
                format!("{:016x}", shard_id as u64 * range_size)
            }
            ShardingStrategy::Range => {
                format!("{:08x}", shard_id * (u32::MAX / self.shard_count))
            }
            _ => format!("shard_{}", shard_id),
        }
    }

    /// Calculate range end for a shard
    fn calculate_range_end(&self, shard_id: u32) -> String {
        match self.strategy {
            ShardingStrategy::Hash => {
                let range_size = u64::MAX / self.shard_count as u64;
                format!("{:016x}", (shard_id as u64 + 1) * range_size - 1)
            }
            ShardingStrategy::Range => {
                format!("{:08x}", (shard_id + 1) * (u32::MAX / self.shard_count) - 1)
            }
            _ => format!("shard_{}", shard_id + 1),
        }
    }

    /// Get shard for a given key
    pub fn get_shard_for_key(&self, table: &str, key: &str) -> QubeResult<ShardKey> {
        let hash = self.hash_key(key);
        let shard_id = self.calculate_shard_id(hash);
        
        Ok(ShardKey {
            table: table.to_string(),
            key: key.to_string(),
            hash,
            shard_id,
        })
    }

    /// Hash a key to determine shard
    fn hash_key(&self, key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    /// Calculate shard ID from hash
    fn calculate_shard_id(&self, hash: u64) -> u32 {
        match self.strategy {
            ShardingStrategy::Hash => (hash % self.shard_count as u64) as u32,
            ShardingStrategy::Consistent => {
                // Consistent hashing implementation
                self.consistent_hash(hash)
            }
            _ => (hash % self.shard_count as u64) as u32,
        }
    }

    /// Consistent hashing implementation
    fn consistent_hash(&self, hash: u64) -> u32 {
        // Simplified consistent hashing
        // In a real implementation, this would use a ring structure
        (hash % self.shard_count as u64) as u32
    }

    /// Assign nodes to shards
    pub fn assign_nodes_to_shards(&mut self, nodes: &[String]) -> QubeResult<()> {
        println!("🔄 Assigning {} nodes to {} shards", nodes.len(), self.shard_count);

        for (shard_id, shard) in self.shards.iter_mut() {
            let mut assigned_nodes = Vec::new();
            
            // Assign primary and replicas
            for i in 0..self.replication_factor {
                let node_index = (shard_id + i as u32) % nodes.len() as u32;
                assigned_nodes.push(nodes[node_index as usize].clone());
            }
            
            shard.nodes = assigned_nodes;
            shard.leader = shard.nodes.first().cloned();
            
            println!("  📦 Shard {}: nodes={:?}, leader={:?}", 
                shard_id, shard.nodes, shard.leader);
        }

        Ok(())
    }

    /// Get shard information
    pub fn get_shard(&self, shard_id: u32) -> Option<&ShardInfo> {
        self.shards.get(&shard_id)
    }

    /// Get all shards
    pub fn get_all_shards(&self) -> &HashMap<u32, ShardInfo> {
        &self.shards
    }

    /// Check if a key belongs to a shard
    pub fn is_key_in_shard(&self, shard_id: u32, key: &str) -> bool {
        if let Some(shard) = self.shards.get(&shard_id) {
            let hash = self.hash_key(key);
            let key_shard = self.calculate_shard_id(hash);
            key_shard == shard_id
        } else {
            false
        }
    }

    /// Migrate shard to different nodes
    pub fn migrate_shard(&mut self, shard_id: u32, new_nodes: Vec<String>) -> QubeResult<()> {
        if let Some(shard) = self.shards.get_mut(&shard_id) {
            println!("🔄 Migrating shard {} to nodes: {:?}", shard_id, new_nodes);
            shard.status = ShardStatus::Migrating;
            shard.nodes = new_nodes;
            shard.leader = shard.nodes.first().cloned();
            
            // In a real implementation, this would trigger data migration
            // and update the shard status when complete
            shard.status = ShardStatus::Active;
        }
        
        Ok(())
    }

    /// Rebalance shards across nodes
    pub fn rebalance_shards(&mut self, nodes: &[String]) -> QubeResult<()> {
        println!("⚖️ Rebalancing shards across {} nodes", nodes.len());
        
        // Simple round-robin assignment
        for (shard_id, shard) in self.shards.iter_mut() {
            let mut assigned_nodes = Vec::new();
            
            for i in 0..self.replication_factor {
                let node_index = (shard_id + i as u32) % nodes.len() as u32;
                assigned_nodes.push(nodes[node_index as usize].clone());
            }
            
            shard.nodes = assigned_nodes;
            shard.leader = shard.nodes.first().cloned();
        }
        
        println!("✅ Shard rebalancing completed");
        Ok(())
    }

    /// Get sharding statistics
    pub fn get_statistics(&self) -> ShardingStatistics {
        let total_size: u64 = self.shards.values().map(|s| s.size_bytes).sum();
        let total_records: u64 = self.shards.values().map(|s| s.record_count).sum();
        let active_shards = self.shards.values()
            .filter(|s| s.status == ShardStatus::Active)
            .count();

        ShardingStatistics {
            total_shards: self.shards.len(),
            active_shards,
            total_size_bytes: total_size,
            total_records,
            average_shard_size: if self.shards.is_empty() { 0 } else { total_size / self.shards.len() as u64 },
            strategy: self.strategy.clone(),
        }
    }
}

/// Sharding statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardingStatistics {
    pub total_shards: usize,
    pub active_shards: usize,
    pub total_size_bytes: u64,
    pub total_records: u64,
    pub average_shard_size: u64,
    pub strategy: ShardingStrategy,
}
