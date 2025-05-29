use std::io::{Write, stdin};

pub fn prompt_user(prompt: &str) -> String {
    println!("{}: ", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();
    if input.is_empty() {
        println!("Input cannot be empty");
        return prompt_user(prompt);
    }
    input
}

pub fn prompt_user_with_default(prompt: &str, default: &str) -> String {
    println!(
        "{} (\x1b[2mPress Enter to use default: {}\x1b[0m):",
        prompt, default
    );
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() {
        default.to_string()
    } else {
        input.to_string()
    }
}
