use color_eyre::eyre;
use std::io::Write;
use tracing::debug;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let input = read_input()?;
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
    let mut num_zeroes_encountered = 0;
    for line in &lines {
        let direction = &line[0..1];
        let value = line[1..].parse::<i32>()?;
        match direction {
            "L" => curr_val -= value,
            "R" => curr_val += value,
            _ => panic!("Unknown direction: {}", direction),
        }
        curr_val %= 100;
        writeln!(debug_file, "{} -> {}", line, curr_val)?;
        if curr_val == 0 {
            num_zeroes_encountered += 1;
        }
    }

    println!("Final value: {num_zeroes_encountered}");
    Ok(())
}

fn read_input() -> Result<String, std::io::Error> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir).join("input.txt");
    std::fs::read_to_string(path)
}
