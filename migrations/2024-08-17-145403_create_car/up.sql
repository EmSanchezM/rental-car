-- Your SQL goes here
CREATE TABLE cars (
    id SERIAL PRIMARY KEY,
    car_number    Int NOT NULL,
    car_model     VARCHAR NOT NULL,
    car_color     VARCHAR NOT NULL,
    car_status    VARCHAR NOT NULL,
    rent_prize    DECIMAL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);