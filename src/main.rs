use std::io;
use crate::io::stdout;
use std::io::Write;

fn main() -> io::Result<()> {
    print!("Enter an expression: ");
    let _ = stdout().flush()?;

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    print!("You entered {}", &mut buffer);
    Ok(())
}
