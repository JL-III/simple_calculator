fn main() -> io:Result<()> {
    println!("Hello, world!");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(())
}
