#![warn(clippy::all, clippy::pedantic)]

extern crate postgres;
use postgres::{Connection, SslMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    let conn =
        Connection::connect(
            "postgres://alexandr_burov@localhost",
            &SslMode::None)
        .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                id      SERIAL PRIMARY KEY,
                name    VARCHAR NOT NULL,
                data    BYTEA
        )",
        &[]
    ).unwrap();

    let me = Person {
        id: 0,
        name: "Alexandr".to_string(),
        data: None
    };

    conn.execute(
        "INSERT INTO person (name, data) VAlUES ($1, $2)",
        &[&me.name, &me.data]
    ).unwrap();

    let stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    for row in stmt.query(&[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };

        print!("Found person: {} with id {}", person.name, person.id);
    };

    conn.execute("DELETE FROM person", &[]).unwrap();
    conn.execute("DROP TABLE person", &[]).unwrap();
}
