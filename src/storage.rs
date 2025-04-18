use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};

pub struct DbStorage {
    con: Connection,
}

pub trait Storage {
    fn get_tags(&self) -> Result<Vec<StoredTag>>;
    fn save_tags(&self, tags: &Vec<Tag>); // Todo: Return result
    fn save_expenses(&self, expenses: &Vec<Expense>); // Todo: Return result
}

pub struct Expense {
    pub date: DateTime<Utc>,
    pub tag: String,
    pub amount: i32,
}

pub struct Tag {
    pub name: String,
    pub color: String,
}

pub struct StoredTag {
    pub id: u32,
    pub name: String,
    pub color: String,
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

    fn save_tags(&self, tags: &Vec<Tag>) {
        for tag in tags {
            self.con
                .execute(
                    "INSERT INTO tags (name, color) VALUES (?1, ?2)",
                    params![tag.name, tag.color],
                )
                .expect("Table creation failed.");
        }
    }

    fn save_expenses(&self, expenses: &Vec<Expense>) {
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
