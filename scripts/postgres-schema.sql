CREATE TABLE timeseries (
    id              SERIAL      NOT NULL PRIMARY KEY,
    sensor_mac      CHAR(17)    NOT NULL,
    created         TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,

    illumination    REAL        NOT NULL,
    humidity        REAL        NOT NULL,
    temperature     REAL        NOT NULL,
    voltage         REAL        NOT NULL,
    soil_humidity   REAL        NOT NULL,
    soil_salt       REAL        NOT NULL
);