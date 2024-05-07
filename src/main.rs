use std::net::Ipv4Addr;

use actix_web::{middleware::Logger, App, HttpServer};
use api::ApiDocs;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;
use db::{connect, get_connection_string};
use logs::info;


mod db;
mod die;
mod disctinction;
mod sfx;
mod r#trait;
mod api;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging().await;
    init_db().await;
    let openapi = ApiDocs::openapi();
    HttpServer::new(move || {
        // This factory closure is called on each worker thread independently.
        App::new()
            .wrap(Logger::default())
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            // There is no need to create RapiDoc::with_openapi because the OpenApi is served
            // via SwaggerUi. Instead we only make rapidoc to point to the existing doc.
            //
            // If we wanted to serve the schema, the following would work:
            // .service(RapiDoc::with_openapi("/api-docs/openapi2.json", openapi.clone()).path("/rapidoc"))
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await?;

    Ok(())
}

async fn init_db() -> mongodb::error::Result<()> {
    let connection_string = get_connection_string();
    let db_client = connect(&connection_string)
        .await
        .expect(format!("Could not connect via {}", connection_string).as_str());
    db::init(db_client).await?;
    Ok(())
}


async fn init_logging() {
    let stdout = log4rs::append::console::ConsoleAppender::builder().build();
    let config = log4rs::Config::builder()
        .appender(log4rs::config::Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            log4rs::config::Root::builder()
                .appender("stdout")
                .build(logs::LevelFilter::Trace),
        )
        .unwrap();
    log4rs::init_config(config).unwrap();
    info!("log4rs is initialized, ready to log!");
}
