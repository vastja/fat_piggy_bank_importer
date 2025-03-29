use rusqlite::Connection;

fn main() {
    let expenses = vec![Expense {
        date: String::from("17-5-2025"),
        tag: String::from("Present"),
        amount: 100,
    }];

    let connection = Connection::open("./fat_piggy_bank.db").expect("Database connection failed.");

    connection
        .execute(
            "CREATE TABLE expenses (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    date DATE,
                    tag TEXT NOT NULL,
                    amount INTEGER
                )",
            (), // empty list of parameters.
        )
        .expect("Table creation failed.");

    for expense in expenses {
        connection
            .execute(
                "INSERT INTO expenses (date, tag, amount) VALUES (?1, ?2, ?3)",
                [
                    expense.date.clone(),
                    expense.tag.clone(),
                    expense.amount.to_string(),
                ],
            )
            .expect("Table creation failed.");
    }
}

struct Expense {
    date: String,
    tag: String,
    amount: i32,
}
