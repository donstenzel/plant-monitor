CREATE TABLE [PlantTypes] (
  id          INTEGER NOT NULL,
  name        TEXT    NOT NULL,
  scientific  TEXT    NOT NULL,
  description TEXT    NOT NULL,
  ml_per_day  INTEGER NOT NULL,
  ideal_ph    REAL    NOT NULL,
  ideal_temp  REAL    NOT NULL,
  ideal_hum   REAL    NOT NULL,
  ideal_ec    REAL    NOT NULL,
  ideal_lux   INTEGER NOT NULL,
  image       TEXT    NOT NULL,

  PRIMARY KEY (id)
);

CREATE TABLE [PotTypes] (
  id       INTEGER NOT NULL,
  name     TEXT    NOT NULL,
  drainage INTEGER NOT NULL,
  volume   INTEGER NOT NULL,
  image    TEXT    NOT NULL,

  PRIMARY KEY (id)
);

CREATE TABLE [Usages] (
  id      INTEGER  NOT NULL,
  plant   INTEGER  NOT NULL,
  pot     INTEGER  NOT NULL,
  planted DATETIME NOT NULL,

  PRIMARY KEY (id),
  FOREIGN KEY (plant) REFERENCES [PlantTypes](id) ON DELETE CASCADE,
  FOREIGN KEY (pot)   REFERENCES [PotTypes](id)   ON DELETE CASCADE
);

CREATE TABLE [Measurements] (
  id          INTEGER  NOT NULL,
  usage       INTEGER  NOT NULL,
  humidity    REAL     NOT NULL,
  temperature REAL     NOT NULL,
  lux         INTEGER  NOT NULL,
  ph          REAL     NOT NULL,
  ec          REAL     NOT NULL,
  instant     DATETIME NOT NULL,

  PRIMARY KEY (id),
  FOREIGN KEY (usage) REFERENCES [Usages](id) ON DELETE CASCADE
);
