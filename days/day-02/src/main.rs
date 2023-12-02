fn main() {
    // part1("advent-of-code-inputs/2023/day-02/example");
    part1("advent-of-code-inputs/2023/day-02/input");
    part2("advent-of-code-inputs/2023/day-02/example");
    part2("advent-of-code-inputs/2023/day-02/input");
}

#[derive(Debug)]
enum CubeColor {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<CubeSet>,
}

fn part1(file: &str) {
    let lines = utils::read_lines(file);

    let mut valid_games = Vec::new();

    for line in lines {
        let game = parse_game(&line);
        if check_if_game_is_valid(&game) {
            valid_games.push(game.id);
        }
    }

    let sum: u32 = valid_games.iter().sum();

    println!("Sum of valid games: {}", sum);
}

fn part2(file: &str) {
    let lines = utils::read_lines(file);

    let mut games_set_powers = Vec::new();

    for line in lines {
        let game = parse_game(&line);
        let min_set = find_minimum_viable_set(&game);
        games_set_powers.push(calculate_set_power(&min_set));
    }

    let sum: u32 = games_set_powers.iter().sum();

    println!("Sum of games minimum set: {}", sum);
}

fn parse_game(line: &str) -> Game {
    let game_output: Vec<&str> = line.split(": ").collect();

    // too lazy to make it pretty:
    //
    // splitting the "Game X:" string here
    let id = game_output
        .first()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();

    // spliting the second part:
    //
    // " X <color>(,) X <color>;", etc
    let game_output = game_output.last().unwrap();
    let cubes_withdraw: Vec<&str> = game_output.split("; ").collect();

    let mut subsets = Vec::new();

    for round in cubes_withdraw {
        let cube_colors: Vec<&str> = round.split(", ").collect();

        let mut new_cube_set = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube in cube_colors {
            match parse_cube_color(cube) {
                CubeColor::Red(value) => new_cube_set.red = value,
                CubeColor::Green(value) => new_cube_set.green = value,
                CubeColor::Blue(value) => new_cube_set.blue = value,
            }
        }

        subsets.push(new_cube_set);
    }

    Game { id, subsets }
}

fn parse_cube_color(cube_output: &str) -> CubeColor {
    let splited_output = cube_output.split(' ').collect::<Vec<&str>>();
    match splited_output[1] {
        "red" => CubeColor::Red(splited_output[0].parse().unwrap()),
        "green" => CubeColor::Green(splited_output[0].parse().unwrap()),
        "blue" => CubeColor::Blue(splited_output[0].parse().unwrap()),
        value => panic!("invalid color: {value}"),
    }
}

fn check_if_game_is_valid(game: &Game) -> bool {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    for round in game.subsets.iter() {
        if round.red > red_limit || round.green > green_limit || round.blue > blue_limit {
            return false;
        }
    }

    true
}

fn find_minimum_viable_set(game: &Game) -> CubeSet {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    game.subsets.iter().for_each(|set| {
        update_max(&mut max_red, &set.red);
        update_max(&mut max_green, &set.green);
        update_max(&mut max_blue, &set.blue)
    });

    CubeSet {
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}

fn update_max(curr_max: &mut u32, new_value: &u32) {
    if new_value > curr_max {
        *curr_max = *new_value;
    }
}

fn calculate_set_power(cube_set: &CubeSet) -> u32 {
    cube_set.red * cube_set.green * cube_set.blue
}
