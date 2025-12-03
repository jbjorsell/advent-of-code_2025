fn main() {
    let input = std::fs::read_to_string("inputs/day3_input.txt").unwrap();

    for (task, n_batteries) in [(1, 2), (2, 12)] {
        let total_joltage: u64 = input
            .lines()
            .map(|line| bank_joltage(line.as_bytes(), n_batteries))
            .sum();
        println!("Task {task}: {total_joltage}");
    }
}

fn bank_joltage(bank: &[u8], n_active_batteries: usize) -> u64 {
    let mut start_idx = 0;
    let mut joltage = 0;

    for battery_idx in 0..n_active_batteries {
        let n_batteries_left = n_active_batteries - battery_idx;
        let end_idx = bank.len() - (n_batteries_left - 1);

        let (idx, &byte) = bank[start_idx..end_idx]
            .iter()
            .enumerate()
            // Use rev since max_by_key returns last max on ties, we want first max
            .rev()
            .max_by_key(|(_, val)| *val)
            .unwrap();

        start_idx += idx + 1;
        let multiplier = 10_u64.pow((n_batteries_left - 1) as u32);
        joltage += multiplier * (byte - b'0') as u64;
    }
    joltage
}
