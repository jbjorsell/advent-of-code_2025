fn main() {
    let input = std::fs::read_to_string("inputs/day4_input.txt").unwrap();
    let rows: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let (height, width) = (rows.len() as isize, rows[0].len() as isize);
    let delta_idxs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let free_rolls = (0..height)
        .flat_map(|row| (0..width).map(move |col| (row, col)))
        .filter(|&(row, col)| rows[row as usize][col as usize] != b'.')
        .filter(|&(row, col)| {
            delta_idxs
                .iter()
                .filter(|&&(dr, dc)| {
                    let (nr, nc) = (row + dr, col + dc);
                    // Validate indices and check for '@'
                    nr >= 0
                        && nr < height
                        && nc >= 0
                        && nc < width
                        && rows[nr as usize][nc as usize] == b'@'
                })
                .count()
                < 4
        })
        .count();

    println!("Task 1: {free_rolls}");
}
