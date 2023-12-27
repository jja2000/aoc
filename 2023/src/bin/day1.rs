use std::env;
use std::fs;

fn part1<'a>(mut input: impl Iterator<Item = &'a str>) -> i32 {
    let mut result: i32 = 0;
    for line in input.by_ref() {
        let mut number = String::new();
        for c in line.chars() {
            if char::is_numeric(c) {
                number.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if char::is_numeric(c) {
                number.push(c);
                break;
            }
        }
        result += number.clone().parse::<i32>().unwrap();
    }
    result
}

fn part2<'a>(mut input: impl Iterator<Item = &'a str>) -> i32 {
    let options = vec!["0","1","2","3","4","5","6","7","8","9","zero","one","two","three","four","five","six","seven","eight","nine"];
    let mut result: i32 = 0;
    for line in input.by_ref() {
        // Executed per line of input
        let mut low: i32 = -1;
        let mut high: i32 = -1;
        let mut low_string = 0;
        let mut high_string = 0;
        let options_iter = options.iter();
        for (i, option) in options_iter.enumerate() {
            let test = line.find(option);
            if test.is_none() {
                continue;
            }
            else {
                let low_test = line.find(option).unwrap() as i32;
                let high_test = line.rfind(option).unwrap() as i32;
                if low == -1 {
                    low = low_test;
                    low_string = i;
                }
                else if low > low_test {
                    low = low_test;
                    low_string = i;
                }
    
                if high == -1 {
                    high = high_test;
                    high_string = i;
                }
                else if high < high_test {
                    high = high_test;
                    high_string = i;
                }
            }
        }

        if low_string >= 10 {
            low_string -= 10;
        }

        if high_string >= 10 {
            high_string -= 10;
        }
        let number: String = options[low_string].to_owned() + options[high_string];
        result += number.parse::<i32>().unwrap();
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
