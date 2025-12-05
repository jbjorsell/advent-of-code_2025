fn main() {
    let input = std::fs::read_to_string("inputs/day5_input.txt").unwrap();
    let (fresh, available) = input.split_once("\n\n").unwrap();

    let fresh_ranges: Vec<_> = fresh
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect();

    let num_fresh_available = available
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|&ingredient| {
            fresh_ranges
                .iter()
                .any(|&(start, end)| ingredient >= start && ingredient <= end)
        })
        .count();

    println!("Task 1: {num_fresh_available}");
}
