fn main() {
    part1("advent-of-code-inputs/2023/day-05/example");
    part1("advent-of-code-inputs/2023/day-05/input");
    part2("advent-of-code-inputs/2023/day-05/example");
    part2("advent-of-code-inputs/2023/day-05/input");
}

pub fn part1(file: &str) {
    let lines = utils::read_lines(file);
    let (seeds, maps) = parse_inputs(lines);
    let map_order = find_map_order(&maps);

    let min_location = seeds
        .iter()
        .map(|&s| convert_seed_to_location(s, &maps, &map_order))
        .min();

    // println!("{:#?}", maps);
    println!("{:?}", min_location)
}

fn part2(file: &str) {
    let lines = utils::read_lines(file);
    let (seeds, maps) = parse_inputs(lines);
    let map_order = find_map_order(&maps);

    let flat_map = flatten_maps(&maps, &map_order);

    let mut seed_pairs = Vec::new();
    for i in 0..seeds.len() {
        if i % 2 != 0 {
            continue;
        }

        seed_pairs.push((seeds[i] as i64, seeds[i + 1] as i64));
    }

    let min_map = find_min_map_locations(&flat_map);

    let mut current_min = u32::MAX as i64;
    for map_range in min_map.iter() {
        let mut found_min = false;

        for (seed, range) in seed_pairs.iter() {
            if let Some(new_min) = find_if_seed_is_in_range(*seed, *range, map_range) {
                if new_min < current_min {
                    current_min = new_min;
                }
                found_min = true;
            }
        }

        if found_min {
            break;
        }
    }

    println!(">>>>>>>>>>>>>>>>> {:?}", current_min)
}

type Seeds = Vec<u32>;

#[derive(Debug, Default, Clone)]
struct AlmanacMap {
    name: String,
    from: String,
    to: String,
    original_to_new: Vec<MapRange>,
}

#[derive(Debug, Default, Clone)]
struct MapRange {
    dst: u32,
    src: u32,
    range: u32,
}

#[derive(Debug, Default, Clone)]
struct FlatRange {
    src_start: i64,
    src_end: i64,
    partial_operations: i64,
}

fn parse_inputs(lines: Vec<String>) -> (Seeds, Vec<AlmanacMap>) {
    let mut seeds = Vec::new();
    let mut maps = Vec::new();

    let mut current_map = AlmanacMap::default();

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            parse_seeds(line, &mut seeds);
            continue;
        }

        if line.is_empty() {
            if i != 1 {
                maps.push(current_map);
                current_map = AlmanacMap::default();
            }
            continue;
        }

        let split: Vec<&str> = line.split(' ').collect();
        match *split.last().unwrap() {
            "map:" => {
                let name = split.first().unwrap();
                let name_split: Vec<&str> = name.split('-').collect();

                current_map.name = name.to_string();
                current_map.from = name_split.first().unwrap().to_string();
                current_map.to = name_split.last().unwrap().to_string();
            }
            _ => {
                let mut src = 0;
                let mut dst = 0;
                let mut range = 0;

                for (i, value) in split.iter().enumerate() {
                    match i {
                        0 => dst = value.parse().unwrap(),
                        1 => src = value.parse().unwrap(),
                        2 => range = value.parse().unwrap(),
                        _ => panic!("¯\\_(ツ)_/¯"),
                    }
                }

                current_map
                    .original_to_new
                    .push(MapRange { dst, src, range });
            }
        }

        // edge case for the last map (too lazy to refactor this)
        if i == lines.len() - 1 {
            maps.push(current_map.clone());
        }
    }

    (seeds, maps)
}

fn parse_seeds(line: &str, seeds: &mut Seeds) {
    for seed in line
        .split(": ")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split(' ')
    {
        seeds.push(seed.parse::<u32>().unwrap())
    }
}

// Probably just overthinking, but 🤷
fn find_map_order(almanacs: &[AlmanacMap]) -> Vec<usize> {
    let source = "seed";
    let destination = "location";

    let mut path = Vec::new();

    let first = almanacs.iter().position(|a| a.from == source).unwrap();
    path.push(first);

    let mut dst_pos = first;

    loop {
        let next_dst = &almanacs[dst_pos].to;
        if next_dst == destination {
            break;
        }
        dst_pos = almanacs.iter().position(|a| &a.from == next_dst).unwrap();
        path.push(dst_pos);
    }

    path
}

fn convert_seed_to_location(seed: u32, almanacs: &[AlmanacMap], order: &Vec<usize>) -> u32 {
    let mut curr_value = seed;
    for &i in order {
        let curr_almanac = &almanacs[i];
        curr_value = match curr_almanac
            .original_to_new
            .iter()
            .find(|m| m.src <= curr_value && curr_value <= m.src + m.range)
        {
            Some(map_range) => {
                let diff = curr_value - map_range.src;
                map_range.dst + diff
            }
            None => curr_value,
        };
    }

    curr_value
}

fn find_if_seed_is_in_range(seed: i64, seed_range: i64, flat_range: &FlatRange) -> Option<i64> {
    if seed + seed_range <= flat_range.src_start || flat_range.src_end <= seed {
        None
    } else {
        // find the lowest intersection bound:
        if seed > flat_range.src_start {
            Some(seed + flat_range.partial_operations)
        } else {
            Some(flat_range.src_start + flat_range.partial_operations)
        }
    }
}

// since we have already a flatten map, we can find the ranges with
// the minimum starting range after the transformation
fn find_min_map_locations(flat_map: &[FlatRange]) -> Vec<FlatRange> {
    let mut flat_map = flat_map.to_vec();

    flat_map.sort_by_key(|m| m.src_start + m.partial_operations);

    flat_map
}

fn flatten_maps(almanacs: &[AlmanacMap], order: &[usize]) -> Vec<FlatRange> {
    // start with the first map
    let mut flat_map = sort_and_fill_map(&almanacs[order[0]]);

    for &i in &order[1..] {
        let curr_almanac = &almanacs[i];

        let next_flat_map = sort_and_fill_map(curr_almanac);

        flat_map = merge_maps(&flat_map, &next_flat_map);
        // println!("merged: {:#?}", flat_map);
    }

    flat_map
}

fn sort_and_fill_map(almanac: &AlmanacMap) -> Vec<FlatRange> {
    let mut new_map = almanac.original_to_new.clone();

    new_map.sort_by_key(|m| m.src);
    let mut flat_map = Vec::new();

    // if the first element doesn't start at 0, need to create the
    // first element
    if let Some(first) = new_map.first() {
        if first.src > 0 {
            flat_map.push(FlatRange {
                src_start: 0,
                src_end: first.src as i64,
                partial_operations: 0,
            })
        }
    }

    // iterate over each original map
    for original in new_map.iter() {
        // check if there's a gap between the filled map and the
        // original one
        if let Some(last) = flat_map.last() {
            if last.src_end < original.src as i64 {
                flat_map.push(FlatRange {
                    src_start: last.src_end,
                    src_end: original.src as i64,
                    partial_operations: 0,
                })
            }
        }

        // actually append the original map now
        flat_map.push(FlatRange {
            src_start: original.src as i64,
            src_end: original.src as i64 + original.range as i64,
            partial_operations: original.dst as i64 - original.src as i64,
        })
    }

    // just as a safe measure, fill up to u32 max
    if let Some(last) = flat_map.last() {
        flat_map.push(FlatRange {
            src_start: last.src_end,
            src_end: u32::MAX as i64,
            partial_operations: 0,
        })
    }

    // println!("flatten: {:#?}", flat_map);

    flat_map
}

fn merge_maps(map_1: &Vec<FlatRange>, map_2: &Vec<FlatRange>) -> Vec<FlatRange> {
    let mut merged = Vec::new();

    for range_1 in map_1 {
        for range_2 in map_2 {
            let shifted_range_2_start = range_2.src_start - range_1.partial_operations;
            let shifted_range_2_end = range_2.src_end - range_1.partial_operations;

            // disjoint
            if range_1.src_end <= shifted_range_2_start || shifted_range_2_end <= range_1.src_start
            {
                continue;
            }

            // find intersecting bounds
            let lower_bound = if shifted_range_2_start > range_1.src_start {
                shifted_range_2_start
            } else {
                range_1.src_start
            };

            let upper_bound = if shifted_range_2_end < range_1.src_end {
                shifted_range_2_end
            } else {
                range_1.src_end
            };

            merged.push(FlatRange {
                src_start: lower_bound,
                src_end: upper_bound,
                partial_operations: range_1.partial_operations + range_2.partial_operations,
            });
        }
    }

    merged
}
