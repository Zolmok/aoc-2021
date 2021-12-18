extern crate aoc;

#[derive(Debug, Clone)]
struct Bingo {
    board: Vec<Vec<i32>>,
    x: usize,
    y: usize,
    winner: bool
}

impl Bingo {
    fn new(width: usize, height: usize) -> Self {
        let mut board = Vec::with_capacity(height);
        let x = width;
        let y = height;
        let winner = false;

        for _ in 0..height {
            board.push([i32::default()].repeat(width));
        }

        Self { board, x, y, winner }
    }

    fn set(&mut self, x: usize, values: Vec<i32>) {
        self.board[x] = values;
    }

    fn get(&self, x: usize, y: usize) -> &i32 {
        &self.board[x][y]
    }

    fn clear(&mut self) {
        for y in 0..self.y {
            for x in 0..self.x {
                self.board[y][x] = i32::default();
            }
        }
    }

    fn check_win(&mut self, numbers: Vec<i32>) -> Result<bool, bool> {
        let mut winner = Err(false);
        let mut matches: Vec<(usize, usize)> = vec![];

        if self.winner {
            return winner;
        }
        if numbers.len() < 5 {
            return winner;
        }

        numbers.iter().for_each(|number| {
            for y in 0..self.y {
                for x in 0..self.x {
                    if winner == Ok(true) {
                       break; 
                    }
                    if self.board[y][x] == *number {
                        matches.push((y, x));

                        // do we have an x winner?
                        for x in 0..self.x {
                            let found = matches.iter().filter(|&&coord| coord.0 == x).count();

                            if found == self.x {
                                self.winner = true;
                                winner = Ok(true);
                            }
                        }
                        // do we have an y winner?
                        for y in 0..self.y {
                            let found = matches.iter().filter(|&&coord| coord.1 == y).count();

                            if found == self.y {
                                self.winner = true;
                                winner = Ok(true);
                            }
                        }
                    }
                }
            }
        });

        winner
    }
}

fn part1(cards: &mut Vec<Bingo>, input: Vec<i32>) -> Result<i32, String> {
    let mut score: i32 = 0;
    let mut sum_unmarked: i32 = 0;
    let mut next_number: Vec<i32> = vec![];

    // call the numbers out
    for number in 0..input.len() {
        let mut winner = Err(false);

        next_number.push(input[number]);

        if number < 4 {
            // no winners until at least 5 numbers called
            continue;
        }

        // check cards for winners
        cards.iter_mut().try_for_each(|card| {
            winner = card.check_win(next_number.to_vec());

            if winner == Ok(true) {
                // winner found, calculate score
                let mut unmarked: Vec<i32> = vec![];

                // find all the unmarked numbers
                for row in 0..card.y {
                    for col in 0..card.x {
                        let val = card.get(row, col);

                        if next_number.contains(val) == false {
                            unmarked.push(*val);                           
                        }
                    }
                }

                // sum the unmarked numbers
                sum_unmarked = unmarked.iter().sum();

                // find the last number that was called
                let last_num_called = match next_number.last() {
                    Some(val) => val,
                    None => &0
                };

                // score is the sum of all the unmarked numbers * the last number called
                score = sum_unmarked * last_num_called;

                return std::ops::ControlFlow::Break(score);
            }

            std::ops::ControlFlow::Continue(())
        });

        if winner == Ok(true) {
            break;
        }
    }

    Ok(score)
}

fn part2(cards: &mut Vec<Bingo>, input: Vec<i32>) -> Result<i32, String> {
    let mut next_number: Vec<i32> = vec![];
    let mut unmarked: Vec<i32> = vec![];
    let mut last_called: Vec<Vec<i32>> = vec![];
    let score: i32;
    let sum_unmarked: i32;
    let winners: &mut Vec<Bingo> = &mut vec![];

    // call the numbers out
    for number in 0..input.len() {
        let mut winner = Err(false);

        next_number.push(input[number]);

        // check cards for winners
        cards.iter_mut().for_each(|card| {
            winner = card.check_win(next_number.to_vec());

            if winner == Ok(true) {
                winners.push(card.to_owned());
                last_called.push(next_number.to_owned());
            }
        });
    }

    // winner found, calculate score
    let last_winner = match winners.last() {
        Some(value) => value,
        None => panic!("Error getting last winner")
    };
    let last_called_winner: Vec<i32> = last_called[winners.len() - 1].clone();

    // find all the unmarked numbers
    for row in 0..last_winner.y {
        for col in 0..last_winner.x {
            let val = last_winner.get(row, col);

            if last_called_winner.contains(val) == false {
                unmarked.push(*val);                           
            }
        }
    }

    // sum the unmarked numbers
    sum_unmarked = unmarked.iter().sum();

    // find the last number that was called
    let last_num_called = match last_called_winner.last() {
        Some(val) => val,
        None => &0
    };

    // score is the sum of all the unmarked numbers * the last number called
    score = sum_unmarked * last_num_called;

    Ok(score)
}

fn main() {
    let mut card: Bingo = Bingo::new(5, 5);
    let cards: &mut Vec<Bingo> = &mut vec![];
    let mut bingo_numbers: Vec<i32> = vec![];
    let mut row_count: usize = 0;
    let mut line_count: usize = 0;

    if let Ok(lines) = aoc::read_lines("./input.txt") {
        for line in lines {
            if line_count == 0 {
                // first line is the bingo numbers
                if let Ok(number_batch) = &line {
                    bingo_numbers = number_batch.split(",")
                        .map(|input| input.parse::<i32>().unwrap())
                        .collect();
                }
            } else {
                // the rest of the lines are the card numbers
                if let Ok(card_line) = &line {
                    if card_line == "" {
                        // we got a new card
                        if line_count > 1 {
                            cards.push(card.to_owned());
                        }
                        card.clear();
                        row_count = 0;
                    } else {
                        // build the card
                        let card_numbers: Vec<i32> = card_line
                            .split_whitespace()
                            .map(|input| input.parse::<i32>().unwrap())
                            .collect();
                        card.set(row_count, card_numbers);
                        row_count = row_count + 1;
                    }
                }
            }

            line_count = line_count + 1;
        }
    }

    let score = match part1(cards, bingo_numbers.to_vec()) {
        Ok(value) => value,
        Err(error) => {
            println!("No winner found: {}", error);

            0
        }
    };

    println!("Score: {}", score);

    let score = match part2(cards, bingo_numbers.to_vec()) {
        Ok(value) => value,
        Err(error) => {
            println!("No winner found: {}", error);

            0
        }
    };

    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_works() {
        let mut card = Bingo::new(5, 5);

        card.set(0, [23, 14, 18, 12, 1].to_vec());
        card.set(1, [9, 3, 24, 5, 25].to_vec());
        card.set(2, [22, 10, 15, 17, 8].to_vec());

        let value0 = card.get(0, 0);
        let value1 = card.get(1, 1);
        let value2 = card.get(2, 2);

        assert_eq!(value0, &23);
        assert_eq!(value1, &3);
        assert_eq!(value2, &15);
    }

    #[test]
    fn check_win_horizontal() {
        let mut card = Bingo::new(5, 5);
        let numbers: Vec<i32> = [14, 9, 21, 16, 23, 19, 34, 7].to_vec();

        card.set(0, [22, 13, 17, 11,  0].to_vec());
        card.set(1, [ 8,  2, 23,  4, 24].to_vec());
        card.set(2, [21,  9, 14, 16,  7].to_vec());
        card.set(3, [ 6, 10,  3, 18,  5].to_vec());
        card.set(4, [ 1, 12, 20, 15, 19].to_vec());

        let winner = match card.check_win(numbers.to_vec()) {
            Ok(value) => value,
            Err(_error) => false
        };

        assert!(winner, "No horizontal winner")
    }

    #[test]
    fn check_win_vertical() {
        let mut card = Bingo::new(5, 5);
        let numbers = [23, 17, 14, 3, 11, 19, 6, 16, 2, 20];

        card.set(0, [22, 13, 17, 11,  0].to_vec());
        card.set(1, [ 8,  2, 23,  4, 24].to_vec());
        card.set(2, [21,  9, 14, 16,  7].to_vec());
        card.set(3, [ 6, 10,  3, 18,  5].to_vec());
        card.set(4, [ 1, 12, 20, 15, 19].to_vec());

        let winner = match card.check_win(numbers.to_vec()) {
            Ok(value) => value,
            Err(_error) => false
        };

        assert!(winner, "No vertical winner")
    }

    #[test]
    fn bingo() {
        let answer = 4512;
        let input = [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1];

        let mut card1: Bingo = Bingo::new(5, 5);
        let mut card2: Bingo = Bingo::new(5, 5);
        let mut card3: Bingo = Bingo::new(5, 5);

        card1.set(0, [22, 13, 17, 11,  0].to_vec());
        card1.set(1, [ 8,  2, 23,  4, 24].to_vec());
        card1.set(2, [21,  9, 14, 16,  7].to_vec());
        card1.set(3, [ 6, 10,  3, 18,  5].to_vec());
        card1.set(4, [ 1, 12, 20, 15, 19].to_vec());

        card2.set(0, [ 3, 15,  0,  2, 22].to_vec());
        card2.set(1, [ 9, 18, 13, 17,  5].to_vec());
        card2.set(2, [19,  8,  7, 25, 23].to_vec());
        card2.set(3, [20, 11, 10, 24,  4].to_vec());
        card2.set(4, [14, 21, 16, 12,  6].to_vec());

        card3.set(0, [14, 21, 17, 24,  4].to_vec());
        card3.set(1, [10, 16, 15,  9, 19].to_vec());
        card3.set(2, [18,  8, 23, 26, 20].to_vec());
        card3.set(3, [22, 11, 13,  6,  5].to_vec());
        card3.set(4, [ 2,  0, 12,  3,  7].to_vec());

        let cards: &mut Vec<Bingo> = &mut vec![card1, card2, card3];

        let score = match part1(cards, input.to_vec()) {
            Ok(value) => value,
            Err(error) => {
                println!("No winner found: {}", error);

                0
            }
        };

        assert_eq!(score, answer);
    }
}
