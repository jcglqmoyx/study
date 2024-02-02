use std::{
    collections::HashMap,
    fs::File,
    io::{self, Error, ErrorKind, Write},
    process::Command,
    time::Duration,
};

use rbatis::{RBatis, rbdc::datetime::DateTime};
use rbdc_sqlite::SqliteDriver;

use crate::{
    db::{self, record::get_records_of_n_days},
    entity::{record::Record, report::Report, rule::Rule, user::User},
    global::git::GIT_REPOSITORY_NAME,
    util::time::today_start_moment,
};

fn write_report_file(records: Vec<Record>, users: Vec<User>, task_frequency: u8) -> io::Result<()> {
    let mut data: HashMap<String, Vec<&Record>> = HashMap::new();
    let mut user_map: HashMap<u32, String> = HashMap::new();
    for user in &users {
        user_map.insert(user.id.unwrap(), user.username.clone().unwrap());
    }
    for record in &records {
        let user_id = record.user_id.unwrap();
        data.entry(user_map.get(&user_id).unwrap().to_string())
            .or_insert(Vec::new())
            .push(record);
    }

    let today = today_start_moment() / 1000000;
    let path = GIT_REPOSITORY_NAME.to_owned() + "/" + today.to_string().as_str() + ".md";
    let mut file = File::create(path)?;
    let mut content = format!("# {}年{}月{}日\n", today / 10000, today % 10000 / 100, today % 100);
    content += &format!("## 前{}天打卡记录\n", task_frequency);

    for (username, user_records) in &data {
        content.push_str(username);
        content.push_str(": ");
        for record in user_records {
            let s = format!("[{}]({}) ", record.time.unwrap(), record.url.clone().unwrap());
            content.push_str(&s);
        }
        content.push_str("<br>\n");
    }


    if users.iter().all(|user| data.contains_key(&user.username.clone().unwrap())) {
        content += "\n## 所有人均完成了打卡\n";
    } else {
        content += "## 没有完成打卡的朋友\n";
        for user in &users {
            if !data.contains_key(&user.username.clone().unwrap()) {
                content.push_str(&user.username.clone().unwrap());
                content.push(' ');
            }
        }
    }

    file.write(content.as_bytes())?;

    let path = GIT_REPOSITORY_NAME.to_owned() + "/index.md";
    let mut file = File::create(path)?;
    let content = format!("# [{}年{}月{}日]({})", today / 10000, today % 10000 / 100, today % 100, today.to_string() + ".md");
    file.write(content.as_bytes())?;

    Ok(())
}

async fn generate_report() -> io::Result<()> {
    let rb = RBatis::new();
    rb.link(SqliteDriver {}, "study.db").await.unwrap();
    let rule = Rule::select_all(&rb).await.unwrap();
    let n = rule.get(0).unwrap().task_frequency;
    let records = get_records_of_n_days(&rb, n).await;
    let users = db::user::query::list_user(&rb).await;
    write_report_file(records, users, n)
}

async fn push_to_remote_repository() -> io::Result<()> {
    let commands = format!("cd {} && git add . && git commit -m 'update' && git push", GIT_REPOSITORY_NAME);
    let output = Command::new("sh")
        .arg("-c")
        .arg(commands)
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, format!("git push failed:{}\n", String::from_utf8_lossy(&output.stderr))))
    }
}

pub async fn generate_report_scheduler() {
    loop {
        let now = DateTime::now();
        let date = now.year() * 10000 + now.mon() as i32 * 100 + now.day() as i32;

        let h = now.hour();

        let rb = RBatis::new();
        rb.link(SqliteDriver {}, "sqlite://study.db").await.unwrap();
        let rule = Rule::select_all(&rb).await.unwrap();
        let generate_report_hour = rule.get(0).unwrap().generate_report_hour;
        if h == generate_report_hour {
            match Report::select_by_column(&rb, "date", date).await {
                Ok(reports) => {
                    if reports.is_empty() {
                        println!("needs to generate report");
                        match generate_report().await {
                            Ok(_) => {
                                match push_to_remote_repository().await {
                                    Ok(_) => {}
                                    Err(_) => {}
                                }
                                let report = Report::new(date);
                                match Report::insert(&rb, &report).await {
                                    Ok(_) => {}
                                    Err(_) => {}
                                }
                            }
                            Err(_) => {}
                        }
                    } else {
                        println!("no need to generate report");
                    }
                }
                Err(_) => {}
            }
        }
        tokio::time::sleep(Duration::from_secs(60 * 10)).await;
    }
}