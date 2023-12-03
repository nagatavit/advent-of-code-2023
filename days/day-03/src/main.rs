fn main() {
    // part1("advent-of-code-inputs/2023/day-03/example");
    part1("advent-of-code-inputs/2023/day-03/input");
    // part2("advent-of-code-inputs/2023/day-03/example");
    // part2("advent-of-code-inputs/2023/day-03/input");
}

fn part1(file: &str) {
    let lines = utils::read_lines(file);
    let engine_sch = parse_engine_schematic(lines);

    let mut part_number = Vec::new();

    for number in &engine_sch.number_positions {
        if check_if_symbol_is_adjacent(&engine_sch.sch, number) {
            part_number.push(convert_num_to_u32(&engine_sch.sch, number));
        }
    }

    // println!("found the following part numbers: {:?}", part_number);
    println!("Sum of part numbers: {}", part_number.iter().sum::<u32>());
}

// fn part2(file: &str) {}

#[derive(Debug)]
enum SchField {
    Num(u32),
    Dot,
    Symbol(char),
}

#[derive(Debug)]
struct NumberPos {
    line: usize,
    col_span: (usize, usize),
}

#[derive(Debug)]
struct EngineSch {
    sch: Vec<Vec<SchField>>,
    // stores the starting and ending positions of numbers found
    number_positions: Vec<NumberPos>,
}

fn parse_engine_schematic(lines: Vec<String>) -> EngineSch {
    let mut engine_sch = EngineSch {
        sch: Vec::new(),
        number_positions: Vec::new(),
    };

    for line in lines {
        let mut sch_line = Vec::new();

        let mut prev_char_was_a_number = false;

        for c in line.chars() {
            if c.is_numeric() {
                let digit = c.to_digit(10).unwrap();

                sch_line.push(SchField::Num(digit));

                let line_num = engine_sch.sch.len();
                let line_len = sch_line.len() - 1;

                if prev_char_was_a_number {
                    engine_sch.number_positions.last_mut().unwrap().col_span.1 = line_len;
                } else {
                    engine_sch.number_positions.push(NumberPos {
                        line: line_num,
                        col_span: (line_len, line_len),
                    })
                }

                prev_char_was_a_number = true;
            } else if c == '.' {
                sch_line.push(SchField::Dot);
                prev_char_was_a_number = false;
            } else {
                sch_line.push(SchField::Symbol(c));
                prev_char_was_a_number = false;
            }
        }

        engine_sch.sch.push(sch_line);
    }

    engine_sch
}

fn check_if_symbol_is_adjacent(sch: &Vec<Vec<SchField>>, number_pos: &NumberPos) -> bool {
    for idx in indexes_around_number_pos(sch, number_pos) {
        if let SchField::Symbol(_) = sch[idx.0][idx.1] {
            return true;
        }
    }

    false
}

fn indexes_around_number_pos(
    sch: &Vec<Vec<SchField>>,
    number_pos: &NumberPos,
) -> Vec<(usize, usize)> {
    let mut box_around_number = Vec::new();

    let min_i = if number_pos.line > 0 {
        number_pos.line - 1
    } else {
        number_pos.line
    };

    let min_j = if number_pos.col_span.0 > 0 {
        number_pos.col_span.0 - 1
    } else {
        number_pos.col_span.0
    };

    let max_i = if number_pos.line < sch.len() - 1 {
        number_pos.line + 1
    } else {
        number_pos.line
    };

    let max_j = if number_pos.col_span.1 < sch[0].len() - 1 {
        number_pos.col_span.1 + 1
    } else {
        number_pos.col_span.1
    };

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if i == number_pos.line && j >= number_pos.col_span.0 && j <= number_pos.col_span.1 {
                continue;
            }
            box_around_number.push((i, j))
        }
    }

    box_around_number
}

fn convert_num_to_u32(sch: &Vec<Vec<SchField>>, number_pos: &NumberPos) -> u32 {
    let mut partial_sum = 0;

    for val in &sch[number_pos.line][number_pos.col_span.0..=number_pos.col_span.1] {
        if let SchField::Num(val) = val {
            partial_sum = partial_sum * 10 + val;
        }
    }

    partial_sum
}
