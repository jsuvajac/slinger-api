table! {
    users (id) {
        id -> Uuid,
        passwd -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
