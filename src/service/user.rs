use rbatis::RBatis;

use crate::{
    db::user,
    entity::user::User,
    global::command::{DELETE_USER_COMMAND, REGISTRATION_COMMAND},
    util::string::concatenate_usernames,
};
use crate::global::command::ACTIVATE_USER_COMMAND;

pub async fn register(content: String, wechat_id: String, rb: &RBatis) -> String {
    let username: String = content.chars().skip(REGISTRATION_COMMAND.len()).take(content.len() - REGISTRATION_COMMAND.len()).collect();
    let mut tx = rb.acquire_begin().await.unwrap();
    if !user::query::find_by_wechat_id_via_tx(&tx, &wechat_id).await.is_none() {
        String::from("您已经注册过账号，请勿重复注册")
    } else if !user::query::find_by_username(&tx, username.clone()).await.is_none() {
        String::from("该用户名已经被占用，请换一个用户名")
    } else {
        let mut user = User::new(username.clone(), wechat_id);

        let mut information = String::new();
        if user::query::count_user(&tx).await == 0 {
            user.is_admin = Some(1);
            user.is_active = Some(1);
            information.push_str("您的账号是管理员账号");
        } else {
            information.push_str("请联系管理员激活账号");
        }

        let data = User::insert(&tx, &user).await;
        match data {
            Ok(_) => {
                tx.commit().await.unwrap();
                format!("注册成功, 您的用户名是: {}, {}", username, information)
            }
            Err(_) => {
                tx.rollback().await.unwrap();
                String::from("注册失败, 请稍候再试")
            }
        }
    }
}

pub async fn activate_user(content: String, wechat_id: String, rb: &RBatis) -> String {
    match user::query::find_by_wechat_id_via_rb(&rb, &wechat_id).await {
        None => {
            return String::from("未注册账号，无法进行该操作");
        }
        _ => {}
    }
    if user::query::is_admin(wechat_id, rb).await {
        let id = &content[ACTIVATE_USER_COMMAND.len()..content.len()];
        match id.parse::<u32>() {
            Ok(id) => {
                let users = User::select_by_column(rb, "id", id).await.unwrap();
                if users.is_empty() {
                    String::from("用户不存在，无法激活")
                } else {
                    let mut user = users.get(0).unwrap().clone();
                    user.is_active = Option::from(1);
                    match User::update_by_column(rb, &user, "id").await {
                        Ok(exec_result) => {
                            match exec_result.rows_affected {
                                0 => String::from("用户激活失败"),
                                _ => String::from("用户激活成功"),
                            }
                        }
                        Err(_) => {
                            String::from("发生了内部错误, 请稍候再试")
                        }
                    }
                }
            }
            Err(_) => {
                format!("id{}不合法，格式错误\n", id)
            }
        }
    } else {
        String::from("非管理员账号，无法进行该操作")
    }
}

pub async fn list_user(wechat_id: String, rb: &RBatis) -> String {
    match user::query::find_by_wechat_id_via_rb(&rb, &wechat_id).await {
        None => {
            return String::from("未注册账号，无法进行该操作");
        }
        _ => {}
    }
    if user::query::is_admin(wechat_id, rb).await {
        let users = user::query::list_user(rb).await;
        concatenate_usernames(&users)
    } else {
        String::from("非管理员账号，无法进行该操作")
    }
}


pub async fn delete_user(content: String, wechat_id: String, rb: &RBatis) -> String {
    match user::query::find_by_wechat_id_via_rb(&rb, &wechat_id).await {
        None => {
            return String::from("未注册账号，无法进行该操作");
        }
        _ => {}
    }
    if user::query::is_admin(wechat_id, rb).await {
        let id = &content[DELETE_USER_COMMAND.len()..content.len()];
        match id.parse::<u32>() {
            Ok(id) => {
                match User::delete_by_column(rb, "id", id).await {
                    Ok(exec_result) => {
                        match exec_result.rows_affected {
                            0 => String::from("删除失败"),
                            _ => String::from("删除成功"),
                        }
                    }
                    Err(_) => {
                        String::from("内部错误")
                    }
                }
            }
            Err(_) => {
                format!("id{}不合法，格式错误\n", id)
            }
        }
    } else {
        String::from("非管理员账号，无法进行该操作")
    }
}