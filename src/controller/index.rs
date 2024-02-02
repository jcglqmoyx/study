use std::cmp::Ordering;

use actix_web::{HttpResponse, Responder, web};
use crypto::{digest::Digest, sha1::Sha1};
use rbatis::RBatis;

use crate::{
    db::user as user_entity,
    entity::{
        dto::{message::WeChatMessage, response::MessageResponse, signature::Signature},
        record::Record,
    },
    global::{command, token::WECHAT_OFFICIAL_ACCOUNT_VERIFICATION_TOKEN},
    service::{rule, user as user_service},
    util::xml::{parse_xml, to_xml},
};

pub async fn check_signature(data: web::Query<Signature>) -> impl Responder {
    let mut tmp_arr = [WECHAT_OFFICIAL_ACCOUNT_VERIFICATION_TOKEN, &data.timestamp, &data.nonce];
    tmp_arr.sort();
    let tmp_str = tmp_arr.join("");

    let mut hasher = Sha1::new();
    hasher.input_str(&tmp_str);

    match hasher.result_str().cmp(&data.signature) {
        Ordering::Equal => HttpResponse::Ok().body(data.echo_str.clone()),
        _ => HttpResponse::Ok().body("")
    }
}

async fn reply_to_text_message(msg: &WeChatMessage, rb: &RBatis) -> String {
    if msg.content.clone().unwrap().starts_with(command::REGISTRATION_COMMAND) {
        user_service::register(msg.content.clone().unwrap(), msg.from_user_name.clone(), rb).await
    } else if msg.content.clone().unwrap().starts_with(command::ACTIVATE_USER_COMMAND) {
        user_service::activate_user(msg.content.clone().unwrap(), msg.from_user_name.clone(), rb).await
    } else if msg.content.clone().unwrap() == command::LIST_USER_COMMAND {
        user_service::list_user(msg.from_user_name.clone(), rb).await
    } else if msg.content.clone().unwrap().starts_with(command::DELETE_USER_COMMAND) {
        user_service::delete_user(msg.content.clone().unwrap(), msg.from_user_name.clone(), rb).await
    } else if msg.content.clone().unwrap().starts_with(command::CHANGE_RULE_COMMAND) {
        rule::change_rule(msg.content.clone().unwrap(), msg.from_user_name.clone(), rb).await
    } else if msg.content.clone().unwrap() == command::HELP_COMMAND {
        String::from("r username: 注册账号\n\n\
                        h: 查看帮助\n\n\
                        以下操作需要管事员权限:\n\n\
                        d user_id: 删除用户\n\n\
                        a user_id: 激活用户\n\n\
                        l: 显示所有用户\n\n\
                        c frequency(在[1,255]之间): 更改打卡规则\n\n\
                    ")
    } else {
        String::from("没有此功能，请重新输入\n\n输入h可以查看帮助\n")
    }
}

async fn reply_to_image_message(msg: &WeChatMessage, rb: &RBatis) -> String {
    let mut tx = rb.acquire_begin().await.unwrap();
    let wechat_id = msg.from_user_name.clone();
    match user_entity::query::find_by_wechat_id_via_tx(&tx, &wechat_id).await {
        Some(user) => {
            if user.is_active.unwrap() == 1 {
                let record = Record::new(user.id.unwrap(), msg.pic_url.clone().unwrap());
                let insertion_result = Record::insert(&tx, &record).await;
                match insertion_result {
                    Ok(_) => {
                        tx.commit().await.unwrap();
                        String::from("图片上传成功")
                    }
                    Err(_) => {
                        tx.rollback().await.unwrap();
                        String::from("图片上传失败")
                    }
                }
            } else {
                String::from("用户未激活, 请联系管理员激活账号")
            }
        }
        None => String::from("用户未注册"),
    }
}

pub async fn reply_to_message(data: web::Bytes, rb: web::Data<RBatis>) -> impl Responder {
    let body = String::from_utf8_lossy(&data);
    match parse_xml(&body) {
        Ok(msg) => {
            let rb = rb.as_ref();
            let mut resp = MessageResponse::text_message(msg.from_user_name.clone(), msg.to_user_name.clone(), msg.create_time);
            match msg.msg_type.as_str() {
                "text" => resp.content = reply_to_text_message(&msg, rb).await,
                "image" => resp.content = reply_to_image_message(&msg, rb).await,
                _ => resp.content = String::from("该功能尚未开发"),
            }
            HttpResponse::Ok().content_type("application/xml").body(to_xml(&resp))
        }
        Err(_) => {
            println!("invalid xml format");
            HttpResponse::BadRequest().body("Invalid XML format")
        }
    }
}