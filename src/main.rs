use logs::info;

mod r#trait;
mod die;
mod sfx;
mod disctinction;
#[tokio::main]
async fn main() {
    init_logging().await;
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