use rusqlite::Connection;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read command");

    let command: Vec<&str> = input.trim().split(' ').collect();

    match command[0] {
        "init" => {
            let config = Config {
                location: command[1].to_string(),
            };
            init_db(&config);
            print!("Database initialized.");
        }
        _ => println!("{} command not recognized.", input),
    }
}

struct Config {
    location: String,
}

fn init_db(config: &Config) {
    let conn = Connection::open(&config.location).expect("Database connection failed.");

    conn.execute(
        "CREATE TABLE expenses (
            id    INTEGER PRIMARY KEY,
            type  TEXT NOT NULL,
            amount INTEGER
        )",
        (), // empty list of parameters.
    )
    .expect("Table creation failed.");
}
