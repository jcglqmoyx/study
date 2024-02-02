use rbatis::RBatis;
use rbs::to_value;

use crate::{db::user, global::command::CHANGE_RULE_COMMAND};

pub async fn change_rule(content: String, wechat_id: String, rb: &RBatis) -> String {
    if user::query::is_admin(wechat_id, rb).await {
        let task_frequency = &content[CHANGE_RULE_COMMAND.len()..content.len()];
        match task_frequency.parse::<u8>() {
            Ok(frequency) => {
                match frequency {
                    0 => String::from("task_frequency{}不合法, 请输入一个[1, 255]之间的数字"),
                    _ => {
                        let update_sql = "UPDATE rule SET task_frequency = ?";
                        match rb.exec(update_sql, vec![to_value!(frequency)]).await {
                            Ok(_) => String::from("规则更新成功"),
                            Err(_) => return String::from("规则更新失败"),
                        }
                    }
                }
            }
            Err(_) => {
                format!("task_frequency{}不合法, 请输入一个[1, 255]之间的数字", task_frequency)
            }
        }
    } else {
        String::from("非管理员账号，无法进行该操作")
    }
}
