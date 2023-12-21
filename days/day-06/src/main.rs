//! V = t
//!
//! d = (T - t)*V
//! d = T*t - t^2
//! -t^2 + T*t - d = 0
//!
//! t = (-T +- sqrt(T^2 - 4d)) / -2

fn main() {
    part1("advent-of-code-inputs/2023/day-06/example");
    part1("advent-of-code-inputs/2023/day-06/input");
    part2("advent-of-code-inputs/2023/day-06/example");
    part2("advent-of-code-inputs/2023/day-06/input");
}

#[derive(Debug)]
struct Race {
    time: f64,
    record: f64,
}

fn part1(file: &str) {
    let lines = utils::read_lines(file);
    let races = parse_input(lines);

    let mut chances_to_win = Vec::new();

    for race in races {
        let (t1, t2) = quadratic_solutions(race.record + 1.0, race.time);
        // println!("race with time {}: {t1}, {t2}", race.time);
        chances_to_win.push(t2 - t1 + 1)
    }

    let result: u32 = chances_to_win.iter().product();
    println!("chances to win multiplied: {result}");
}

fn part2(file: &str) {
    let lines = utils::read_lines(file);
    let race = parse_input_with_correct_kernel(lines);
    println!("chances to win: {:?}", race);

    let (t1, t2) = quadratic_solutions(race.record + 1.0, race.time);
    let result = t2 - t1 + 1;

    println!("chances to win: {result}");
}

fn parse_input(lines: Vec<String>) -> Vec<Race> {
    let times: Vec<&str> = lines[0].split(' ').filter(|t| !t.is_empty()).collect();
    let distances: Vec<&str> = lines[1].split(' ').filter(|d| !d.is_empty()).collect();

    let mut races = Vec::new();

    for (i, time) in times[1..].iter().enumerate() {
        races.push(Race {
            time: time.parse().unwrap(),
            record: distances[i + 1].parse().unwrap(),
        })
    }

    races
}

fn parse_input_with_correct_kernel(lines: Vec<String>) -> Race {
    let times: Vec<&str> = lines[0].split(' ').filter(|t| !t.is_empty()).collect();
    let distances: Vec<&str> = lines[1].split(' ').filter(|d| !d.is_empty()).collect();

    let corrected_time = times[1..].join("");
    let corrected_distance = distances[1..].join("");

    Race {
        time: corrected_time.parse().unwrap(),
        record: corrected_distance.parse().unwrap(),
    }
}

// and round it
fn quadratic_solutions(min_distance: f64, time: f64) -> (u32, u32) {
    let delta = time * time - 4.0 * min_distance;

    let t1 = (-time - delta.sqrt()) / (-2.0);
    let t1 = t1.floor() as u32;

    let t2 = (-time + delta.sqrt()) / (-2.0);
    let t2 = t2.ceil() as u32;

    // because the signal of the parabola is inverted, the order is
    // also inverted (from the plus and minus operations)
    (t2, t1)
}
