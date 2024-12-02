use std::io::{BufReader, BufRead};
use std::fs::File;

fn part1() {
    let path: &str = "input.txt";
    let input: File = File::open(path).unwrap();
    let buffer: BufReader<File> = BufReader::new(input);

    let mut safe = 0;
    for line in buffer.lines() {
        let l = line.unwrap();
        let iter = l.split_whitespace();
        let mut report: Vec<i32> = Vec::new();

        for i in iter {
            let num = i.parse().expect("invalid number");
            report.push(num);
        }

        let mut valid_dec = true;
        let mut valid_inc = true;
        let mut prev_num = report[0];
        for i in 1..report.len() {
            let num = report[i];

            if prev_num - num < 1 || prev_num - num > 3 {
                valid_dec = false;
            }

            if num - prev_num < 1 || num - prev_num > 3 {
                valid_inc = false;
            }

            prev_num = num;
        }

        safe += if valid_inc || valid_dec {1} else {0};
    }

    println!("{:?}", safe);
}

fn valid_remove(ind: i32, report: &mut Vec<i32>, inc: bool) -> bool {
    if(ind == 0 || ind == report.len()-1) {
        return true;
    }

    if(inc) {
        if(report[ind+1] - report[ind-1] < 1 || report[ind+1] - report[ind-1] > 3) {
            return false;
        }
        return true;
    } else {
        if(report[ind-1] - report[ind+1] < 1 || report[ind-1] - report[ind+1] > 3) {
            return false;
        }
        return true;
    }
}

fn part2() {
    let path: &str = "input.txt";
    let input: File = File::open(path).unwrap();
    let buffer: BufReader<File> = BufReader::new(input);

    let mut safe = 0;
    for line in buffer.lines() {
        let l = line.unwrap();
        let iter = l.split_whitespace();
        let mut report: Vec<i32> = Vec::new();

        for i in iter {
            let num = i.parse().expect("invalid number");
            report.push(num);
        }

        let mut valid_dec = true;
        let mut valid_inc = true;
        let mut prev_num = report[0];
        let mut skip = false;
        for i in 1..report.len() {
            let num = report[i];

            if prev_num - num < 1 || prev_num - num > 3 {
                if skip {
                    valid_dec = false;
                }
            }

            if num - prev_num < 1 || num - prev_num > 3 {
                if skip {
                    valid_inc = false;
                }
            }

            prev_num = num;
        }

        safe += if valid_inc || valid_dec {1} else {0};
    }

    println!("{:?}", safe);
}

fn main() {
    part1();
    // part2();
}