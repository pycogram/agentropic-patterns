use serde::{Deserialize, Serialize};

/// Type of role in a team
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoleType {
    /// Leader role
    Leader,
    /// Coordinator role
    Coordinator,
    /// Executor role
    Executor,
    /// Specialist role
    Specialist,
}

/// Role in a team
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
    name: String,
    role_type: RoleType,
    responsibilities: Vec<String>,
}

impl Role {
    /// Create a new role
    pub fn new(name: impl Into<String>, role_type: RoleType) -> Self {
        Self {
            name: name.into(),
            role_type,
            responsibilities: Vec::new(),
        }
    }

    /// Add a responsibility
    pub fn with_responsibility(mut self, responsibility: impl Into<String>) -> Self {
        self.responsibilities.push(responsibility.into());
        self
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the role type
    pub fn role_type(&self) -> RoleType {
        self.role_type
    }

    /// Get responsibilities
    pub fn responsibilities(&self) -> &[String] {
        &self.responsibilities
    }
}
