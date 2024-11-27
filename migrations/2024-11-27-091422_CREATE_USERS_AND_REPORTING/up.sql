-- Your SQL goes here
CREATE TABLE Users(
    id SERIAL PRIMARY KEY ,
    username VARCHAR(150) NOT NULL,
    email VARCHAR(150) NOT NULL,
    password TEXT NOT NULL,
    staff_no VARCHAR(150) UNIQUE NOT NULL,
    license_no VARCHAR(150) UNIQUE NOT NULL,
    digi_signature TEXT NOT NULL
);

CREATE TABLE Reporting (

    id SERIAL PRIMARY KEY ,
    flight_number VARCHAR(50) NOT NULL,
    time_of_reporting TIMESTAMP NOT NULL,
    ba_reading FLOAT NOT NULL,
    medical_personal_id VARCHAR(50) NOT NULL,
    remarks TEXT,
    user_id INT NOT NULL ,
    FOREIGN KEY (user_id) REFERENCES Users(id)

);