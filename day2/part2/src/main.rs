use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut commands: std::vec::Vec<String> = vec![];

    if let Ok(lines) = read_lines("./commands.txt") {
        for line in lines {
            if let Ok(measurement) = line {
                commands.push(measurement.parse::<String>().unwrap());
            }
        }
    }

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    commands.iter().for_each(|directions| {
        let mut words = directions.split_whitespace();
        let command = words.next().unwrap();
        let value: i32 = std::str::FromStr::from_str(words.next().unwrap()).unwrap();

        if command == "forward" {
            horizontal = horizontal + value;
            depth = depth + (aim * value);
        }
        if command == "up" {
            aim = aim - value;
        }
        if command == "down" {
            aim = aim + value;
        }
    });

    let position = horizontal * depth;

    println!("Position: {}", position);
}
