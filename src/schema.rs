table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        content -> Text,
        published -> Bool,
        created_at -> Date,
        updated_at -> Date,
        active -> Bool,
        image_path -> Varchar,
        sub_title -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Date,
        updated_at -> Date,
        active -> Bool,
        email_verify -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
