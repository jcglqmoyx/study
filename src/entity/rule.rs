use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub generate_report_hour: u8,
    pub task_frequency: u8,
}

crud!(Rule{});

impl Rule {
    pub fn new(generate_report_hour: u8, task_frequency: u8) -> Self {
        Rule {
            generate_report_hour,
            task_frequency,
        }
    }
}
