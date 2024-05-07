
use futures::TryStreamExt;
use mongodb::{bson::doc, error::Error, Collection};


use super::{connect, DataError, User};

pub struct UserValidation{
    pub name_in_use: bool,
    pub email_in_use: bool
}


impl Default for UserValidation{
    fn default() -> Self {
        Self { name_in_use: false, email_in_use: false }
    }
}


pub async fn validate_email_and_name(email: &String, name: &String) -> Result<(), DataError> {
    let collection = get_collection().await?;
    let filter = doc! {
        "$or": [
            { "name": name },
            { "email": email }
        ]
    };
    let mut find_cursor = collection.find(filter,None).await?;
    while let Some(user) = find_cursor.try_next().await? {
        if user.name == *name {
            return Err(DataError::NameMustBeUnique(name.clone()));
        } else if  user.email == *email {
            return Err(DataError::EmailMustBeUnique(email.clone()));
        }
    }
    Ok(())
}

async fn get_collection() -> Result<Collection<User>, Error> {
    let client = connect(&super::get_connection_string()).await?;
    Ok(client.database(super::DB).collection::<super::User>(super::collections::USER))
}

pub async fn create_user(email: String, name: String, password_hash: String, salt: Vec<u8>) -> Result<User, DataError> {
    let collection = get_collection().await?;
}