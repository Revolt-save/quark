use crate::models::message::{FieldsMessage, Message, MessageSort, PartialMessage};
use crate::Result;

#[async_trait]
pub trait AbstractMessage: Sync + Send {
    /// Fetch a message by its id
    async fn fetch_message(&self, id: &str) -> Result<Message>;

    /// Insert a new message into the database
    async fn insert_message(&self, message: &Message) -> Result<()>;

    /// Update a given message with new information
    async fn update_message(
        &self,
        id: &str,
        message: &PartialMessage,
        remove: Vec<FieldsMessage>,
    ) -> Result<()>;

    /// Delete a message from the database by its id
    async fn delete_message(&self, id: &str) -> Result<()>;

    /// Fetch multiple messages
    async fn fetch_messages(
        &self,
        channel: &str,
        limit: Option<i64>,
        before: Option<String>,
        after: Option<String>,
        sort: Option<MessageSort>,
        nearby: Option<String>,
    ) -> Result<Vec<Message>>;

    /// Search for messages
    async fn search_messages(
        &self,
        channel: &str,
        query: &str,
        limit: Option<i64>,
        before: Option<String>,
        after: Option<String>,
        sort: MessageSort,
    ) -> Result<Vec<Message>>;
}
