CREATE TABLE IF NOT EXISTS Users(
                                    id SERIAL PRIMARY KEY ,
                                    username VARCHAR(150) NOT NULL,
    email VARCHAR(150) NOT NULL,
    password TEXT NOT NULL,
    staff_no VARCHAR(150) UNIQUE NOT NULL,
    license_no VARCHAR(150) UNIQUE NOT NULL,
    digi_signature TEXT NOT NULL
    );

CREATE TABLE IF NOT EXISTS Machines(
                                       id SERIAL PRIMARY KEY ,
                                       under_maintenance BOOL NOT NULL,
                                       eligible_for_use BOOL DEFAULT FALSE,
                                       last_service TIMESTAMP NOT NULL,
                                       next_service TIMESTAMP NOT NULL,
                                       last_serviced_by INT,
                                       FOREIGN KEY (last_serviced_by) REFERENCES Users(id)
    );


CREATE TABLE IF NOT EXISTS SERVICE_HISTORY(
                                              id SERIAL PRIMARY KEY,
                                              machine_id INT NOT NULL,
                                              user_id INT NOT NULL,
                                              service_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                              service_notes TEXT,
                                              FOREIGN KEY (machine_id) REFERENCES Machines(id),
    FOREIGN KEY (user_id) REFERENCES Users(id)
    );
