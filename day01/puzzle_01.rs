use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Open the file in read-only mode
    let file = File::open("inputs/real.txt")?;

    // Create a buffered reader to efficiently read lines
    let reader = io::BufReader::new(file);

    // Iterate over the lines using lines() method
    let mut res_sum: i32 = 0;
    for line in reader.lines() {
        // Handle each line here
        match line {
            Ok(content) => {
                match find_first_and_last_numbers(&content) {
                    Ok(parsed_number) => {
                        //println!("Successfully parsed: {}", parsed_number);
                        res_sum += parsed_number;
                        // Now you can use 'parsed_number' as an integer in your code
                    }
                    Err(err) => {
                        eprintln!("Error parsing the string: {}", err);
            
                        // You may choose to provide a default value or handle the error in some way
                    }
                }
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    println!("result is {}", res_sum);

    Ok(())
}


fn find_first_and_last_numbers(line: &str) -> Result<i32, std::num::ParseIntError> {
    let first_digit = line.chars().find(|c| c.is_digit(10));
    let last_digit = line.chars().rev().find(|c| c.is_digit(10));
    let combined: String = match (first_digit, last_digit) {
        (Some(first), Some(last)) => first.to_string() + &last.to_string(),
        _ => String::new(),
    };
    let parsed_value: i32 = combined.parse()?;
    Ok(parsed_value)
}
