CREATE TABLE users (
    id          INT AUTO_INCREMENT PRIMARY KEY,
    firstname   VARCHAR(255) NOT NULL,
    lastname    VARCHAR(255) NOT NULL,
    email       VARCHAR(255) NOT NULL UNIQUE,
    password    VARCHAR(255) NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE queens (
    id          INT AUTO_INCREMENT PRIMARY KEY,
    race        VARCHAR(255) NOT NULL,
    origin      VARCHAR(255) NOT NULL,
    birth_year  INT NOT NULL,
    fertilization_site  VARCHAR(255),
    clipped     TINYINT DEFAULT 0,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE hives (
    id          INT AUTO_INCREMENT PRIMARY KEY,
    user_id     INT NOT NULL,
    name        VARCHAR(255) NOT NULL,
    temperature FLOAT,
    humidity    FLOAT,
    queen_id    INT,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT fk_hives_user_id
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_hives_queen_id
        FOREIGN KEY (queen_id) REFERENCES queens(id)
        ON DELETE SET NULL
);

CREATE TABLE inspections (
    id               INT AUTO_INCREMENT PRIMARY KEY,
    user_id          INT NOT NULL,
    hive_id          INT NOT NULL,
    queen_id         INT,
    date             DATE NOT NULL,
    behaviour        VARCHAR(255),
    queen_seen       BOOLEAN,
    honeycomb_count  INT,
    windows_occupied INT,
    BRIAS            VARCHAR(255),
    BRIAS_healthy     VARCHAR(255),
    invested_swarm_cells  INT,
    stock_food       INT,
    pollen           INT,
    mite_fall        INT,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT fk_inspections_user_id
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_inspections_hive_id
        FOREIGN KEY (hive_id) REFERENCES hives(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_inspections_queen_id
        FOREIGN KEY (queen_id) REFERENCES queens(id)
        ON DELETE SET NULL
);
