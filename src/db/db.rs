use mongodb::{error::Result, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

pub async fn connect(uri: &String) -> Result<Client> {
    let mut client_options = ClientOptions::parse_async(uri).await?;
    client_options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());
    Client::with_options(client_options)
}

pub fn get_connection_string() -> String{
    "mongodb://root:example@mongo:27017".to_string()
}