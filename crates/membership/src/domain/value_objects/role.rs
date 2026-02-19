#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Owner,
    Admin,
    Member,
}

impl Role {
    pub fn can_manage_members(self) -> bool {
        matches!(self, Role::Owner | Role::Admin)
    }

    pub fn can_delete_community(self) -> bool {
        matches!(self, Role::Owner)
    }

    pub fn can_change_roles(self) -> bool {
        matches!(self, Role::Owner)
    }
}
