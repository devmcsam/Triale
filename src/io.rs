use crate::errors::AppError;
use crate::point::{to_point, Point};
use std::io;

pub fn get_input(prompt: &str) -> Result<String, io::Error> {
    print!("{prompt}");
    io::Write::flush(&mut io::stdout())?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.is_empty() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"));
    }
    Ok(input.trim().to_string())
}

pub fn read_point_with_retries(label: &str) -> Result<Point, AppError> {
    loop {
        let input = match get_input(&format!("Enter point {label} (x, y): ")) {
            Ok(val) => val,
            Err(e) => return Err(AppError::Io(e)),
        };

        match to_point(&input) {
            Ok(pt) => return Ok(pt),
            Err(e) => {
                println!("  ⚠  {e}");
                println!("     Please try again using the format 'x, y' (e.g. 1.0, 2.0)");
            }
        }
    }
}
