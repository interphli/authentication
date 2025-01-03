use super::super::types::{Error, Either, Value, StdError, Verification, Id, Uuid, EmailAddress, User};
use aws_sdk_dynamodb::{Client, types::{AttributeValue, AttributeValueUpdate, ReturnValue}};
use std::collections::HashMap;



type Result<T> = std::result::Result<T, Error>;

/// The `Table` trait provides a set of methods for interacting with a database table.
/// It requires the implementing type to be convertible to and from a `HashMap` of `AttributeValue`.
pub trait Table : Into<HashMap<String, AttributeValue>> + TryFrom<HashMap<String, AttributeValue>, Error = StdError> {
    type PK : Into<AttributeValue>;
    type SK: Into<AttributeValue>;
    const NAME: &'static str;
    const PK_NAME: &'static str;
    /// This is the Global Secondary Index's PK.
    const SK_NAME: &'static str;

    /// Checks if an item exists in the database.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    /// * `key` - The primary or secondary key of the item.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating whether the item exists.
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

    /// Retrieves an item from the database using either the primary key or secondary key.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    /// * `key` - The primary or secondary key of the item.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` with the item if found, or `None` if not found.
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

    /// Inserts a new item into the table.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    /// * `item` - The item to be inserted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn create_item(client: &Client, item: Self) -> Result<()> {
        let _ = client.put_item()
            .table_name(Self::NAME)
            .set_item(Some(item.into()))
            .send().await?;
        Ok(())
    }

    /// Updates an item with the provided primary key.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    /// * `pk` - The primary key of the item.
    /// * `update` - A `HashMap` containing the fields to update and their new values.
    ///
    /// # Returns
    ///
    /// A `Result` containing the updated item.
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


    async fn expire_item(client: &Client, pk: Self::PK, (key, value): (impl Into<String>, Value)) -> Result<()> {
        let value = AttributeValueUpdate::builder().value(value.into()).build();
        let _ = client.update_item().table_name(Self::NAME).key(Self::PK_NAME, pk.into()).condition_expression(&format!("attribute_exists({})", Self::PK_NAME)).send().await?;
        Ok(())
    }

    /// Deletes an item with the provided primary key.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    /// * `pk` - The primary key of the item.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn delete_item(client: &Client, pk: Self::PK) -> Result<()> {
        let _ = client.delete_item()
            .table_name(Self::NAME)
            .key(Self::PK_NAME, pk.into())
            .send().await?;
        Ok(())
    }
}



impl Table for Verification {
    type PK = Id;
    type SK = Uuid;
    const NAME: &'static str = "Interphlix-Verification-Codes";
    const PK_NAME: &'static str = "user_id";
    const SK_NAME: &'static str = "magic_id";
}


impl Table for User {
    type PK = Id;
    type SK = EmailAddress;
    const NAME: &'static str = "Interphlix-Users";
    const PK_NAME: &'static str = "id";
    const SK_NAME: &'static str = "email";
}
