#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum UserRole {
    Admin = 1,
    Moderator = 2,
    Seller = 3,
    Bidder = 4,
}

impl UserRole {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl TryFrom<i32> for UserRole {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UserRole::Admin),
            2 => Ok(UserRole::Moderator),
            3 => Ok(UserRole::Seller),
            4 => Ok(UserRole::Bidder),
            _ => Err("Invalid role value"),
        }
    }
}

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> Self {
        role.as_i32()
    }
}

pub const ROLE_ADMIN: UserRole = UserRole::Admin;
pub const ROLE_MODERATOR: UserRole = UserRole::Moderator;
pub const ROLE_SELLER: UserRole = UserRole::Seller;
pub const ROLE_BIDDER: UserRole = UserRole::Bidder;
