use serde::{Deserialize, Serialize};

pub struct User {
    pub name: String,
}

pub struct Role {
    pub name: String,
    pub users_name: Vec<String>,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    Admin = 0,
    GameCreate = 1,
    GamePlay = 2,
    GameDelete = 3,
}

impl From<Permission> for u16 {
    fn from(value: Permission) -> Self {
        value as u16
    }
}

#[deprecated(note = "this is not for production use")]
pub fn fake_query_user_permissions(user_name: String) -> Vec<Permission> {
    match user_name.as_str() {
        "admin" => vec![Permission::Admin, Permission::GameCreate, Permission::GamePlay, Permission::GameDelete],
        "game" => vec![Permission::GameCreate, Permission::GamePlay],
        _ => vec![],
    }
}
