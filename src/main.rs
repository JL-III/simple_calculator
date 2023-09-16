use crate::io::stdout;
use std::io;
use std::io::Stdin;
use std::io::Write;
use std::process::exit;

fn main() -> io::Result<()> {
    let _ = stdout().flush()?;
    let stdin = io::stdin();
    let _ = validate_arguments(&get_three_arguments(stdin));
    Ok(())
}

fn get_three_arguments(stdin: Stdin) -> String {
    let mut buffer = String::new();
    while &buffer.split_whitespace().collect::<Vec<_>>().len() != &3 {
        println!("\nEnter 3 arguments.\nExample: 8 * 3");
        print!("> ");
        let _ = stdout().flush();

        buffer = String::new();
        stdin.read_line(&mut buffer);
    }
    buffer
}

fn validate_arguments(s: &str) -> Vec<&str> {
    let mut args: Vec<&str> = s.split_whitespace().collect();
    if is_parsable_to_i32(args[0])
        && is_mathematical_operator(args[1])
        && is_parsable_to_i32(args[2])
    {
        print!("Valid inputs!\n");
    } else {
        print!("Please enter a mathematical expression\nExample: 8 * 3\n");
        exit(1)
    }
    args
}

fn is_parsable_to_i32(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

fn is_mathematical_operator(s: &str) -> bool {
    let vec_of_operators: Vec<&str> = vec!["*", "/", "+", "-"];
    if vec_of_operators.contains(&s) {
        true
    } else {
        false
    }
}
