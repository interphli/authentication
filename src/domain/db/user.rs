use super::{Result, Error, super::types::{User, Value, Id}};
use aws_sdk_dynamodb::types::AttributeValueUpdate;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::types::ReturnValue;
use std::collections::HashMap;
use aws_sdk_dynamodb::Client;
use lettre::Address;



const TABLE_NAME: &'static str = "Interphlix-Users";


/// This function makes ure no user with the provided email is in our database.
///It checks if a user exists and returns an error of `UserWithEmailExists`
/// if the user does not exist the function returns `()`
pub async fn user_by_email_does_not_exist(client: &Client, email: Address) -> Result<()> {
    let output = client.get_item()
        .table_name(TABLE_NAME)
        .key("email", AttributeValue::S(email.to_string()))
        .projection_expression("id")
        .send().await?;

    match output.item {
        None => Ok(()),
        Some(_) => Err(Error::UserWithEmailAlreadyExists)
    }
}


///This function inserts a new user to the Database.
pub async fn create_user(client: &Client, user: User) -> Result<()> {
    let _ = client.put_item()
        .table_name(TABLE_NAME)
        .set_item(Some(user.into()))
        .send().await?;
    Ok(())
}


///This function updates user with the provided id
pub async fn update_user(client: &Client, id: Id, update: HashMap<String, Value>) -> Result<User> {
    let hex= id.to_hex();
    let mut builder = client.update_item().table_name(TABLE_NAME).key("id", id.into()).return_values(ReturnValue::AllNew).condition_expression("attribute_exists(id)");
    for (key, value) in update {
        let value = AttributeValueUpdate::builder().value(value.into()).build();
        builder = builder.attribute_updates(key, value)
    }
    let output = builder.send().await?;
    match output.attributes {
        None => Err(Error::InternalServerError(format!("got empty response when updating user by id {}", hex).into())),
        Some(map) => Ok(map.try_into()?)
    }
}


///This function deletes a user with the provided id
pub async fn delete_user(client: &Client, id: Id) -> Result<()> {
    let _ = client.delete_item().table_name(TABLE_NAME).key("id", id.into()).send().await?;
    Ok(())
}