-- Your SQL goes here
CREATE TABLE rentals (
    id SERIAL PRIMARY KEY,
    rental_number VARCHAR,
    rental_date   TIMESTAMP,
    departure_time TIMESTAMP,
    arrival_time   TIMESTAMP,
    accommodation_date TIMESTAMP,
    return_date    TIMESTAMP,
    capacity INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
