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

fn find_most_common(diagnostics: &std::vec::Vec<String>, index: usize) -> std::vec::Vec<String> {
    let mut most_common_0: std::vec::Vec<String> = vec![];
    let mut most_common_1: std::vec::Vec<String> = vec![];

    diagnostics.iter().for_each(|diagnostic| {
        let digit = diagnostic.as_bytes()[index] as char;

        if digit == '0' {
            most_common_0.push(diagnostic.to_owned());
        } else {
            most_common_1.push(diagnostic.to_owned());
        }
    });

    if most_common_1.len() >= most_common_0.len() {
        most_common_1
    } else {
        most_common_0
    }
}

fn find_least_common(diagnostics: &std::vec::Vec<String>, index: usize) -> std::vec::Vec<String> {
    let mut least_common_0: std::vec::Vec<String> = vec![];
    let mut least_common_1: std::vec::Vec<String> = vec![];

    diagnostics.iter().for_each(|diagnostic| {
        let digit = diagnostic.as_bytes()[index] as char;

        if digit == '0' {
            least_common_0.push(diagnostic.to_owned());
        } else {
            least_common_1.push(diagnostic.to_owned());
        }
    });

    if least_common_1.len() >= least_common_0.len() {
        least_common_0
    } else {
        least_common_1
    }
}

fn calculate_oxygen_generator(diagnostics: std::vec::Vec<String>) -> String {
    let mut oxygen_generator_rating = String::new();
    let mut most_common = vec![];

    for n in 0..=11 {
        if n == 0 {
            most_common = find_most_common(&diagnostics, n);
        } else {
            most_common = find_most_common(&most_common, n);
        }
        if most_common.len() == 1 {
            oxygen_generator_rating = match most_common.first() {
                Some(value) => value.to_owned(),
                None => "".to_owned()
            };
            break;
        }
    }

    oxygen_generator_rating.to_owned()
}

fn calculate_scrubber(diagnostics: &std::vec::Vec<String>) -> String {
    let mut scrubber_rating = String::new();
    let mut least_common = vec![];

    for n in 0..=11 {
        if n == 0 {
            least_common = find_least_common(&diagnostics, n);
        } else {
            least_common = find_least_common(&least_common, n);
        }
        if least_common.len() == 1 {
            scrubber_rating = match least_common.first() {
                Some(value) => value.to_owned(),
                None => "".to_owned()
            };
            break;
        }
    }

    scrubber_rating.to_owned()
}

fn main() {
    let mut diagnostics: std::vec::Vec<String> = vec![];

    if let Ok(lines) = read_lines("./diagnostics.txt") {
        for line in lines {
            if let Ok(measurement) = line {
                diagnostics.push(measurement.parse::<String>().unwrap());
            }
        }
    }

    let oxygen_generator_rating = calculate_oxygen_generator(diagnostics.clone());
    let scrubber_rating = calculate_scrubber(&diagnostics);

    let oxygen = isize::from_str_radix(&oxygen_generator_rating[..], 2).unwrap();
    let scrubber = isize::from_str_radix(&scrubber_rating[..], 2).unwrap();

    let life_support = oxygen * scrubber;

    println!("Life support: {}", life_support);
}
