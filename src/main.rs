use std::fs;

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rusqlite::{params, Connection};

fn main() {
    let connection = Connection::open("./fat_piggy_bank.db").expect("Database connection failed.");

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS expenses (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    date TEXT NOT NULL,
                    tag TEXT NOT NULL,
                    amount INTEGER NOT NULL
                )",
            (), // empty list of parameters.
        )
        .expect("Table creation failed.");

    let data = fs::read_to_string("../../Finance/06_2024_vydaje.csv").unwrap();
    let expenses: Vec<Expense> = data
        .lines()
        .skip(2)
        .map(columns)
        .map(|cols| Expense {
            date: NaiveDate::parse_from_str(&cols[0], "%m/%d/%Y")
                .expect("Failed to parse expense date.")
                .and_time(NaiveTime::default())
                .and_utc(),
            tag: cols[1].clone(),
            amount: cols[3].split(',').next().unwrap().parse::<i32>().unwrap(),
        })
        .collect();

    for expense in expenses {
        connection
            .execute(
                "INSERT INTO expenses (date, tag, amount) VALUES (?1, ?2, ?3)",
                params![
                    expense.date,
                    expense.tag.clone(),
                    expense.amount.to_string(),
                ],
            )
            .expect("Table creation failed.");
    }
}

struct Expense {
    date: DateTime<Utc>,
    tag: String,
    amount: i32,
}

fn columns(line: &str) -> Vec<String> {
    let mut take = true;

    let mut columns: Vec<String> = vec![];

    let mut accum = String::new();
    for char in line.chars() {
        if char == '"' {
            take = !take;
        } else if char == ',' && take {
            columns.push(accum);
            accum = String::new();
        } else {
            accum.push(char);
        }
    }

    columns
}
