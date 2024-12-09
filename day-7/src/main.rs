// ok !!

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

// helper function to recursively try every operator
fn evaluate1(target: i64, operands: &Vec<i64>, res: i64, loc: usize) -> bool {
    if loc == operands.len() {
        return res == target;
    }

    if res > target {
        return false;
    }

    let prod = evaluate1(target, operands, res*operands[loc], loc+1);
    let sum = evaluate1(target, operands, res+operands[loc], loc+1);
    return prod || sum;
}

fn part1(target: i64, operands: &Vec<i64>) -> bool {
    return evaluate1(target, operands, operands[0], 1);
}

fn concat(first: i64, second: i64) -> i64 {
    // log10 of second gives the number of digits in second, which we append to first to leave space to fit second in
    return first * i64::pow(10, second.ilog10()+1) + second;
}

// helper function to recursively try every operator (with concat)
fn evaluate2(target: i64, operands: &Vec<i64>, res: i64, loc: usize) -> bool {
    if loc == operands.len() {
        return res == target;
    }

    if res > target {
        return false;
    }

    let prod = evaluate2(target, operands, res*operands[loc], loc+1);
    let sum = evaluate2(target, operands, res+operands[loc], loc+1);
    let large = evaluate2(target, operands, concat(res, operands[loc]), loc+1);
    return prod || sum || large;

    // i thought concat was performed out of order, but its not, so ignore the below code. i think its cool so im keeping it lmao!

    // // combines both numbers into one
    // let large = concat(operands[loc-1], operands[loc]);
    // // println!("{}", large);
    // if prev == 0 {
    //     res /= operands[loc-1];
    //     if evaluate2(target, operands, res*large, loc+1, 0) {return true};
    // } else {
    //     res -= operands[loc-1];
    //     if evaluate2(target, operands, res+large, loc+1, 1) {return true};
    // }

    // return false;
}

fn part2(target: i64, operands: &Vec<i64>) -> bool {
    return evaluate2(target, operands, operands[0], 1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || (args[1] != "--1" && args[1] != "--2") {
        println!("Invalid argument.");
        return;
    }

    let path: &str = "input.txt";
    let input: File = File::open(path).unwrap();
    let buffer: BufReader<File> = BufReader::new(input);

    let mut sum = 0;
    for line in buffer.lines() {
        let text = line.unwrap();
        let row: Vec<&str> = text.split(": ").collect();

        let target = row[0].parse::<i64>().unwrap();
        // how it feels to write javascript
        let operands: Vec<i64> = row[1].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();

        let viable = if args[1] == "--1" {part1(target, &operands)} else {part2(target, &operands)};
        if viable {
            sum += target;
        }
    }

    println!("{:?}", sum);
}