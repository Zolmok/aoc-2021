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

fn calculate_gamma(diagnostics: &std::vec::Vec<String>) -> String {
    let mut counts: [(i32, i32); 12] = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
    let mut gamma = String::new();

    diagnostics.iter().for_each(|diagnostic| {
        let mut digits = diagnostic.chars();

        for n in 0..=11 {
            let digit = digits.next().unwrap();

            if digit == '0' {
                counts[n].0 = counts[n].0 + 1;
            } else {
                counts[n].1 = counts[n].1 + 1;
            }
        }
    });

    for n in 0..=11 {
        if counts[n].0 > counts[n].1 {
            gamma.push('0');
        } else {
            gamma.push('1');
        }
    }

    gamma
}

fn calculate_epsilon(gamma: &String) -> String {
    let mut epsilon = String::new();
    let mut digits = gamma.chars();

    for _ in 0..=11 {
        let digit = digits.next().unwrap();

        if digit == '0' {
            epsilon.push('1');
        } else {
            epsilon.push('0');
        }
    }

    epsilon
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

    let gamma_rate = calculate_gamma(&diagnostics);
    let epsilon_rate = calculate_epsilon(&gamma_rate);

    let gamma = isize::from_str_radix(&gamma_rate[..], 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_rate[..], 2).unwrap();

    let power_consumption = gamma * epsilon;

    println!("Power consumption: {}", power_consumption);
}
