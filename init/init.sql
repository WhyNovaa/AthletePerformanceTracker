CREATE TABLE IF NOT EXISTS Sportsmen
(
    ID  SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS Running
(
    ID SERIAL PRIMARY KEY,
    distance REAL NOT NULL,
    speed REAL NOT NULL,
    sportsman_id INT NOT NULL,
    FOREIGN KEY (sportsman_id) REFERENCES Sportsmen(ID)
);

CREATE TABLE IF NOT EXISTS Biathlon
(
    ID SERIAL PRIMARY KEY,
    accuracy REAL NOT NULL,
    distance REAL NOT NULL,
    speed REAL NOT NULL,
    sportsman_id INT NOT NULL,
    FOREIGN KEY (sportsman_id) REFERENCES Sportsmen(ID)
);

CREATE TABLE IF NOT EXISTS WeighLifting
(
    ID SERIAL PRIMARY KEY,
    weight REAL NOT NULL,
    lifted_weight REAL NOT NULL,
    sportsman_id INT NOT NULL,
    FOREIGN KEY (sportsman_id) REFERENCES Sportsmen(ID)
);
