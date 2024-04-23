use mongodb::{
    bson::{doc, Uuid},
    error::Result,
    options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion},
    Client, IndexModel,
};
use serde::{Deserialize, Serialize};

use crate::{disctinction::Distinction, r#trait::TraitSet};

pub mod collections {
    pub const USER: &str = "user";
    pub const CHARACTER: &str = "character";
    pub const GAME: &str = "game";
}

pub const DB: &str = "chargen";

pub async fn connect(uri: &String) -> Result<Client> {
    let mut client_options = ClientOptions::parse_async(uri).await?;
    client_options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());
    Client::with_options(client_options)
}

pub fn get_connection_string() -> String {
    "mongodb://root:example@mongo:27017".to_string()
}

pub async fn init(client: Client) -> Result<()> {
    init_user(&client).await?;
    init_character(&client).await?;
    init_game(&client).await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    id: Uuid,
    name: String,
    email: String,
    salt: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    #[serde(rename = "_id")]
    id: Uuid,
    user_id: Uuid,
    name: String,
    main_trait: String,
    distinctions: Vec<Distinction>,
    trait_sets: Vec<TraitSet>,
    assets: Vec<String>,
    complications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_id")]
    id: Uuid,
    character_ids: Vec<Uuid>,
    name: String,
    owner_id: Uuid,
    player_ids: Vec<Uuid>,
}

pub async fn init_game(client: &Client) -> mongodb::error::Result<()> {
    let db = client.database(DB);
    let validator = doc! {
        "$jsonSchema": {
            "bsonType": "object",
            "required": ["owner_id", "name"],
            "properties": {
                "name": {
                    "bsonType": "string",
                    "description": "must be a string and is required"
                },
                "owner_id": {
                    "bsonType": "string",
                    "description": "Uuid of the owner of the game. Required"
                }
            }
        }
    };
    let options = mongodb::options::CreateCollectionOptions::builder()
        .validator(validator)
        .build();
    db.create_collection(collections::GAME, options).await?;

    let collection = db.collection::<Game>(collections::GAME);
    let non_unique_options = IndexOptions::builder().unique(false).build();

    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {"owner_id": 1})
                .options(non_unique_options.clone())
                .build(),
            None,
        )
        .await?;
    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {"owner_id": 1, "name": 1})
                .options(non_unique_options)
                .build(),
            None,
        )
        .await?;

    Ok(())
}

pub async fn init_character(client: &Client) -> mongodb::error::Result<()> {
    let db = client.database(DB);
    let validator = doc! {
        "$jsonSchema": {
            "bsonType": "object",
            "required": ["user_id", "name"],
            "properties": {
                "name": {
                    "bsonType": "string",
                    "description": "must be a string and is required"
                },
                "user_id": {
                    "bsonType": "binData",
                    "description": "Relation to user. Required"
                }
            }
        }
    };
    let options = mongodb::options::CreateCollectionOptions::builder()
        .validator(validator)
        .build();
    db.create_collection(collections::CHARACTER, options)
        .await?;

    let collection = db.collection::<Character>(collections::CHARACTER);
    let non_unique_options = IndexOptions::builder().unique(false).build();

    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {"name": 1})
                .options(non_unique_options.clone())
                .build(),
            None,
        )
        .await?;
    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {"user_id": 1, "name": 1})
                .options(non_unique_options)
                .build(),
            None,
        )
        .await?;

    Ok(())
}

pub async fn init_user(client: &Client) -> mongodb::error::Result<()> {
    let db = client.database(DB);
    let validator = doc! {
        "$jsonSchema": {
            "bsonType": "object",
            "required": ["name", "email", "salt"],
            "properties": {
                "name": {
                    "bsonType": "string",
                    "description": "must be a string and is required"
                },
                "email": {
                    "bsonType": "string",
                    "description": "must be a string and is required"
                },
                "salt": {
                    "bsonType": "binData",
                    "description": "must be a uuid and is required"
                }
            }
        }
    };
    let options = mongodb::options::CreateCollectionOptions::builder()
        .validator(validator)
        .build();
    db.create_collection(collections::USER, options).await?;

    let collection = db.collection::<User>(collections::USER);
    let unique_options = IndexOptions::builder().unique(true).build();
    let non_unique_options = IndexOptions::builder().unique(false).build();

    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {"email": 1})
                .options(unique_options.clone())
                .build(),
            None,
        )
        .await?;
    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {"salt": 1})
                .options(unique_options)
                .build(),
            None,
        )
        .await?;
    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {"email": 1, "salt": 1})
                .options(non_unique_options)
                .build(),
            None,
        )
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use mongodb::bson::doc;
    use mongodb::error::Result;
    use crate::db::{connect, get_connection_string, DB};

    #[tokio::test]
    async fn db_connection_test() -> Result<()> {
        let connection_string = get_connection_string();
        let db_client = connect(&connection_string)
            .await
            .expect(format!("Could not connect via {}", connection_string).as_str());
        db_client
            .database(DB)
            .run_command(doc! { "ping": 1 }, None)
            .await?;
        
        Ok(())
    }
}
