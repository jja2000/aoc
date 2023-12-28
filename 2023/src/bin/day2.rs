use std::env;
use std::fs;

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red_amount: usize,
    green_amount: usize,
    blue_amount: usize,
}

fn part1<'a>(mut input: impl Iterator<Item = &'a str>) -> i32 {
    let mut result: i32 = 0;
    let max_red: usize = 12;
    let max_green: usize = 13;
    let max_blue: usize = 14;

    
    'line: for line in input.by_ref() {
        let split_line = line.split(' ');
        let element_string = split_line.collect::<String>();
        let mut split = element_string.split(':');

        // Parse game info
        let game_string = split.next().unwrap();
        let mut game = Game { id: 0, sets: Vec::new()};
        let mut gameid: String = Default::default();
        for c in game_string.chars().rev() {
            if c.is_numeric() {
                //EEEEEEEEEEEEEEEEEEEW
                gameid = c.to_string() + &gameid;
            }
            else {
                game.id = gameid.clone().parse::<usize>().unwrap();
                break;
            }
        }

        // Get sets
        let game_result_string = split.next().unwrap();
        let new_sets = game_result_string.split(';');
        for set in new_sets {
            // Split set data
            let set_data = set.split(',');
            //Run through data, set up sets
            let mut new_set = Set { red_amount: 0, green_amount: 0, blue_amount: 0};
            for data in set_data {
                let mut block_color: String = Default::default();
                let mut block_amount: String = Default::default();
                for c in data.chars() {
                    if c.is_numeric() {
                        block_amount.push(c);
                    }
                    else if c.is_alphabetic() {
                        block_color.push(c);
                    }
                }

                match block_color.as_ref() {
                    "red" => new_set.red_amount = block_amount.clone().parse::<usize>().unwrap(),
                    "green" => new_set.green_amount = block_amount.clone().parse::<usize>().unwrap(),
                    "blue" => new_set.blue_amount = block_amount.clone().parse::<usize>().unwrap(),
                    _ => println!("Something went wrong"),
                }
            }
            game.sets.push(new_set);
        }
        
        for set in &game.sets {
            if set.red_amount > max_red || set.green_amount > max_green || set.blue_amount > max_blue {
                continue 'line;
            }
        }

        result += game.id as i32;
    }
    result
}

fn part2<'a>(mut input: impl Iterator<Item = &'a str>) -> i32 {
    let mut result: i32 = 0;
    
    for line in input.by_ref() {
        let mut min_red: usize = Default::default();
        let mut min_green: usize = Default::default();
        let mut min_blue: usize = Default::default();
        let split_line = line.split(' ');
        let element_string = split_line.collect::<String>();
        let mut split = element_string.split(':');

        // Parse game info
        let game_string = split.next().unwrap();
        let mut game = Game { id: 0, sets: Vec::new()};
        let mut gameid: String = Default::default();
        for c in game_string.chars().rev() {
            if c.is_numeric() {
                //As I said before:
                //EEEEEEEEEEEEEEEEEEEEEEEEEEW
                gameid = c.to_string() + &gameid;
            }
            else {
                game.id = gameid.clone().parse::<usize>().unwrap();
                break;
            }
        }

        // Get sets
        let game_result_string = split.next().unwrap();
        let new_sets = game_result_string.split(';');
        for set in new_sets {
            // Split set data
            let set_data = set.split(',');
            //Run through data, set up sets
            let mut new_set = Set { red_amount: 0, green_amount: 0, blue_amount: 0};
            for data in set_data {
                let mut block_color: String = Default::default();
                let mut block_amount: String = Default::default();
                for c in data.chars() {
                    if c.is_numeric() {
                        block_amount.push(c);
                    }
                    else if c.is_alphabetic() {
                        block_color.push(c);
                    }
                }

                match block_color.as_ref() {
                    "red" => new_set.red_amount = block_amount.clone().parse::<usize>().unwrap(),
                    "green" => new_set.green_amount = block_amount.clone().parse::<usize>().unwrap(),
                    "blue" => new_set.blue_amount = block_amount.clone().parse::<usize>().unwrap(),
                    _ => println!("Something went wrong"),
                }
            }
            game.sets.push(new_set);
        }
        
        for set in &game.sets {
            if set.red_amount > min_red {
                min_red = set.red_amount;
            }

            if set.green_amount > min_green {
                min_green = set.green_amount;
            }

            if set.blue_amount > min_blue {
                min_blue = set.blue_amount;
            }
        }

        result += min_red as i32 * min_green as i32 * min_blue as i32;
    }
    result
}

fn main() {
    let file_path = env::args().nth(2).expect("Please enter a path to the input file!");
    let binding = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let input = binding.lines();
    let mut result: i32 = 0;
    if env::args().nth(1).expect("Please enter a puzzle part number").parse::<i32>().unwrap() == 1 {
        result = part1(input);
    }
    else if env::args().nth(1).expect("Please enter a puzzle part number").parse::<i32>().unwrap() == 2 {
        result = part2(input);
    }
    else {

    }
    println!("Answer: {}", result);
}
