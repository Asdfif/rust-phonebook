#![warn(clippy::all, clippy::pedantic)]

extern crate postgres;
extern crate ini;

use ini::Ini;
use postgres::{Connection, SslMode, ConnectParams, ConnectTarget, UserInfo};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn params() -> (ConnectParams, SslMode) {
    let conf = Ini::load_from_file(".phonebookrc").unwrap();
    let general = conf.general_section();

    let host = general.get("host").unwrap();
    let port = general.get("port").unwrap();
    let sslmode = general.get("sslmode").unwrap();
    let dbname = general.get("dbname").unwrap();
    let user = general.get("user").unwrap();
    let pass = general.get("pass").unwrap();


    let sslmode_ = match sslmode {
        "disable" => SslMode::None,
        "enable" => unimplemented!(),
        _ => panic!("Wrong sslmode"),
    };

    let params = ConnectParams {
        target: ConnectTarget::Tcp(host.to_owned()),
        port: Some(port.parse().unwrap()),
        user: Some(UserInfo {
            user: user.to_owned(),
            password: Some(pass.to_owned()),
        }),
        database: Some(dbname.to_owned()),
        options: vec![],
    };
    (params, sslmode_)
}

fn main() {
    let (params, sslmode) = params();

    let conn =
        Connection::connect(
            params,
            &sslmode)
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
