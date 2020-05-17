table! {
    reviews (id) {
        id -> Int4,
        user_id -> Int4,
        review_text -> Nullable<Text>,
        product_id -> Varchar,
        created -> Int4,
        rating -> Int4,
    }
}
