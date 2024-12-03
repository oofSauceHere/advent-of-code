// i hate regex #happyholidays

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

extern crate regex; // ok
use regex::Regex;

fn part1(text: String) -> i32 {
    let mut sum = 0;

    // match all mul commands
    let mul_re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    for cmd in mul_re.find_iter(text.as_str()).map(|m| m.as_str()) {
        // match all digits
        let digit_re = Regex::new(r"\d+").unwrap();

        // multiply all digits (of which there will only be 2...)
        let mut prod = 1;
        for num in digit_re.find_iter(cmd).map(|m| m.as_str().parse::<i32>().unwrap()) {
            prod *= num;
        }
        
        sum += prod;
    }

    return sum;
}

fn part2(text: String) -> i32 {
    let mut sum = 0;

    // match all text between do() and don't() (or from start to don't() or do() to end)
    let re = Regex::new(r"(do\(\)|^)(.*?)(don't\(\)|$)").unwrap();
    for stretch in re.find_iter(text.as_str()).map(|m| m.as_str()) {
        // match all mul commands
        let mul_re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        for cmd in mul_re.find_iter(stretch).map(|m| m.as_str()) {
            // match all digits
            let digit_re = Regex::new(r"\d+").unwrap();

            // multiply all digits (of which there will only be 2...)
            let mut prod = 1;
            for num in digit_re.find_iter(cmd).map(|m| m.as_str().parse::<i32>().unwrap()) {
                prod *= num;
            }

            sum += prod;
        }
    }

    return sum;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || (args[1] != "--1" && args[1] != "--2") {
        println!("Invalid argument.");
        return;
    }

    let path: &str = "input.txt";
    let input: File = File::open(path).expect("couldnt open file");
    let buffer: BufReader<File> = BufReader::new(input);

    // its necessary to put everything on one line because a dont() command could end one line,
    // meaning no mul() at the start of the next line should be executed. the regex doesn't know
    // this and will eat shit and die
    let mut instruction:String = String::new();
    for line in buffer.lines() {
        let text = line.unwrap();
        instruction = format!("{}{}", instruction, text); // low key evil method of string concatenation
    }

    let sum = if args[1] == "--1" {part1(instruction)} else {part2(instruction)};
    println!("{:?}", sum);
}