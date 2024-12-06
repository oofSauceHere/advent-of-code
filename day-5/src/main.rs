// getting the hang of it

use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::env;

// O(i'm going to kill myself)
fn part2(ruleset: &HashMap<i32, Vec<i32>>, mut update: Vec<i32>) -> i32 {
    let mut loc = HashMap::new();
    for page_num in 0..update.len() {
        loc.insert(update[page_num], page_num);
    }

    let mut page_num = 0;
    while page_num < update.len() {
        let page = update[page_num];

        if ruleset.contains_key(&page) {
            let following = ruleset.get(&page).unwrap();
            let mut min_follow = update.len()-1;
            for follow in following {
                if loc.contains_key(&follow) {
                    let follow_num = loc.get(&follow).unwrap();
                    if *follow_num < min_follow {
                        min_follow = *follow_num;
                    }
                }
            }

            if min_follow < page_num {
                for i in ((min_follow+1)..=page_num).rev() {
                    let temp = update[i];
                    update[i] = update[i-1];
                    update[i-1] = temp;

                    let temp_loc = *loc.get(&update[i]).unwrap();
                    *loc.get_mut(&update[i]).unwrap() = *loc.get(&update[i-1]).unwrap();
                    *loc.get_mut(&update[i-1]).unwrap() = temp_loc;
                }
                
                page_num = min_follow;
            }
        }

        page_num += 1;
    }

    return update[update.len()/2];
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

    let mut ruleset = HashMap::new();
    let mut updates = Vec::new();

    let mut manual = true;
    for line in buffer.lines() {
        let text = line.unwrap().to_string();
        if text == "" {
            manual = !manual;
            continue;
        }

        if manual {
            let order: Vec<i32> = text.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            if !ruleset.contains_key(&order[0]) {
                ruleset.insert(order[0], Vec::new());
            }

            ruleset.get_mut(&order[0]).unwrap().push(order[1]);
        } else {
            let update: Vec<i32> = text.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            updates.push(update);
        }
    }

    let mut sum = 0;

    // over all updates
    for update in updates {
        let mut visited = HashSet::new();
        let mut valid = true; // valid update

        // over all pages
        for page in &update {
            // if page has associated rule
            if ruleset.contains_key(&page) {
                let following = ruleset.get(&page).unwrap();

                // over all follow requirements
                for follow in following {

                    // if follow requirement was previously found, we've broken rule
                    if visited.contains(follow) {
                        valid = false;
                        break;
                    }
                }
            }

            visited.insert(page);
        }

        if args[1] == "--1" && valid {
            // part 1
            sum += update[update.len()/2];
        } else if args[1] == "--2" && !valid {
            sum += part2(&ruleset, update.clone());
        }
    }

    println!("{:?}", sum);
}