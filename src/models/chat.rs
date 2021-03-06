use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::chats;

#[derive(Debug, Queryable, Identifiable)]
pub struct Chat {
    pub id: Uuid,
    pub lobby_id: Uuid,
    pub message_counter: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "chats"]
pub struct NewChat {
    pub lobby_id: Uuid,
    pub status: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "chats"]
pub struct UpdateChat {
    pub message_counter: i32,
    pub status: String,
    pub updated_at: NaiveDateTime,
}
