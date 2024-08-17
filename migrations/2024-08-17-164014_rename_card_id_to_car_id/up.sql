-- Your SQL goes here
ALTER TABLE transactions ADD COLUMN car_id INTEGER NOT NULL REFERENCES cars(id);
