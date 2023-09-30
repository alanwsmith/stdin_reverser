use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim() == "".to_string() {
            break;
        }
        let reversed = buffer.trim().chars().rev().collect::<String>();
        io::stdout().write_all(reversed.as_bytes())?;
        io::stdout().write_all(b"\n")?;
    }
    Ok(())
}
