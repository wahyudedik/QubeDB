//! Replication module for QubeDB
//! Implements data replication across cluster nodes

use crate::error::QubeResult;
use crate::types::{Row, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Replication log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub index: u64,
    pub term: u64,
    pub command: ReplicationCommand,
    pub timestamp: u64,
}

/// Replication command types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationCommand {
    Insert { table: String, key: String, row: Row },
    Update { table: String, key: String, row: Row },
    Delete { table: String, key: String },
    CreateTable { name: String, schema: HashMap<String, String> },
    DropTable { name: String },
}

/// Replication manager
pub struct ReplicationManager {
    log: Vec<LogEntry>,
    commit_index: u64,
    last_applied: u64,
    next_index: HashMap<String, u64>, // peer_id -> next_index
    match_index: HashMap<String, u64>, // peer_id -> match_index
}

impl ReplicationManager {
    pub fn new() -> Self {
        Self {
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: HashMap::new(),
            match_index: HashMap::new(),
        }
    }

    /// Append entry to replication log
    pub fn append_entry(&mut self, entry: LogEntry) -> QubeResult<()> {
        println!("📝 Appending log entry: index={}, term={}", entry.index, entry.term);
        self.log.push(entry);
        Ok(())
    }

    /// Get log entry by index
    pub fn get_entry(&self, index: u64) -> Option<&LogEntry> {
        self.log.get(index as usize)
    }

    /// Get last log index
    pub fn get_last_log_index(&self) -> u64 {
        if self.log.is_empty() {
            0
        } else {
            self.log.len() as u64 - 1
        }
    }

    /// Get last log term
    pub fn get_last_log_term(&self) -> u64 {
        if self.log.is_empty() {
            0
        } else {
            self.log.last().unwrap().term
        }
    }

    /// Commit entries up to index
    pub fn commit_to_index(&mut self, index: u64) -> QubeResult<()> {
        if index > self.commit_index {
            println!("✅ Committing entries up to index: {}", index);
            self.commit_index = index;
        }
        Ok(())
    }

    /// Apply committed entries
    pub async fn apply_committed_entries(&mut self) -> QubeResult<()> {
        while self.last_applied < self.commit_index {
            self.last_applied += 1;
            if let Some(entry) = self.get_entry(self.last_applied) {
                self.apply_entry(entry).await?;
            }
        }
        Ok(())
    }

    /// Apply a single log entry
    async fn apply_entry(&self, entry: &LogEntry) -> QubeResult<()> {
        println!("🔄 Applying log entry: {:?}", entry.command);
        
        match &entry.command {
            ReplicationCommand::Insert { table, key, row } => {
                println!("  ➕ Insert: {} -> {}", table, key);
            }
            ReplicationCommand::Update { table, key, row } => {
                println!("  🔄 Update: {} -> {}", table, key);
            }
            ReplicationCommand::Delete { table, key } => {
                println!("  ➖ Delete: {} -> {}", table, key);
            }
            ReplicationCommand::CreateTable { name, schema } => {
                println!("  📊 Create table: {} with {} columns", name, schema.len());
            }
            ReplicationCommand::DropTable { name } => {
                println!("  🗑️ Drop table: {}", name);
            }
        }
        
        Ok(())
    }

    /// Replicate to followers
    pub async fn replicate_to_followers(&mut self, followers: &[String]) -> QubeResult<()> {
        for follower_id in followers {
            self.replicate_to_follower(follower_id).await?;
        }
        Ok(())
    }

    /// Replicate to a single follower
    async fn replicate_to_follower(&mut self, follower_id: &str) -> QubeResult<()> {
        let next_idx = self.next_index.get(follower_id).copied().unwrap_or(0);
        let last_log_idx = self.get_last_log_index();
        
        if next_idx <= last_log_idx {
            println!("📤 Replicating to follower {}: entries {} to {}", 
                follower_id, next_idx, last_log_idx);
            
            // In a real implementation, this would send the log entries
            // to the follower via network RPC
        }
        
        Ok(())
    }

    /// Handle append entries response from follower
    pub fn handle_append_entries_response(
        &mut self, 
        follower_id: &str, 
        success: bool, 
        match_index: u64
    ) -> QubeResult<()> {
        if success {
            self.match_index.insert(follower_id.to_string(), match_index);
            self.next_index.insert(follower_id.to_string(), match_index + 1);
            
            // Update commit index if majority of followers have the entry
            self.update_commit_index();
        } else {
            // Decrement next index for this follower
            if let Some(current_next) = self.next_index.get(follower_id) {
                if *current_next > 0 {
                    self.next_index.insert(follower_id.to_string(), current_next - 1);
                }
            }
        }
        
        Ok(())
    }

    /// Update commit index based on match indices
    fn update_commit_index(&mut self) {
        let mut indices: Vec<u64> = self.match_index.values().copied().collect();
        indices.sort();
        
        // Find the median index (majority)
        if !indices.is_empty() {
            let median_idx = indices.len() / 2;
            let new_commit_index = indices[median_idx];
            
            if new_commit_index > self.commit_index {
                self.commit_index = new_commit_index;
                println!("📈 Updated commit index to: {}", self.commit_index);
            }
        }
    }

    /// Get replication status
    pub fn get_status(&self) -> ReplicationStatus {
        ReplicationStatus {
            log_size: self.log.len(),
            commit_index: self.commit_index,
            last_applied: self.last_applied,
            next_indices: self.next_index.clone(),
            match_indices: self.match_index.clone(),
        }
    }
}

/// Replication status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatus {
    pub log_size: usize,
    pub commit_index: u64,
    pub last_applied: u64,
    pub next_indices: HashMap<String, u64>,
    pub match_indices: HashMap<String, u64>,
}
