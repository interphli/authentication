use super::super::types::{Error, Value, Either};
use super::super::services::table::Table;
use aws_sdk_dynamodb::operation::get_item;
use chrono::{DateTime, Utc, TimeDelta};
use std::collections::HashMap;
use aws_sdk_dynamodb::Client;
use std::fs::exists;


type Result<T> = std::result::Result<T, Error>;

/// The `Manager` trait provides a set of methods for managing items in a database.
/// It extends the `Table` trait, which provides basic CRUD operations.
pub trait Manager: Sized + Table {
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
    async fn exists(client: &Client, key: Either<<Self as Table>::PK, <Self as Table>::SK>) -> Result<bool> {
        <Self as Table>::item_exists(client, key).await
    }

    /// Creates a new item in the database.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn create(self, client: &Client) -> Result<()> {
        <Self as Table>::create_item(client, self).await
    }

    /// Reads an item from the database using its primary key.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    /// * `id` - The primary key of the item.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` with the item if found, or `None` if not found.
    async fn read(client: &Client, id: <Self as Table>::PK) -> Result<Option<Self>> {
        let key = Either::Right(id);
        <Self as Table>::get_item(client, key).await
    }

    /// Updates an existing item in the database.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    /// * `id` - The primary key of the item.
    /// * `update` - A `HashMap` containing the fields to update and their new values.
    ///
    /// # Returns
    ///
    /// A `Result` containing the updated item.
    async fn update(client: &Client, id: <Self as Table>::PK, update: HashMap<String, Value>) -> Result<Self> {
        <Self as Table>::update_item(client, id, update).await
    }

    /// Sets an expiration time for an item in the database.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    /// * `id` - The primary key of the item.
    /// * `minutes` - The number of minutes until the item expires.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn expire(client: &Client, id: Self::PK, minutes: i64) -> Result<()> {
        let timestamp = (Utc::now() + TimeDelta::minutes(minutes)).timestamp();
        <Self as Table>::expire_item(client, id, ("expires", Value::Number(timestamp.into()))).await?;
        Ok(())
    }


    /// Deletes an item from the database using its primary key.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the DynamoDB client.
    /// * `id` - The primary key of the item.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn delete(client: &Client, id: Self::PK) -> Result<()> {
        <Self as Table>::delete_item(client, id).await
    }
}
