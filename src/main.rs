use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};

fn main() {
    let expenses = vec![Expense {
        date: "1995-05-17T00:00:00Z".parse().unwrap(),
        tag: String::from("Present"),
        amount: 100,
    }];

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
