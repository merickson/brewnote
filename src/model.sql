CREATE TABLE IF NOT EXISTS batch (
    id  integer PRIMARY KEY,
    batchname varchar(100) NOT NULL CHECK (batchname <> ''),
    created timestamp DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS brands (
    id integer PRIMARY KEY,
    name varchar(100) NOT NULL CHECK (name <> '')
);

CREATE TYPE fermentable_type AS ENUM (
    'applejuice',
    'honey'
);

CREATE TYPE measure_type AS ENUM (
    'dry_weight_g',
    'dry_volume_l',
    'liquid_volume_l'
);

CREATE TABLE IF NOT EXISTS fermentable (
    id integer PRIMARY KEY,
    brand integer REFERENCES brands.id,
    type fermentable_type,
    size integer,
    measure measure_type,
    name varchar(100),
    CONSTRAINT UNIQUE (brand, name, size)
);

CREATE TABLE IF NOT EXISTS fermentable_batch (
    id integer PRIMARY KEY,
    fermentable integer REFERENCES fermentable.id,
    purchased timestamp DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE yeasttype AS ENUM (
    'dry',
    'liquid'
);

CREATE TABLE IF NOT EXISTS yeasts (
    id integer PRIMARY KEY,
    brand integer REFERENCES brands.id,
    name varchar(100),
    measure measure_type,
    size integer,
    type yeasttype
);

CREATE TABLE IF NOT EXISTS yeastpacket (
    id integer PRIMARY KEY,
    yeast integer REFERENCES yeasts.id,
    expiration timestamp,
    already_opened boolean
);

CREATE TABLE IF NOT EXISTS additive_types (
    id integer PRIMARY_KEY,
    name varchar(100)
)

CREATE TABLE IF NOT EXISTS additives (
    id integer PRIMARY_KEY,
    type integer REFERENCES additive_types.name,
    brand integer REFERENCES brands.id,
    measure measure_type NOT NULL,
    name varchar(100),
)

CREATE TABLE IF NOT EXISTS fermentation (
    id integer PRIMARY KEY,
    batch integer REFERENCES batch.id,
    startdate timestamp DEFAULT CURRENT_TIMESTAMP,
    enddate timestamp,
    start_specific_gravity real,
    end_specific_gravity real,
    start_ph real,
    end_ph real,
    start_ta real,
    end_ta real
);

CREATE TABLE IF NOT EXISTS fermentation_fermentables (
    fermentation integer REFERENCES fermentation.id,
    fermentable integer REFERENCES fermentable.id,
    amount real,
    CONSTRAINT UNIQUE(fermentation, fermentable) 
);

CREATE TABLE IF NOT EXISTS fermentation_yeasts (
    fermentation integer REFERENCES fermentation.id,
    yeast integer REFERENCES yeastpacket.id,
    amount real,
    CONSTRAINT UNIQUE(fermentation, fermentable)
);

CREATE TABLE IF NOT EXISTS fermentation_additives (
    fermentation integer REFERENCES fermentation.id,
    additive integer REFERENCES additives.id,
    added_time timestamp,
    amount real
)