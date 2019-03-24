table! {
    repo (namespace, name) {
        namespace -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Int4,
        name -> Varchar,
        nickname -> Nullable<Varchar>,
        email -> Varchar,
        password_hash -> Varchar,
        token -> Nullable<Varchar>,
        token_secret -> Nullable<Varchar>,
        token_expiry -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(repo, user,);
