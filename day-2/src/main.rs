use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

// this sucks
fn part1(report: &Vec<i32>) -> bool {
    // determine if report is valid decreasing
    let mut valid = true;
    let mut prev_num = report[0];
    for i in 1..report.len() {
        let num = report[i];

        // if invalid decrease, break
        if prev_num - num < 1 || prev_num - num > 3 {
            valid = false;
            break;
        }

        prev_num = num;
    }

    // only need 1 valid condition to be safe
    if valid {
        return true;
    }

    // determine if report is valid increasing
    valid = true;
    prev_num = report[0];
    for i in 1..report.len() {
        let num = report[i];

        // if invalid increase, break
        if num - prev_num < 1 || num - prev_num > 3 {
            valid = false;
            break;
        }

        prev_num = num;
    }

    return valid;
}

// check if removing value at index results in valid report (locally) (idiotically written) (should be called "valid_skip")
fn valid_remove(ind: usize, report: &Vec<i32>, inc: i32) -> bool {
    // check if removing is valid
    if ind > 0 && ind < report.len()-1 &&
      (inc*(report[ind+1] - report[ind-1]) < 1 || 
       inc*(report[ind+1] - report[ind-1]) > 3) {
        return false;
    }

    // can cause problems in next number so must check to solve problem locally
    return !(ind < report.len()-2 &&
            (inc*(report[ind+2] - report[ind+1]) < 1 || 
             inc*(report[ind+2] - report[ind+1]) > 3));
}

// inc is a multiplier signifying whether we're checking for increasing (1) or decreasing (-1)
fn check_valid(report: &Vec<i32>, inc: i32) -> bool {
    // determine if report is valid increasing/decreasing
    let mut skip = false;
    let mut prev_num = report[0];
    for i in 1..report.len() {
        let num = report[i];

        let diff = inc*(num - prev_num);
        if diff < 1 || diff > 3 {
            if skip { // if weve skipped once, we cant skip again
                return false;
            }

            if valid_remove(i-1, &report, inc) { // remove left
                skip = true;
            } else if valid_remove(i, &report, inc) { // remove right
                skip = true;
                continue; // so prev_num isn't updated
            } else { // if no remove works, report is invalid
                return false;
            }
        }

        prev_num = num;
    }

    return true;
}

// awesome, unlike part 1
fn part2(report: &Vec<i32>) -> bool {
    if check_valid(&report, 1) {
        return true;
    }

    return check_valid(&report, -1);
}

fn main() {
    // using cmd args (b/c why not)
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || (args[1] != "--1" && args[1] != "--2") {
        println!("Invalid argument.");
        return;
    }

    // set up file i/o
    let path: &str = "input.txt";
    let input: File = File::open(path).unwrap();
    let buffer: BufReader<File> = BufReader::new(input);

    let mut safe = 0; // number of safe reports
    for line in buffer.lines() {
        let l = line.unwrap();
        let iter = l.split_whitespace();
        let mut report: Vec<i32> = Vec::new();

        for i in iter {
            let num = i.parse().expect("invalid number");
            report.push(num);
        }

        // easy to choose btwn part 1 and part 2
        if args[1] == "--1" {
            safe += if part1(&report) {1} else {0};
        } else if args[1] == "--2" {
            safe += if part2(&report) {1} else {0};
        }
    }

    println!("{:?}", safe);
}