use std::env;
use std::fs;
use std::str::FromStr;

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
        let mut number = String::new();
        let mut low: i32 = -1;
        let mut high: i32 = -1;
        let mut lowString = 0;
        let mut highString = 0;
        let mut optionsIter = options.iter();
        for (i, option) in optionsIter.enumerate() {
            let test = line.find(option);
            if test.is_none() {
                continue;
            }
            else {
                let lowTest = line.find(option).unwrap() as i32;
                let highTest = line.rfind(option).unwrap() as i32;
                if low == -1 {
                    low = lowTest;
                    lowString = i;
                }
                else if low > lowTest {
                    low = lowTest;
                    lowString = i;
                }
    
                if high == -1 {
                    high = highTest;
                    highString = i;
                }
                else if high < highTest {
                    high = highTest;
                    highString = i;
                }
            }
        }

        if lowString >= 10 {
            lowString -= 10;
        }

        if highString >= 10 {
            highString -= 10;
        }
        number = options[lowString].to_owned() + options[highString];
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
