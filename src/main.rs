use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read command");

    let command = input.trim();

    match command {
        "init" => println!("Creating table ..."),
        _ => println!("{} command not recognized.", command),
    }
}
