//! Security module for QubeDB
//! Provides authentication, authorization, and encryption

pub mod auth;
pub mod tls;
pub mod rbac;
pub mod encryption;

use crate::error::QubeResult;
use std::collections::HashMap;

/// Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub enable_tls: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
    pub enable_auth: bool,
    pub jwt_secret: Option<String>,
    pub enable_rbac: bool,
    pub default_permissions: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
            enable_auth: false,
            jwt_secret: None,
            enable_rbac: false,
            default_permissions: vec!["read".to_string()],
        }
    }
}

/// User authentication info
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub created_at: u64,
    pub last_login: Option<u64>,
}

/// Security context for requests
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user: Option<User>,
    pub permissions: Vec<String>,
    pub is_authenticated: bool,
    pub request_id: String,
}

impl SecurityContext {
    pub fn new() -> Self {
        Self {
            user: None,
            permissions: vec![],
            is_authenticated: false,
            request_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn with_user(user: User) -> Self {
        Self {
            user: Some(user.clone()),
            permissions: user.permissions,
            is_authenticated: true,
            request_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    pub fn has_role(&self, role: &str) -> bool {
        if let Some(user) = &self.user {
            user.roles.contains(&role.to_string())
        } else {
            false
        }
    }
}

/// Security manager
pub struct SecurityManager {
    config: SecurityConfig,
    users: HashMap<String, User>,
    roles: HashMap<String, Vec<String>>, // role -> permissions
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        let mut manager = Self {
            config,
            users: HashMap::new(),
            roles: HashMap::new(),
        };
        
        // Initialize default roles
        manager.initialize_default_roles();
        manager
    }

    fn initialize_default_roles(&mut self) {
        // Admin role
        self.roles.insert("admin".to_string(), vec![
            "read".to_string(),
            "write".to_string(),
            "delete".to_string(),
            "create_table".to_string(),
            "drop_table".to_string(),
            "grant_permission".to_string(),
        ]);

        // User role
        self.roles.insert("user".to_string(), vec![
            "read".to_string(),
            "write".to_string(),
        ]);

        // Read-only role
        self.roles.insert("readonly".to_string(), vec![
            "read".to_string(),
        ]);
    }

    /// Authenticate user
    pub async fn authenticate(&self, username: &str, password: &str) -> QubeResult<Option<User>> {
        // In a real implementation, you would verify password hash
        if let Some(user) = self.users.get(username) {
            // For demo purposes, accept any password
            Ok(Some(user.clone()))
        } else {
            Ok(None)
        }
    }

    /// Create new user
    pub fn create_user(&mut self, username: String, email: Option<String>, roles: Vec<String>) -> QubeResult<User> {
        let permissions = self.get_permissions_for_roles(&roles);
        
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: username.clone(),
            email,
            roles,
            permissions,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_login: None,
        };

        self.users.insert(username, user.clone());
        Ok(user)
    }

    /// Get permissions for roles
    fn get_permissions_for_roles(&self, roles: &[String]) -> Vec<String> {
        let mut permissions = Vec::new();
        
        for role in roles {
            if let Some(role_permissions) = self.roles.get(role) {
                permissions.extend(role_permissions.clone());
            }
        }
        
        permissions.sort();
        permissions.dedup();
        permissions
    }

    /// Check if user has permission
    pub fn check_permission(&self, user: &User, permission: &str) -> bool {
        user.permissions.contains(&permission.to_string())
    }

    /// Create security context from user
    pub fn create_context(&self, user: &User) -> SecurityContext {
        SecurityContext::with_user(user.clone())
    }
}
