fn main() {
    let input = std::fs::read_to_string("inputs/day4_input.txt").unwrap();
    let rows: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let (height, width) = (rows.len() as isize, rows[0].len() as isize);

    println!("Task 1: {}", task_1(&rows, height, width));

    let mut grid: Vec<Vec<u8>> = rows.iter().map(|row| row.to_vec()).collect();
    println!("Task 2: {}", task_2(&mut grid, height, width));
}

fn is_movable(
    grid: &[impl AsRef<[u8]>],
    height: isize,
    width: isize,
    row: isize,
    col: isize,
) -> bool {
    (-1..=1)
        .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
        .filter(|&(dr, dc)| dr != 0 || dc != 0)
        .filter(|&(dr, dc)| {
            let (r, c) = (row + dr, col + dc);
            r >= 0
                && r < height
                && c >= 0
                && c < width
                && grid[r as usize].as_ref()[c as usize] == b'@'
        })
        .count()
        < 4
}

fn task_1(rows: &[&[u8]], height: isize, width: isize) -> usize {
    (0..height)
        .flat_map(|row| (0..width).map(move |col| (row, col)))
        .filter(|&(row, col)| rows[row as usize][col as usize] != b'.')
        .filter(|&(row, col)| is_movable(rows, height, width, row, col))
        .count()
}

fn task_2(grid: &mut [Vec<u8>], height: isize, width: isize) -> usize {
    let mut total = 0;
    loop {
        let removable: Vec<_> = (0..height)
            .flat_map(|row| (0..width).map(move |col| (row, col)))
            .filter(|&(row, col)| grid[row as usize][col as usize] != b'.')
            .filter(|&(row, col)| is_movable(grid, height, width, row, col))
            .collect();

        if removable.is_empty() {
            break;
        }

        total += removable.len();
        removable
            .into_iter()
            .for_each(|(r, c)| grid[r as usize][c as usize] = b'.');
    }
    total
}
