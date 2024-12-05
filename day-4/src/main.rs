// getting the hang of it

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

fn part1(crossword: &Vec<Vec<char>>) -> i32 {
    let mut cnt = 0;
    let n = crossword.len();
    let m = crossword[0].len();

    // iterate through every space in the crossword
    // (i considered finding every X and then doing a radial search for
    // any XMAS matches, but given the density of Xs in the crossword
    // i'm not sure it'll bring much of a performance increase)
    for i in 0..n {
        for j in 0..m {
            let mut word1 = String::new();
            let mut word2 = String::new();
            let mut word3 = String::new();
            let mut word4 = String::new();
            for k in 0..4 {
                if i+3 < n {word1.push(crossword[i+k][j])}; // vertical
                if j+3 < m {word2.push(crossword[i][j+k])}; // horizontal
                if i+3 < n && j+3 < m {
                    word3.push(crossword[i+k][j+k]);
                    word4.push(crossword[i+(3-k)][j+k]);
                } // both diagonals
            }

            // match string forwards or backwards
            if i+3 < n && (word1 == "XMAS" || word1 == "SAMX") { 
                cnt += 1;
            }
            if j+3 < m && (word2 == "XMAS" || word2 == "SAMX") { 
                cnt += 1;
            }
            if i+3 < n && j+3 < m {
                if word3 == "XMAS" || word3 == "SAMX" { 
                    cnt += 1;
                }
                if word4 == "XMAS" || word4 == "SAMX" { 
                    cnt += 1;
                }
            }
        }
    }

    return cnt;
}

fn part2(crossword: &Vec<Vec<char>>) -> i32 {
    let mut cnt = 0;
    let n = crossword.len();
    let m = crossword[0].len();

    // iterate through all 3x3 blocks in the crossword
    for i in 0..(n-2) {
        for j in 0..(m-2) {
            // looks ugly and i hate it
            if crossword[i+1][j+1] == 'A' &&
               (crossword[i][j] == 'M' && crossword[i+2][j+2] == 'S' || crossword[i][j] == 'S' && crossword[i+2][j+2] == 'M') &&
               (crossword[i+2][j] == 'M' && crossword[i][j+2] == 'S' || crossword[i+2][j] == 'S' && crossword[i][j+2] == 'M')
            {
                cnt += 1;
            }
        }
    }

    return cnt;
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

    let mut crossword: Vec<Vec<char>> = Vec::new();

    for line in buffer.lines() {
        // gather string data
        let text = line.unwrap();
        let row = text.chars().collect(); // vec of chars because working with strings SUCKS
        crossword.push(row);
    }

    let cnt = if args[1] == "--1" {part1(&crossword)} else {part2(&crossword)};
    println!("{:?}", cnt);
}