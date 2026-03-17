export const CREATE_DEFAULT_TABLES = [
    `
    CREATE TABLE IF NOT EXISTS "table_versions" (
        "tableName"     TEXT NOT NULL UNIQUE,
        "version"       INTEGER NOT NULL,
        "updatedAt"     DATETIME DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY("tableName")
    );
    `,
    `
    CREATE TABLE IF NOT EXISTS "default_raids" (
        "raidId"	    INTEGER NOT NULL UNIQUE,
        "priority"      INTEGER NOT NULL,
        "level"         INTEGER NOT NULL,
        "raidName"	    TEXT NOT NULL,
        "difficulty"    TEXT NOT NULL,
        "gate"	        TEXT NOT NULL,
        "reward"	    INTEGER NOT NULL,
        "resetType"	    INTEGER NOT NULL,
        PRIMARY KEY("raidId")
    );
    `,
    `
    CREATE TABLE IF NOT EXISTS "group_raids" (
        "raidId"	    INTEGER NOT NULL UNIQUE,
        "priority"      INTEGER NOT NULL,
        "level"         INTEGER NOT NULL,
        "raidName"	    TEXT NOT NULL,
        "difficulty"    TEXT NOT NULL,
        "gate"	        TEXT NOT NULL,
        "reward"	    INTEGER NOT NULL,
        "resetType"	    INTEGER NOT NULL,
        PRIMARY KEY("raidId")
    );
    `,
    `
    CREATE TABLE IF NOT EXISTS "group_raids_map" (
        "groupId"	    INTEGER NOT NULL,
        "raidId"	    INTEGER NOT NULL,
        PRIMARY KEY("groupId", "raidId")
    );
    `,
    `
    CREATE TABLE IF NOT EXISTS "characters" (
        "charId"        INTEGER NOT NULL UNIQUE,
        "priority"      INTEGER NOT NULL,
        "charName"      TEXT NOT NULL,
        "class"         TEXT NOT NULL,
        "color"         TEXT NOT NULL DEFAULT slate,
        PRIMARY KEY("charId")
    );
    `,
    `
    CREATE TABLE IF NOT EXISTS "live_raids" (
        "id"	INTEGER NOT NULL UNIQUE,
        "charId"	INTEGER NOT NULL,
        "raidType"	TEXT NOT NULL,
        "raidId"	INTEGER NOT NULL,
        "complete"	INTEGER NOT NULL,
        "swap"	INTEGER NOT NULL,
        "resetType"	INTEGER NOT NULL DEFAULT 1,
        PRIMARY KEY("id" AUTOINCREMENT)
    );
    `
];
