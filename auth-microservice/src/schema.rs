table! {
    users (id) {
        id -> Int4,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
        pass -> Text,
        tokens -> Nullable<Array<Text>>,
    }
}
