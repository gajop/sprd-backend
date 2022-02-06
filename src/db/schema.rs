table! {
    rapid_entry (id) {
        id -> Int4,
        repo_id -> Int4,
        fullname -> Text,
        hash -> Text,
        something -> Text,
        alias -> Text,
    }
}

table! {
    repo (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
    }
}

joinable!(rapid_entry -> repo (repo_id));

allow_tables_to_appear_in_same_query!(rapid_entry, repo,);
