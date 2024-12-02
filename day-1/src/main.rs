use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn part1(list1: &mut Vec<i32>, list2: &mut Vec<i32>) {
    list1.sort();
    list2.sort();

    let mut res: i32 = 0;
    for i in 0..list1.len() {
        let diff = list2[i] - list1[i];
        res += diff.abs();
    }

    println!("Part 1: {}", res);
}

fn part2(list1: &mut Vec<i32>, list2: &mut Vec<i32>) {
    let mut freq = HashMap::new(); // i dont specify type, but prob involves "&mut i32"
    for n in list2 {
        if freq.contains_key(&n) { // i dont know what these ampersands are for
            *freq.get_mut(&n).unwrap() += 1; // access element and modify it (annoying)
        } else {
            freq.insert(n, 1);
        }
    }

    let mut res = 0;
    for n in list1 {
        if freq.contains_key(&n) {
            let freq = freq.get(&n).unwrap(); // alternatively, dont unwrap and handle the none case
            res += *n * freq;
        }
    }

    println!("Part 2: {}", res);
}

fn main() {
    let path: &str = "input.txt";
    let input: File = File::open(path).unwrap();
    let buffer: BufReader<File> = BufReader::new(input);

    // i32... my beloved c++ int...
    let mut list1: Vec<i32> = Vec::new(); // calling a default constructor
    let mut list2: Vec<i32> = Vec::new();
    for line in buffer.lines() {
        let l = line.unwrap();
        let mut iter = l.split_whitespace();
        list1.push(iter.next().unwrap().parse().expect("invalid num1")); // resolve result to expect parsing error
        list2.push(iter.next().unwrap().parse().expect("invalid num2"));
    }

    // part1(&mut list1, &mut list2);
    part2(&mut list1, &mut list2);
}