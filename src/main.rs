use rusqlite::Connection;
use std::{
    fs,
    path::{Path, PathBuf},
    result::Result,
};

fn main() {
    let mut input = String::new();
    std::io::stdin()
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
            id INTEGER PRIMARY KEY,
            date DATE
            type TEXT NOT NULL,
            amount INTEGER
        )",
        (), // empty list of parameters.
    )
    .expect("Table creation failed.");
}

struct Expense {}

fn load_reports(path: &Path) -> std::io::Result<Vec<Expense>> {
    let reports: Vec<PathBuf> = match path.is_dir() {
        true => path
            .read_dir()?
            .map(|entry| entry.map(|x| x.path()))
            .collect::<Result<Vec<_>, _>>()?,
        false => vec![PathBuf::from(path)],
    };

    let expenses = reports
        .into_iter()
        .map(|x| load_report(x.as_path()))
        .collect::<std::io::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(expenses)
}

fn load_report(path: &Path) -> std::io::Result<Vec<Expense>> {
    let expenses: Vec<Expense> = fs::read_to_string(path)?
        .lines()
        .map(|_| Expense {})
        .collect();

    Ok(expenses)
}
