use std::fs;

fn main() {
    let calibration_document = fs::read_to_string("day1.txt").expect("Failed to read day1.txt");

    let sum: u32 = calibration_document
        .lines()
        .filter_map(|line| {
            let first_digit = line
                .chars()
                .find(|c| c.is_digit(10))
                .and_then(|c| c.to_digit(10));
            let last_digit = line
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .and_then(|c| c.to_digit(10));
            match (first_digit, last_digit) {
                (Some(first), Some(last)) => Some(first * 10 + last),
                _ => None,
            }
        })
        .sum();

    println!("Sum of calibration values: {}", sum);
}
