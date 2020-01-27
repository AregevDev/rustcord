/// Represents a Discord user that has a rich presence on
#[derive(Default, Clone, Hash, PartialEq, Debug)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
}

/// A reply to a user's request
#[derive(Clone, Hash, PartialEq, Debug)]
pub enum JoinRequestReply {
    No,
    Yes,
    Ignore,
}
