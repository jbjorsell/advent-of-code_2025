use color_eyre::eyre;
use std::io::Write;
use tracing::debug;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let input = std::fs::read_to_string("inputs/day1_input.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    debug!(
        "Input has {} lines. First 5 lines: {:?}",
        lines.len(),
        lines.iter().take(5).collect::<Vec<_>>()
    );

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let debug_path = std::path::Path::new(manifest_dir).join("debug_output.txt");
    let mut debug_file = std::fs::File::create(debug_path)?;

    let mut curr_val = 50;
    let mut num_zeroes_passed = 0;
    for line in &lines {
        let prev_val = curr_val;
        let (direction, value) = line.split_at(1);
        let value = value.parse::<i32>()?;

        let full_laps = value / 100;
        num_zeroes_passed += full_laps;

        let value_mod = value.rem_euclid(100);

        if value_mod == 0 {
            continue;
        }

        match direction {
            "L" => {
                if curr_val != 0 && value_mod >= curr_val {
                    num_zeroes_passed += 1;
                }
                curr_val -= value_mod;
            }
            "R" => {
                if curr_val + value_mod >= 100 {
                    num_zeroes_passed += 1;
                }
                curr_val += value_mod;
            }
            _ => panic!("Unknown direction: {}", direction),
        }
        curr_val = curr_val.rem_euclid(100);
        writeln!(
            debug_file,
            "{} -> {} ({}), num_zeroes_passed: {}",
            prev_val, curr_val, line, num_zeroes_passed
        )?;
    }

    println!("Final value: {num_zeroes_passed}");
    Ok(())
}
