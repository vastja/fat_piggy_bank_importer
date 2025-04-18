use std::fs;

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rusqlite::{params, Connection, Result};

struct DbStorage {
    con: Connection,
}

impl DbStorage {
    pub fn new() -> Self {
        let con = Connection::open("./fat_piggy_bank.db").expect("Database connection failed.");
        let loc_self = DbStorage { con };
        loc_self.create_expenses_table();
        loc_self.create_tags_table();
        loc_self
    }

    fn create_expenses_table(&self) {
        self.con
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
    }

    fn create_tags_table(&self) {
        self.con
            .execute(
                "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                color TEXT NOT NULL
            )",
                (),
            )
            .expect("Tags table creation failed.");
    }

    pub fn close(self) {
        self.con.close().expect("Failed to close connection.");
    }
}

impl Storage for DbStorage {
    fn get_tags(&self) -> Result<Vec<StoredTag>> {
        let mut select = self
            .con
            .prepare("SELECT * FROM tags")
            .expect("Selecting tags failed.");

        select
            .query_map([], |row| {
                Ok(StoredTag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                })
            })
            .unwrap()
            .collect()
    }

    fn save_tags(&self, tags: Vec<Tag>) {
        for tag in tags {
            self.con
                .execute(
                    "INSERT INTO tags (name, color) VALUES (?1, ?2)",
                    params![tag.name, tag.color],
                )
                .expect("Table creation failed.");
        }
    }

    fn save_expenses(&self, expenses: Vec<Expense>) {
        let tags = self.get_tags().expect("Failed to retrieve tags.");
        for expense in expenses {
            self.con
                .execute(
                    "INSERT INTO expenses (date, tag_id, amount) VALUES (?1, ?2, ?3)",
                    params![
                        expense.date,
                        tags.iter()
                            .find(|x| x.name == expense.tag)
                            .expect("Invalid expense tag name.")
                            .id,
                        expense.amount.to_string(),
                    ],
                )
                .expect("Table creation failed.");
        }
    }
}

trait Storage {
    fn get_tags(&self) -> Result<Vec<StoredTag>>;
    fn save_tags(&self, tags: Vec<Tag>); // Todo: Return result
    fn save_expenses(&self, expenses: Vec<Expense>); // Todo: Return result
}

fn main() {
    let storage = DbStorage::new();
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

    let distinct = [
        "#e6194b", "#3cb44b", "#ffe119", "#4363d8", "#f58231", "#911eb4", "#46f0f0", "#f032e6",
        "#bcf60c", "#fabebe", "#008080", "#e6beff", "#9a6324", "#fffac8", "#800000", "#aaffc3",
        "#808000", "#ffd8b1", "#000075", "#808080", "#ffffff", "#000000",
    ];

    let existing_tags = storage
        .get_tags()
        .expect("Failed to retrieve existing tags.");

    // Todo: Handle if not enough colors
    let mut not_used_colors: Vec<String> = vec![];
    for color in distinct {
        if !existing_tags.iter().any(|x| x.color == color) {
            not_used_colors.push(color.to_string());
        }
    }

    let mut color_index = 0;
    let mut new_tags: Vec<Tag> = vec![];
    for expense in expenses.iter() {
        if !existing_tags.iter().any(|x| x.name == expense.tag)
            && !new_tags.iter().any(|x| x.name == expense.tag)
        {
            new_tags.push(Tag {
                name: expense.tag.clone(),
                color: not_used_colors[color_index].clone(),
            });
            color_index += 1;
        }
    }

    storage.save_tags(new_tags);

    storage.save_expenses(expenses);

    storage.close();
}

struct Expense {
    date: DateTime<Utc>,
    tag: String,
    amount: i32,
}

struct Tag {
    name: String,
    color: String,
}

struct StoredTag {
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
