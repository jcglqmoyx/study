use rbatis::RBatis;
use rbdc_sqlite::SqliteDriver;

use crate::entity::rule::Rule;

pub async fn init_db() {
    let rb = RBatis::new();
    rb.link(SqliteDriver {}, "sqlite://study.db").await.unwrap();

    match rb.exec("DELETE FROM rule", vec![]).await {
        Ok(_) => println!("清空rule表成功"),
        _ => println!("清空rule表失败")
    }

    let default_rule = Rule::new(0, 2);
    match Rule::insert(&rb, &default_rule).await {
        Ok(_) => println!("初始化打卡规则成功。"),
        _ => println!("初始化打卡规则失败。"),
    }
}