use std::env;
use std::fs;

fn part1<'a>(mut input: impl Iterator<Item = &'a str>) -> i32 {
    let mut result: i32 = 0;
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in input.by_ref() {
        let options: Vec<&str> = line.split("   ").collect();
        list1.push(options[0]);
        list2.push(options[1]);
    }

    list1.sort();
    list2.sort();

    for (id1, id2) in list1.iter().zip(list2.iter()) {
        result += id1.parse::<i32>().unwrap().abs_diff(id2.parse::<i32>().unwrap()) as i32;
    }
    result
}

fn part2<'a>(mut input: impl Iterator<Item = &'a str>) -> i32 {
    let mut result: i32 = 0;
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in input.by_ref() {
        let options: Vec<&str> = line.split("   ").collect();
        list1.push(options[0]);
        list2.push(options[1]);
    }

    // list1.sort();
    // list2.sort();

    let mut l1options = vec![];
    let mut l2options = vec![];

    for (id1, id2) in list1.iter().zip(list2.iter()) {
        let mut index = l1options.iter().position(|(&x,_)| x == *id1);
        if index == None {
            l1options.push((id1,1));
        }
        else {
            l1options[index.unwrap()].1 += 1;
        }

        index = l2options.iter().position(|(&x,_)| x == *id2);
        if index == None {
            l2options.push((id2,1));
        }
        else {
            l2options[index.unwrap()].1 += 1;
        }
    }
    
    for id1 in l1options.iter() {
        let index = l2options.iter().position(|(&x,_)| x == *id1.0);
        if index == None {
            continue;
        }
        else {
            result += id1.0.parse::<i32>().unwrap() * id1.1 * l2options[index.unwrap()].1 as i32;
        }
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
