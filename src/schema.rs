table! {
    chats (id) {
        id -> Uuid,
        lobby_id -> Uuid,
        message_counter -> Int4,
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    lobbys (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Text,
        nsfw -> Bool,
        enabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    records (id) {
        id -> Int4,
        chat_id -> Uuid,
        session_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    sessions (id) {
        id -> Varchar,
        address -> Inet,
        created_at -> Timestamptz,
    }
}

joinable!(chats -> lobbys (lobby_id));
joinable!(records -> chats (chat_id));
joinable!(records -> sessions (session_id));

allow_tables_to_appear_in_same_query!(
    chats,
    lobbys,
    records,
    sessions,
);
