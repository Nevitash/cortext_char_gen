use db::db::{connect, get_connection_string};
use logs::info;
use mongodb::bson::doc;

mod r#trait;
mod die;
mod sfx;
mod disctinction;
mod db;
#[tokio::main]
async fn main() {
    init_logging().await;
    let connection_string = get_connection_string();
    let db_client = connect(&connection_string).await.expect(format!("Could not connect via {}", connection_string).as_str());
    db_client.database("admin").run_command(doc! { "ping": 1 }, None).await.expect("Ping failed");
    println!("Pinged your deployment. You successfully connected to MongoDB!");

}

async fn init_logging() {
    let stdout = log4rs::append::console::ConsoleAppender::builder().build();
    let config = log4rs::Config::builder()
        .appender(log4rs::config::Appender::builder().build("stdout", Box::new(stdout)))
        .build(log4rs::config::Root::builder().appender("stdout").build(logs::LevelFilter::Trace))
        .unwrap();
    log4rs::init_config(config).unwrap();
    info!("log4rs is initialized, ready to log!");
}