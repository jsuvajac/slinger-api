table! {
    spell_book (name) {
        user_id -> Uuid,
        name -> Varchar,
        content -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        passwd -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(spell_book -> users (user_id));

allow_tables_to_appear_in_same_query!(
    spell_book,
    users,
);
