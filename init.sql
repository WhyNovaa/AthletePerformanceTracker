CREATE TABLE IF NOT EXISTS Sportsmen
(
    ID  SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

INSERT INTO Sportsmen(name) VALUES
('ABOBA'),
('Somebody');