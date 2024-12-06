// getting the hang of it

use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::env;

fn in_bounds(y: i32, x: i32, n: i32, m: i32) -> bool {
    return !(x < 0 || x >= m || y < 0 || y >= n);
}

fn part2(map: &mut Vec<Vec<char>>, visited: &HashSet<i32>, n: i32, m: i32, x: i32, y: i32, direction: i32, direction_map: &HashMap<i32, (i32, i32)>) -> usize {
    let mut valid = 0;

    // try every space the guard has visited (if the guard never visited a space, placing an obstruction there can't alter the path)
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            // if this isn't the starting position or an obstruction
            if !(col == (x as usize) && row == (y as usize)) && map[row][col] != '#' && visited.contains(&((row as i32)*m + (col as i32))) {
                // place an obstruction
                map[row][col] = '#';

                // >>> simulate guard movement <<<

                let mut temp_x = x;
                let mut temp_y = y;
                let mut temp_direction = direction;
                let mut stops: HashMap<i32, HashSet<i32>> = HashMap::from([
                    (0, HashSet::new()),
                    (1, HashSet::new()),
                    (2, HashSet::new()),
                    (3, HashSet::new())
                ]); // map from direction to obstructions
                
                while in_bounds(temp_y, temp_x, n, m) {
                    // if we hit an obstruction
                    if map[temp_y as usize][temp_x as usize] == '#' {
                        // go back in the direction we came
                        temp_x -= direction_map.get(&temp_direction).unwrap().0;
                        temp_y -= direction_map.get(&temp_direction).unwrap().1;

                        // check if we've stopped at this exact spot in this exact direction before
                        if stops.get(&temp_direction).unwrap().contains(&(temp_y*m + temp_x)) {
                            // means we're in a loop
                            valid += 1;
                            // println!("{}, {}", col, row);
                            break;
                        }

                        // indicate that we stopped at this spot while going in this direction
                        stops.get_mut(&temp_direction).unwrap().insert(temp_y*m + temp_x);

                        // turn 90 deg
                        temp_direction = (temp_direction + 1) % 4;
                    }

                    // move !!
                    temp_x += direction_map.get(&temp_direction).unwrap().0;
                    temp_y += direction_map.get(&temp_direction).unwrap().1;
                }

                // remove obstruction
                map[row][col] = '.';
            }
        }
    }
    
    return valid;
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
    let mut map: Vec<Vec<char>> = buffer.lines().map(|text| text.unwrap().chars().collect()).collect();

    let n = map.len() as i32;
    let m = map[0].len() as i32;

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut direction = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == '^' || map[row][col] == '>' || map[row][col] == 'v' || map[row][col] == '<' {
                x = col as i32;
                y = row as i32;

                if map[row][col] == '^' {direction = 0}
                if map[row][col] == '>' {direction = 1};
                if map[row][col] == 'v' {direction = 2};
                if map[row][col] == '<' {direction = 3};

                break;
            }
        }
    }

    let direction_map: HashMap::<i32, (i32, i32)> = HashMap::from([
        (0, (0, -1)),
        (1, (1, 0)),
        (2, (0, 1)),
        (3, (-1, 0))
    ]);
    let mut visited: HashSet<i32> = HashSet::new();

    let mut temp_x = x;
    let mut temp_y = y;
    let mut temp_direction = direction;
    while in_bounds(temp_y, temp_x, n, m) {
        if map[temp_y as usize][temp_x as usize] == '#' {
            temp_x -= direction_map.get(&temp_direction).unwrap().0;
            temp_y -= direction_map.get(&temp_direction).unwrap().1;
            temp_direction = (temp_direction + 1) % 4;
        }

        visited.insert(temp_y*m + temp_x);
        temp_x += direction_map.get(&temp_direction).unwrap().0;
        temp_y += direction_map.get(&temp_direction).unwrap().1;
    }

    let cnt = if args[1] == "--1" {visited.len() /* part 1 */} else {part2(&mut map, &visited, n, m, x, y, direction, &direction_map)};

    println!("{:?}", cnt);
}