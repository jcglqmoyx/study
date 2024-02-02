extern crate rbatis;
extern crate rbdc_sqlite;

use actix_web::{App, HttpServer, web};
use rbatis::RBatis;
use rbdc_sqlite::SqliteDriver;

use study::init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init::git::pull_git_repository().await;
    init::db::init_db().await;

    let rb = RBatis::new();
    rb.init(SqliteDriver {}, "sqlite://study.db").unwrap();

    tokio::spawn(study::scheduler::report::generate_report_scheduler());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(rb.clone()))
            .route("/", web::get().to(study::controller::index::check_signature))
            .route("/", web::post().to(study::controller::index::reply_to_message))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
