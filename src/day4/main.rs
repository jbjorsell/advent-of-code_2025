fn main() {
    let input = std::fs::read_to_string("inputs/day4_input.txt").unwrap();
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let (height, width) = (grid.len() as isize, grid[0].len() as isize);

    let mut first_round = true;
    let mut rolls_freed = 0;

    loop {
        let removable: Vec<_> = (0..height)
            .flat_map(|row| (0..width).map(move |col| (row, col)))
            .filter(|&(row, col)| grid[row as usize][col as usize] != b'.')
            .filter(|&(row, col)| {
                [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .filter(|&(dr, dc)| {
                    let (r, c) = (row + dr, col + dc);
                    r >= 0
                        && r < height
                        && c >= 0
                        && c < width
                        && grid[r as usize][c as usize] == b'@'
                })
                .count()
                    < 4
            })
            .collect();

        if removable.is_empty() {
            break;
        }

        if first_round {
            println!("Task 1: {}", removable.len());
            first_round = false;
        }

        rolls_freed += removable.len();
        removable
            .into_iter()
            .for_each(|(r, c)| grid[r as usize][c as usize] = b'.');
    }
    println!("Task 2: {rolls_freed}");
}
