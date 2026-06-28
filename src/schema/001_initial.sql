-- Resonance Knowledge Schema — Initial Migration

CREATE TABLE IF NOT EXISTS atoms (
    id          TEXT PRIMARY KEY,
    term        TEXT NOT NULL UNIQUE,
    definition  TEXT NOT NULL,
    etymology   TEXT,
    color       TEXT,
    sound       TEXT,
    texture     TEXT,
    temperature TEXT,
    created_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS molecules (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    definition  TEXT NOT NULL,
    atom_ids    TEXT NOT NULL DEFAULT '[]',
    schema_json TEXT NOT NULL DEFAULT '{}',
    created_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS categories (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    parent_id   TEXT REFERENCES categories(id),
    created_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS senses (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    emoji       TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    created_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS subcategories (
    id          TEXT PRIMARY KEY,
    sense_id    TEXT NOT NULL REFERENCES senses(id),
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    created_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(sense_id, name)
);

CREATE TABLE IF NOT EXISTS emoji_definitions (
    id          TEXT PRIMARY KEY,
    emoji       TEXT NOT NULL UNIQUE,
    label       TEXT NOT NULL,
    category    TEXT NOT NULL DEFAULT 'general',
    keywords    TEXT NOT NULL DEFAULT '[]',
    color       TEXT,
    sound       TEXT,
    texture     TEXT,
    temperature TEXT,
    definition  TEXT,
    created_at  INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_emoji_defs_category ON emoji_definitions(category);
CREATE INDEX IF NOT EXISTS idx_subcategories_sense ON subcategories(sense_id);
CREATE INDEX IF NOT EXISTS idx_atoms_term ON atoms(term);
