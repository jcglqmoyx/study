use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub date: i32,
}

crud!(Report{});

impl Report {
    pub fn new(date: i32) -> Self {
        Report {
            date
        }
    }
}