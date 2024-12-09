// everybody thank jerma for playing cooking mama while i wrote this

use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::env;
use std::cmp;

fn in_bounds(y: i32, x: i32, n: i32, m: i32) -> bool {
    return !(x < 0 || x >= m || y < 0 || y >= n);
}

fn part1(visited: &mut HashSet<i32>, n: i32, m: i32, diff: (i32, i32), a1_loc: (i32, i32), a2_loc: (i32, i32)) {
    // first antinode is antenna 1 - (antenna 2 - antenna 1) = 2 * antenna 1 - antenna 2
    let anti1: (i32, i32) = (a2_loc.0 + diff.0, a2_loc.1 + diff.1);
    if in_bounds(anti1.1, anti1.0, n, m) {
        visited.insert(anti1.1 * m + anti1.0);
    }

    // second antinode is antenna2 + (antenna 2 - antenna 1) = 2 * antenna 2 - antenna 1
    let anti2: (i32, i32) = (a1_loc.0 - diff.0, a1_loc.1 - diff.1);
    if in_bounds(anti2.1, anti2.0, n, m) {
        visited.insert(anti2.1 * m + anti2.0);
    }
}

fn part2(visited: &mut HashSet<i32>, n: i32, m: i32, mut diff: (i32, i32), a2_loc: (i32, i32)) {
    // find gcd and divide to ensure our interval is optimal
    let max = cmp::max(diff.0, diff.1);
    for i in (1..max).rev() {
        if diff.0 % i == 0 && diff.1 % i == 0 {
            diff.0 /= i;
            diff.1 /= i;
            break;
        }
    }

    // go one direction in steps of diff and count space
    let mut temp = (a2_loc.0, a2_loc.1);
    while in_bounds(temp.1, temp.0, n, m) {
        visited.insert(temp.1 * m + temp.0);
        temp = (temp.0 - diff.0, temp.1 - diff.1);
    }

    // go other direction in steps of diff and count space
    temp = (a2_loc.0, a2_loc.1);
    while in_bounds(temp.1, temp.0, n, m) {
        visited.insert(temp.1 * m + temp.0);
        temp = (temp.0 + diff.0, temp.1 + diff.1);
    }
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

    // thanks aaron (who doesnt know im doing this)
    let map: Vec<Vec<char>> = buffer.lines().map(|text| text.unwrap().chars().collect()).collect();

    let n = map.len() as i32;
    let m = map[0].len() as i32;

    let mut antenna_types: HashMap<char, Vec<i32>> = HashMap::new();
    let mut antenna_pairs: HashMap<i32, Vec<i32>> = HashMap::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let row_i32 = row as i32;
            let col_i32 = col as i32;

            // store all antenna positions by type
            if map[row][col] != '.' {
                if antenna_types.contains_key(&map[row][col]) {
                    for loc in antenna_types.get(&map[row][col]).unwrap() {
                        // for each antenna, form a pair with all previous antennas
                        // ...think of this as forming a complete graph
                        if !antenna_pairs.contains_key(&(row_i32*m + col_i32)) {
                            antenna_pairs.insert(row_i32*m + col_i32, Vec::new());
                        }
                        antenna_pairs.get_mut(&(row_i32*m + col_i32)).unwrap().push(*loc);
                    }
                } else {
                    antenna_types.insert(map[row][col], Vec::new());
                }
                antenna_types.get_mut(&map[row][col]).unwrap().push(row_i32*m + col_i32);
            }
        }
    }

    let mut visited = HashSet::new();
    for (key, value) in antenna_pairs.into_iter() {
        let a1_loc: (i32, i32) = (key / m, key % m);
        for loc in value {
            let a2_loc: (i32, i32) = (loc / m, loc % m);

            // diff = antenna 2 - antenna 1
            let diff: (i32, i32) = (a2_loc.0 - a1_loc.0, a2_loc.1 - a1_loc.1);

            if args[1] == "--1" {part1(&mut visited, n, m, diff, a1_loc, a2_loc)} else {part2(&mut visited, n, m, diff, a2_loc)};
        }
    }

    println!("{:?}", visited.len());
}