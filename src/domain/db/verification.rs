use aws_sdk_dynamodb::{types::AttributeValue, Client};
use super::super::types::{Verification, Id};
use super::Result;
use uuid::Uuid;


const TABLE_NAME: &'static str = "Interphlix-Verification-Codes";


pub async fn create_verification(client: &Client, verification: Verification) -> Result<()> {
    let _ = client.put_item()
        .table_name(TABLE_NAME)
        .set_item(Some(verification.into()))
        .send().await?;
    Ok(())
}


pub async fn get_verification_by_magic_id(client: &Client, magic_id: Uuid) -> Result<Option<Verification>> {
    let magic_id = AttributeValue::B(magic_id.as_bytes().as_slice().into());
    let output = client.get_item()
        .table_name(TABLE_NAME)
        .key("magic_id", magic_id)
        .send().await?;
    match output.item {
        None => Ok(None),
        Some(item) => Ok(Some(item.try_into()?))
    }
}


pub async fn get_verification_code_by_user(client: &Client, user_id: Id) -> Result<Option<Verification>> {
    let output = client.get_item()
        .table_name(TABLE_NAME)
        .key("user_id", user_id.into())
        .send().await?;
    match output.item {
        None => Ok(None),
        Some(item) => Ok(Some(item.try_into()?))
    }
}