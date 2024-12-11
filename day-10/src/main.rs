use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::fs::File;
use std::env;

fn part1(map: &Vec<Vec<i32>>, i: usize, j: usize, prev: i32, visited: &mut HashSet<usize>) -> i32 {
    let m = map.len();
    let n = map[0].len();

    if visited.contains(&(i*m + j)) || map[i][j] != prev+1 {
        return 0;
    }

    visited.insert(i*m + j);

    if map[i][j] == 9 {
        return 1;
    }

    let mut sum = 0;
    if i < m-1 {
        sum += part1(map, i+1, j, map[i][j], visited);
    }
    if i > 0 {
        sum += part1(map, i-1, j, map[i][j], visited)
    }
    if j < n-1 {
        sum += part1(map, i, j+1, map[i][j], visited);
    }
    if j > 0 {
        sum += part1(map, i, j-1, map[i][j], visited);
    }

    return sum;
}

fn part2(map: &Vec<Vec<i32>>, i: usize, j: usize, prev: i32) -> i32 {
    let m = map.len();
    let n = map[0].len();

    if map[i][j] != prev+1 {
        return 0;
    }

    if map[i][j] == 9 {
        return 1;
    }

    let mut sum = 0;
    if i < m-1 {
        sum += part2(map, i+1, j, map[i][j]);
    }
    if i > 0 {
        sum += part2(map, i-1, j, map[i][j])
    }
    if j < n-1 {
        sum += part2(map, i, j+1, map[i][j]);
    }
    if j > 0 {
        sum += part2(map, i, j-1, map[i][j]);
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
    let input: File = File::open(path).unwrap();
    let buffer: BufReader<File> = BufReader::new(input);

    let map: Vec<Vec<i32>> = buffer.lines().map(|text| text.unwrap().chars().map(|c| (c.to_digit(10).unwrap() as i32)).collect()).collect();
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                let mut visited: HashSet<usize> = HashSet::new();
                sum += if args[1] == "--1" {part1(&map, i, j, -1, &mut visited)} else {part2(&map, i, j, -1)};
            }
        }
    }

    // if args[1] == "--1" {part1()} else {part2()};
    println!("{:?}", sum);
}
