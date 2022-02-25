table! {
    users (user) {
        user -> Text,
        pass_hash -> Text,
        perm_level -> Nullable<Integer>,
    }
}
