use rbatis::crud;
use serde::{Deserialize, Serialize};

use crate::util::time;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u32>,
    pub username: Option<String>,
    pub wechat_id: Option<String>,
    pub join_time: Option<u64>,
    pub is_admin: Option<u32>,
    pub is_active: Option<u32>,
}

crud!(User{});

impl User {
    pub fn new(username: String, wechat_id: String) -> Self {
        User {
            id: None,
            username: Option::from(username),
            wechat_id: Option::from(wechat_id),
            join_time: Option::from(time::now()),
            is_admin: Option::from(0),
            is_active: Option::from(0),
        }
    }
}