// file: src/security/policy.rs
// Author: Alexandr Roussinov (gd2bk1ng)

use crate::security::capability::Capability;

#[derive(Debug, Clone)]
pub enum Permission { Allow, Deny }

#[derive(Debug, Clone, Default)]
pub struct Policy;

impl Policy {
  pub fn evaluate(&self, _cap: &Capability) -> Permission { Permission::Allow }
}
