use color_eyre::eyre;
use std::io::Write;
use tracing::debug;

mod dial;

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

    let mut dial = dial::Dial::new();

    let mut num_zeroes = 0;
    let mut num_zeroes_passed = 0;
    for line in &lines {
        let prev_val = dial.get_value();
        let turn_result = dial.turn(line.parse()?);

        if turn_result.final_value == 0 {
            num_zeroes += 1;
        }
        num_zeroes_passed += turn_result.zeroes_passed;

        writeln!(
            debug_file,
            "{} -> {} ({}), num_zeroes_passed: {}",
            prev_val, turn_result.final_value, line, num_zeroes_passed
        )?;
    }

    println!("Passed zero {num_zeroes_passed} times and entered zero {num_zeroes} times.");
    Ok(())
}
