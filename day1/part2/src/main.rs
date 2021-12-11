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
    let mut measurements = vec![];

    if let Ok(lines) = read_lines("./measurements.txt") {
        for line in lines {
            if let Ok(measurement) = line {
                measurements.push(measurement.parse::<i32>().unwrap());
            }
        }
    }

    let mut depth_increases = 0;
    let mut rolling3: std::vec::Vec<i32> = vec![];

    measurements.windows(3).for_each(|measurement| {
        rolling3.push(measurement.iter().sum());
    });

    rolling3.windows(2).for_each(|measurement| {
        if measurement[0] < measurement[1] {
            depth_increases = depth_increases + 1;
        }
    });

    println!("Depth increases: {}", depth_increases);
}
