use regex::Regex;
use std::{collections::HashMap, thread};

fn main() {
    part1("advent-of-code-inputs/2023/day-08/example-1");
    part1("advent-of-code-inputs/2023/day-08/example-2");
    part1("advent-of-code-inputs/2023/day-08/input");
    part2("advent-of-code-inputs/2023/day-08/example-3");
    part2("advent-of-code-inputs/2023/day-08/input");
}

#[derive(Debug, Clone)]
struct Network {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn part1(file: &str) {
    let lines = utils::read_lines(file);
    let nw = parse_input(lines);

    let starting_point = "AAA";
    let ending_point = "ZZZ";

    let mut current_node_name = starting_point.to_string();

    let mut steps: u64 = 0;
    let mut curr_instruction: usize = 0;
    loop {
        let current_node = nw.nodes.get(&current_node_name).unwrap();
        if current_node.name == ending_point {
            break;
        }

        current_node_name = match nw.instructions[curr_instruction] {
            Direction::Left => current_node.left.clone(),
            Direction::Right => current_node.right.clone(),
        };

        steps += 1;
        curr_instruction = (curr_instruction + 1) % nw.instructions.len();
    }

    println!("number of steps: {steps}");
}

fn part2(file: &str) {
    let lines = utils::read_lines(file);
    let nw = parse_input(lines);

    let starting_points = find_starting_points(&nw);
    let cycle_sizes = get_cycle_sizes(&nw, starting_points);
    let lcm = lcm(cycle_sizes);

    println!("least common multiple: {:?}", lcm);
}

fn parse_input(lines: Vec<String>) -> Network {
    let mut instructions = Vec::new();

    for dir_str in lines[0].chars() {
        if dir_str == 'L' {
            instructions.push(Direction::Left);
        } else {
            instructions.push(Direction::Right);
        }
    }

    let mut nodes = HashMap::new();

    let node_regex = Regex::new(r"(?<name>.*) = \((?<left>.*), (?<right>.*)\)").unwrap();
    for line in lines[2..].iter() {
        let Some(caps) = node_regex.captures(line) else {
            panic!();
        };

        nodes.insert(
            caps["name"].to_string(),
            Node {
                name: caps["name"].to_string(),
                left: caps["left"].to_string(),
                right: caps["right"].to_string(),
            },
        );
    }

    Network {
        instructions,
        nodes,
    }
}

fn find_starting_points(nw: &Network) -> Vec<String> {
    nw.nodes
        .iter()
        .filter(|(node_name, _)| node_name.ends_with('A'))
        .map(|(ref name, _)| name.to_string())
        .collect()
}

// I was initially going for an offset cycle using Floyd's Algorithm,
// but after peeking into the sub-reddit and seeing people commenting
// that the loop is the whole chain (without a starting point), things
// got simpler.
fn get_cycle_sizes(nw: &Network, starting_nodes: Vec<String>) -> Vec<usize> {
    let mut cycle_sizes = Vec::new();
    let mut curr_instruction: usize = 0;

    let mut jh_list = Vec::new();

    for path in starting_nodes {
        let mut current_node_name = path.clone();
        let nw = nw.clone();

        let jh = thread::spawn(move || {
            let mut current_cycle_size: usize = 0;

            loop {
                let current_node = nw.nodes.get(&current_node_name).unwrap();
                if current_node.name.ends_with('Z') {
                    break;
                }

                current_node_name = match nw.instructions[curr_instruction] {
                    Direction::Left => current_node.left.clone(),
                    Direction::Right => current_node.right.clone(),
                };

                current_cycle_size += 1;
                curr_instruction = (curr_instruction + 1) % nw.instructions.len();
            }

            current_cycle_size
        });

        jh_list.push(jh)
    }

    for jh in jh_list {
        if let Ok(cycle_size) = jh.join() {
            cycle_sizes.push(cycle_size);
        }
    }

    cycle_sizes
}

fn lcm(values: Vec<usize>) -> usize {
    // trying to multiply everything all at once is problematic, gives
    // an overflow, need to do it by parts
    values.iter().fold(1, |acc, &value| {
        (acc * value) / euclidean_algorithm(acc, value)
    })
}

fn euclidean_algorithm(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        let (bigger, smaller) = if a > b { (a, b) } else { (b, a) };
        euclidean_algorithm(smaller, bigger % smaller)
    }
}
