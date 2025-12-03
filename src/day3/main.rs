fn main() {
    let input = std::fs::read_to_string("inputs/day3_input.txt").unwrap();

    let mut sum_joltage = 0;
    for line in input.lines() {
        let bank = line.bytes();
        let (first_idx, first_byte) = bank
            .clone()
            .enumerate()
            .rev()
            .skip(1)
            .max_by_key(|(_, val)| *val)
            .unwrap();
        let second_byte = bank.skip(first_idx + 1).max().unwrap();
        let joltage = (first_byte - b'0') as u32 * 10 + (second_byte - b'0') as u32;
        sum_joltage += joltage;
        // println!("Joltage: {joltage} (bytes: {first_byte}{second_byte})");
    }
    println!("Task 1: {sum_joltage}");
}
