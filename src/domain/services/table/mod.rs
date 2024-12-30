use aws_sdk_dynamodb::{Client, types::{AttributeValue, AttributeValueUpdate, ReturnValue}};
use super::super::types::{Error, Either, Value, StdError};
use std::collections::HashMap;


type Result<T> = std::result::Result<T, Error>;

pub trait Table : Into<HashMap<String, AttributeValue>> + TryFrom<HashMap<String, AttributeValue>, Error = StdError> {
    type PK : Into<AttributeValue>;
    type SK: Into<AttributeValue>;
    const NAME: &'static str;
    const PK_NAME: &'static str;
    const SK_NAME: &'static str;

    /// This function checks if an item exists in the database.
    /// It takes a client and a key (either primary or secondary) and returns a boolean.
    async fn item_exists(client: &Client, key: Either<Self::PK, Self::SK>) -> Result<bool> {
        let (key_name, key_value) = match key {
            Either::Left(pk) => (Self::PK_NAME, pk.into()),
            Either::Right(sk) => (Self::SK_NAME, sk.into()),
        };

        let output = client.get_item()
            .table_name(Self::NAME)
            .key(key_name, key_value)
            .send().await?;

        Ok(output.item.is_some())
    }

    /// This function retrieves an item from the database using either the primary key or secondary key.
    /// It returns the item if it exists.
    async fn get_item(client: &Client, key: Either<Self::PK, Self::SK>) -> Result<Option<Self>> {
        let (key_name, key_value) = match key {
            Either::Left(pk) => (Self::PK_NAME, pk.into()),
            Either::Right(sk) => (Self::SK_NAME, sk.into()),
        };

        let output = client.get_item()
            .table_name(Self::NAME)
            .key(key_name, key_value)
            .send().await?;

        match output.item {
            Some(map) => Ok(Some(map.try_into()?)),
            None => Ok(None),
        }
    }

    /// This function inserts a new item to the table.
    async fn create_item(client: &Client, item: Self) -> Result<()> {
        let _ = client.put_item()
            .table_name(Self::NAME)
            .set_item(Some(item.into()))
            .send().await?;
        Ok(())
    }

    /// This function updates an item with the provided primary key.
    async fn update_item(client: &Client, pk: Self::PK, update: HashMap<String, Value>) -> Result<Self> {
        let mut builder = client.update_item()
            .table_name(Self::NAME)
            .key(Self::PK_NAME, pk.into())
            .return_values(ReturnValue::AllNew)
            .condition_expression(&format!("attribute_exists({})", Self::PK_NAME));

        for (key, value) in update {
            let value = AttributeValueUpdate::builder().value(value.into()).build();
            builder = builder.attribute_updates(key, value);
        }

        let output = builder.send().await?;
        match output.attributes {
            None => Err(Error::InternalServerError("got empty response when updating item".into())),
            Some(map) => Ok(map.try_into()?),
        }
    }

    /// This function deletes an item with the provided primary key.
    async fn delete_item(client: &Client, pk: Self::PK) -> Result<()> {
        let _ = client.delete_item()
            .table_name(Self::NAME)
            .key(Self::PK_NAME, pk.into())
            .send().await?;
        Ok(())
    }
}
