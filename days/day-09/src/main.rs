fn main() {
    part1("advent-of-code-inputs/2023/day-09/example");
    part1("advent-of-code-inputs/2023/day-09/input");
    part2("advent-of-code-inputs/2023/day-09/example");
    part2("advent-of-code-inputs/2023/day-09/input");
}

struct Report {
    history: Vec<isize>,
}

fn part1(file: &str) {
    let lines = utils::read_lines(file);
    let reports = parse_input(lines);

    let mut extrapolations = Vec::new();

    for report in reports {
        extrapolations.push(extrapolate_report_forward(&report));
    }

    println!(
        "forward extrapolation sum: {}",
        extrapolations.iter().sum::<isize>()
    );
}

fn part2(file: &str) {
    let lines = utils::read_lines(file);
    let reports = parse_input(lines);

    let mut extrapolations = Vec::new();

    for report in reports {
        extrapolations.push(extrapolate_report_backwards(&report));
    }

    println!(
        "backwards extrapolation sum: {}",
        extrapolations.iter().sum::<isize>()
    );
}

fn parse_input(lines: Vec<String>) -> Vec<Report> {
    let mut reports = Vec::new();

    for line in lines {
        reports.push(Report {
            history: line
                .split(' ')
                .map(|s| s.parse::<isize>().unwrap())
                .collect(),
        })
    }

    reports
}

fn extrapolate_report_forward(report: &Report) -> isize {
    let mut layers = build_layers(report);

    // to extrapolate the last value, because the last layer is always
    // 0 we can just simply add all the values in the last column.
    let mut acc = 0;

    // unwind the layers
    while let Some(prev_layer) = layers.pop() {
        acc += prev_layer.last().unwrap();
    }

    acc
}

fn extrapolate_report_backwards(report: &Report) -> isize {
    let mut layers = build_layers(report);

    // Similar to the forward extrapolation, we just added the last
    // column. But here is the inverse, we need to subtract it to have
    // the previous value.
    let mut acc = 0;

    // unwind the layers
    while let Some(prev_layer) = layers.pop() {
        acc = prev_layer.first().unwrap() - acc;
    }

    acc
}

fn build_layers(report: &Report) -> Vec<Vec<isize>> {
    let mut layers: Vec<Vec<isize>> = Vec::new();

    layers.push(report.history.clone());

    // build the layers
    loop {
        let prev_layer = layers.last().unwrap();

        let mut layer_of_zeros = true;

        let mut new_layer = Vec::new();
        for i in 0..(prev_layer.len() - 1) {
            let value = prev_layer[i + 1] - prev_layer[i];
            if value != 0 {
                layer_of_zeros = false;
            }

            new_layer.push(value);
        }

        layers.push(new_layer);

        if layer_of_zeros {
            break;
        }
    }

    layers
}
