use std::io::{self, Write};

fn ask(question: &str) {
    print!("{question}");
    io::stdout().flush().unwrap();
}

pub fn input() -> String {
    let mut result: String = String::new();

    io::stdin().read_line(&mut result).unwrap();

    return result.trim().to_string();
}

pub fn number(question: &str) -> u32 {
    ask(question);

    let string = input();

    return string.trim().parse::<u32>().unwrap();
}

pub fn chars(question: &str) -> Vec<char> {
    ask(question);

    let string = input();

    return string.chars().collect();
}

pub fn progress_bar(progress: f64, width: usize) -> String {
    let filled = (progress * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    let filled_bar = "=".repeat(filled);
    let pointer = if filled < width {
        ">".to_string()
    } else {
        "".to_string()
    };
    let spaces = " ".repeat(empty.saturating_sub(pointer.len()));

    let percent = format!("{:3.0}%", progress * 100.0);

    format!(
        "{}{}{}{}{} {}",
        "[", filled_bar, pointer, spaces, "]", percent,
    )
}
