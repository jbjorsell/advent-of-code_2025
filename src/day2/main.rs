use color_eyre::eyre;
use std::io::Write;
use tracing::debug;

mod product_id;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let print_debug = false;

    let input = std::fs::read_to_string("inputs/day2_input.txt")?;
    let id_intervals: Vec<&str> = input.split(",").collect();

    debug!(
        "Input has {} intervals. First 5 intervals: {:?}",
        id_intervals.len(),
        id_intervals.iter().take(5).collect::<Vec<_>>()
    );

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let debug_path = std::path::Path::new(manifest_dir).join("debug_output.txt");
    let mut debug_file = std::fs::File::create(debug_path)?;

    let mut num_repeated_twice = 0;
    let mut twice_repeated_ids_sum = 0;
    let mut num_repeated_any = 0;
    let mut any_repeated_ids_sum = 0;
    for interval in &id_intervals {
        let (start_str, end_str) = interval
            .split_once("-")
            .ok_or(eyre::eyre!("Invalid interval format"))?;
        let product_id_sequence =
            product_id::ProductIdSequence::new(start_str.parse()?, end_str.parse()?);
        for product_id in product_id_sequence.ids {
            let invalid_repeat_twice = !product_id.is_valid(true);
            let invalid_any_repeat = !product_id.is_valid(false);
            if invalid_repeat_twice {
                num_repeated_twice += 1;
                twice_repeated_ids_sum += product_id.id;
            }
            if invalid_any_repeat {
                num_repeated_any += 1;
                any_repeated_ids_sum += product_id.id;
            }
            if print_debug {
                writeln!(
                    debug_file,
                    "ID: {}, Repeats {} (twice {}), : num_repeated_twice {}, num_repeated_any: {}",
                    product_id.id,
                    bool_str(invalid_any_repeat),
                    bool_str(invalid_repeat_twice),
                    num_repeated_twice,
                    num_repeated_any
                )?;
            }
        }
    }

    println!("Sum of twice repeated IDs: {twice_repeated_ids_sum}");
    println!("Sum of any repeated IDs: {any_repeated_ids_sum}");
    Ok(())
}

fn bool_str(bool: bool) -> &'static str {
    if bool { "✅" } else { "❌" }
}
