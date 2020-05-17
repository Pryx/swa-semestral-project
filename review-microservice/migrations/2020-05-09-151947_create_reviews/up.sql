-- Your SQL goes here
CREATE TABLE reviews
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    review_text text,
    product_id varchar NOT NULL,
    created INTEGER NOT NULL,
    rating INTEGER NOT NULL
);