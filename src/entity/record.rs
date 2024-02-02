use rbatis::crud;
use serde::{Deserialize, Serialize};

use crate::util::time;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: Option<i32>,
    pub user_id: Option<u32>,
    pub time: Option<u64>,
    pub url: Option<String>,
}

crud!(Record{});

impl Record {
    pub fn new(user_id: u32, url: String) -> Self {
        Record {
            id: None,
            user_id: Option::from(user_id),
            time: Option::from(time::now()),
            url: Option::from(url),
        }
    }
}