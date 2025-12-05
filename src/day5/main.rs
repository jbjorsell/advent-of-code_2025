fn parse_range(line: &str) -> (u64, u64) {
    let (start, end) = line.split_once('-').unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_unstable();
    ranges
        .into_iter()
        .fold(Vec::new(), |mut acc, (start, end)| {
            match acc.last_mut() {
                Some(last) if start <= last.1 + 1 => last.1 = last.1.max(end),
                _ => acc.push((start, end)),
            }
            acc
        })
}

fn main() {
    let input = std::fs::read_to_string("inputs/day5_input.txt").unwrap();
    let (fresh, available) = input.split_once("\n\n").unwrap();

    let fresh_ranges = merge_ranges(fresh.lines().map(parse_range).collect());

    let num_fresh_available = available
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|&ingredient| {
            fresh_ranges
                .iter()
                .any(|&(start, end)| ingredient >= start && ingredient <= end)
        })
        .count();

    let num_fresh_ingredients: u64 = fresh_ranges
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum();

    println!("Task 1: {num_fresh_available}");
    println!("Task 2: {num_fresh_ingredients}");
}
