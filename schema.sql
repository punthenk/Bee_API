CREATE TABLE IF NOT EXISTS hives (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    temperature FLOAT,
    humidity FLOAT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert some test data
INSERT INTO hives (name, location, temperature, humidity) VALUES
    ('North Garden Hive', 35.5, 65.0);

INSERT INTO hives (name, location, temperature, humidity) VALUES
    ('South Meadow Hive', 34.2, 68.5);

INSERT INTO hives (name, location, temperature, humidity) VALUES
    ('West Orchard Hive', NULL, NULL);
