mod storage;
use std::fs;

use chrono::{NaiveDate, NaiveTime};
use storage::{DbStorage, Expense, Storage, StoredTag, Tag};

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

    let existing_tags = storage
        .get_tags()
        .expect("Failed to retrieve existing tags.");

    let new_tags = find_new_tags(&expenses, &existing_tags);

    storage.save_tags(&new_tags);

    storage.save_expenses(&expenses);

    storage.close();
}

fn available_colors(tags: &[StoredTag]) -> Vec<String> {
    let distinct = [
        "#e6194b", "#3cb44b", "#ffe119", "#4363d8", "#f58231", "#911eb4", "#46f0f0", "#f032e6",
        "#bcf60c", "#fabebe", "#008080", "#e6beff", "#9a6324", "#fffac8", "#800000", "#aaffc3",
        "#808000", "#ffd8b1", "#000075", "#808080", "#ffffff", "#000000",
    ];

    // Todo: Handle if not enough colors
    let mut not_used_colors: Vec<String> = vec![];
    for color in distinct {
        if !tags.iter().any(|x| x.color == color) {
            not_used_colors.push(color.to_string());
        }
    }

    not_used_colors
}

fn find_new_tags(expenses: &Vec<Expense>, tags: &[StoredTag]) -> Vec<Tag> {
    let not_used_colors = available_colors(tags);
    let mut color_index = 0;
    let mut new_tags: Vec<Tag> = vec![];
    for expense in expenses {
        if !tags.iter().any(|x| x.name == expense.tag)
            && !new_tags.iter().any(|x| x.name == expense.tag)
        {
            new_tags.push(Tag {
                name: expense.tag.clone(),
                color: not_used_colors[color_index].clone(),
            });
            color_index += 1;
        }
    }

    new_tags
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
