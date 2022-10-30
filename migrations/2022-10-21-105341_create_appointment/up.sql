CREATE TABLE appointments (
    id SERIAL PRIMARY KEY,
    descrip VARCHAR NOT NULL,
    isApproved BOOLEAN DEFAULT NULL
)