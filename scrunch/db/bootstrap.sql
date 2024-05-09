CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created_at TEXT,
    updated_at TEXT
);

CREATE TABLE profiles (
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    username TEXT UNIQUE NOT NULL,
    background_colour TEXT,
    primary_colour TEXT,
    secondary_colour TEXT,
    image_url TEXT,
    image_alt TEXT,
    is_live INTEGER,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN Key (user_id)
        REFERENCES user (id)
);

CREATE TABLE links (
    id INTEGER NOT NULL PRIMARY KEY,
    profile_id INTEGER NOT NULL,
    href TEXT NOT NULL,
    title TEXT NOT NULL,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY (profile_id)
        REFERENCES profile (id)
);

CREATE TABLE integrations (
    id INTEGER NOT NULL PRIMARY KEY,
    profile_id INTEGER NOT NULL,
    mailchimp_subscribe_url TEXT,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY (profile_id)
        REFERENCES profile (id)
)
