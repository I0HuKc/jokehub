table! {
    jokes_tb (uuid) {
        uuid -> Uuid,
        category -> Varchar,
        language -> Varchar,
        setup -> Text,
        punchline -> Nullable<Text>,
        created_at -> Timestamp,
    }
}
