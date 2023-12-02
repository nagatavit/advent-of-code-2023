use regex::Regex;

fn main() {
    part1("advent-of-code-inputs/2023/day-01/example");
    part1("advent-of-code-inputs/2023/day-01/input");
    part2("advent-of-code-inputs/2023/day-01/example-2");
    part2("advent-of-code-inputs/2023/day-01/input");
}

fn part1(file: &str) {
    let lines = utils::read_lines(file);

    let mut line_digits = Vec::new();

    for line in lines {
        let line = line.as_bytes();
        let first = line
            .iter()
            .find(|&c| *c >= b'0' && *c <= b'9')
            .map(|&c| c - b'0')
            .unwrap();
        let last = line
            .iter()
            .rev()
            .find(|&c| *c >= b'0' && *c <= b'9')
            .map(|&c| c - b'0')
            .unwrap();

        let number: u32 = first as u32 * 10 + last as u32;
        line_digits.push(number)
    }

    let sum: u32 = line_digits.iter().sum();

    println!("Sum of lines: {}", sum);
}

fn part2(file: &str) {
    let lines = utils::read_lines(file);

    let mut line_digits = Vec::new();

    // because of possible overlap numbers e.g. "oneight" (thanks
    // reddit memes), regex search might result in the wrong value.
    //
    // one (stupid, but easy) way to work around that is to reverse
    // the search and the string, so that the last will be the actual
    // last, non overlapping number
    let forward_pattern = r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine";
    let backwards_pattern = r"enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|9|8|7|6|5|4|3|2|1";

    let forward_regex = Regex::new(forward_pattern).unwrap();
    let backwards_regex = Regex::new(backwards_pattern).unwrap();

    for line in lines {
        let first = forward_regex
            .captures(&line)
            .map(|caps| caps.extract::<0>())
            .unwrap()
            .0;

        let first = convert_to_u32(first);

        // reverse the string to search
        let rev_line: String = line.chars().rev().collect();
        let last = backwards_regex
            .captures(&rev_line)
            .map(|caps| caps.extract::<0>())
            .unwrap()
            .0;

        // un-reverse the match and convert
        let last: String = last.chars().rev().collect();
        let last = convert_to_u32(&last);

        println!("value: {}", first * 10 + last);

        line_digits.push(first * 10 + last);
    }

    let sum: u32 = line_digits.iter().sum();

    println!("Sum of lines: {}", sum);
}

fn convert_to_u32(number: &str) -> u32 {
    match number {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        value => panic!("invalid value: {}", value),
    }
}
