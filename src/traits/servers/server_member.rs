use crate::models::server_member::{FieldsMember, Member, PartialMember};
use crate::Result;

#[async_trait]
pub trait AbstractServerMember: Sync + Send {
    /// Fetch a server member by their id
    async fn fetch_member(&self, server: &str, user: &str) -> Result<Member>;

    /// Insert a new server member into the database
    async fn insert_member(&self, server: &str, user: &str) -> Result<()>;

    /// Update information for a server member
    async fn update_member(
        &self,
        id: &str,
        member: &PartialMember,
        remove: Vec<FieldsMember>,
    ) -> Result<()>;

    /// Delete a server member by their id
    async fn delete_member(&self, server: &str, user: &str) -> Result<()>;

    /// Fetch multiple members by their ids
    async fn fetch_members<'a>(&self, server: &str, ids: &'a [String]) -> Result<Vec<Member>>;

    /// Fetch member count of a server
    async fn fetch_member_count(&self, server: &str) -> Result<usize>;
}
