use rusqlite::{Connection, NO_PARAMS};

let conn = Connection::open("my_database.db").unwrap();

conn.execute(
    "CREATE TABLE my_table (
         id              INTEGER PRIMARY KEY,
         name            TEXT NOT NULL,
         date_created    TEXT NOT NULL
     )",
    NO_PARAMS,
).unwrap();
