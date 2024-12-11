#![warn(clippy::pedantic)]

use std::fs;
use std::env;

fn part1(diskmap: &Vec<i32>, disk: &mut Vec<i64>) {
    let mut file = 0;
    let mut empty: Vec<usize> = Vec::new();

    for i in 0..diskmap.len() {
        let num = diskmap[i]; // get_unchecked??
        if i % 2 == 0 {
            for _ in 0..num {
                disk.push(file);
            }
            file += 1;
        } else {
            for _ in 0..num {
                empty.push(disk.len());
                disk.push(-1);
            }
        }
    }

    let mut loc = 0;
    let mut skips = 0;
    for i in (0..disk.len()).rev() {
        if loc + skips == empty.len() {
            break;
        }

        if disk[i] == -1 {
            skips += 1;
            continue;
        }

        disk[empty[loc]] = disk[i];
        disk[i] = -1;
        loc += 1;
    }
}

fn part2(diskmap: &Vec<i32>, disk: &mut Vec<i64>) {
    let mut file = 0;
    let mut blocks: Vec<(usize, i32)> = Vec::new();
    let mut empty: Vec<(usize, i32)> = Vec::new();

    for i in 0..diskmap.len() {
        let num = diskmap[i];
        let mut select_vec = &mut blocks;
        let mut select_val = file;
        if i % 2 == 1 {
            select_vec = &mut empty;
            select_val = -1
        };

        select_vec.push((disk.len(), num));
        for _ in 0..num {
            disk.push(select_val);
        }
            
        if i % 2 == 0 {file += 1};
    }

    for i in (0..blocks.len()).rev() {
        for j in &mut empty {
            let num = i as usize;
            if blocks[num].0 <= j.0 {
                break;
            }

            if blocks[num].1 <= j.1 {
                for ind in 0..blocks[num].1 {
                    disk[j.0 + (ind as usize)] = disk[blocks[num].0 + (ind as usize)];
                    disk[blocks[num].0 + (ind as usize)] = -1;
                }
                j.0 += blocks[num].1 as usize;
                j.1 -= blocks[num].1;
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || (args[1] != "--1" && args[1] != "--2") {
        println!("Invalid argument.");
        return;
    }

    // what an awesome freakign line
    let diskmap: Vec<i32> = fs::read_to_string("input.txt").unwrap().chars().map(|c| (c.to_digit(10).unwrap() as i32)).collect();
    let mut disk: Vec<i64> = Vec::new();

    if args[1] == "--1" {part1(&diskmap, &mut disk)} else {part2(&diskmap, &mut disk)};

    let mut sum = 0;
    for i in 0..disk.len() {
        if disk[i] == -1 {
            continue;
        }

        sum += (i as i64) * disk[i];
    }

    println!("{:?}", sum);
}
