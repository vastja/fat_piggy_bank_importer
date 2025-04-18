use std::fs;

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rusqlite::{params, Connection, Result};
fn main() {
    let connection = Connection::open("./fat_piggy_bank.db").expect("Database connection failed.");

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS expenses (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    date TEXT NOT NULL,
                    tag_id INTEGER NOT NULL,
                    amount INTEGER NOT NULL,
                    FOREIGN KEY (tag_id) REFERENCES tags(id)
                )",
            (), // empty list of parameters.
        )
        .expect("Expenses table creation failed.");

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                color TEXT NOT NULL
            )",
            (),
        )
        .expect("Tags table creation failed.");

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

    let mut tags_select = connection
        .prepare("SELECT * FROM tags")
        .expect("Selecting tags failed.");

    let existing_tags: Vec<DbTag> = tags_select
        .query_map([], |row| {
            Ok(DbTag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .unwrap()
        .collect::<Result<Vec<DbTag>>>()
        .expect("Failed to process existing tags.");

    let distinct = [
        "#e6194b", "#3cb44b", "#ffe119", "#4363d8", "#f58231", "#911eb4", "#46f0f0", "#f032e6",
        "#bcf60c", "#fabebe", "#008080", "#e6beff", "#9a6324", "#fffac8", "#800000", "#aaffc3",
        "#808000", "#ffd8b1", "#000075", "#808080", "#ffffff", "#000000",
    ];

    // Todo: Handle if not enough colors
    let mut not_used_colors: Vec<String> = vec![];
    for color in distinct {
        if !existing_tags.iter().any(|x| x.color == color) {
            not_used_colors.push(color.to_string());
        }
    }

    let mut color_index = 0;
    let mut new_tags: Vec<DbTag> = vec![];
    for expense in expenses.iter() {
        if !existing_tags.iter().any(|x| x.name == expense.tag)
            && !new_tags.iter().any(|x| x.name == expense.tag)
        {
            new_tags.push(DbTag {
                id: 0,
                name: expense.tag.clone(),
                color: not_used_colors[color_index].clone(),
            });
            color_index += 1;
        }
    }

    for tag in new_tags {
        connection
            .execute(
                "INSERT INTO tags (name, color) VALUES (?1, ?2)",
                params![tag.name, tag.color],
            )
            .expect("Table creation failed.");
    }

    let tags: Vec<DbTag> = tags_select
        .query_map([], |row| {
            Ok(DbTag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })
        .unwrap()
        .collect::<Result<Vec<DbTag>>>()
        .expect("Failed to process existing tags.");

    for expense in expenses {
        connection
            .execute(
                "INSERT INTO expenses (date, tag_id, amount) VALUES (?1, ?2, ?3)",
                params![
                    expense.date,
                    tags.iter().find(|x| x.name == expense.tag).unwrap().id,
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

struct DbTag {
    id: u32,
    name: String,
    color: String,
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
