use rbatis::RBatis;
use rbs::to_value;

use crate::{
    entity::record::Record,
    util::time::{n_day_before_start_moment, today_start_moment},
};

pub async fn get_records_of_n_days(rb: &RBatis, n: u8) -> Vec<Record> {
    rb.query_decode("SELECT * FROM record WHERE TIME >= ? AND time < ?", vec![to_value!(n_day_before_start_moment(n)), to_value!(today_start_moment())]).await.unwrap()
}