-- Your SQL goes here
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    transaction_name VARCHAR NOT NULL,
    transaction_date  TIMESTAMP,
    transaction_status VARCHAR NOT NULL,
    card_id INTEGER NOT NULL REFERENCES cars(id),
    rental_id   INTEGER NOT NULL REFERENCES rentals(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    payment_transaction_id VARCHAR NOT NULL,
    payment_amount DECIMAL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
