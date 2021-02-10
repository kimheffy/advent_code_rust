pub fn lower_half(row: &mut Vec<i32>) {
    let stop = row.len() / 2;
    row.truncate(stop);
}

pub fn upper_half(row: &Vec<i32>) -> Vec<i32> {
    let stop = row.len() / 2;
    let (_, right) = row.split_at(stop);
    right.to_vec()
}

pub fn set_up(max: i32) -> Vec<i32> {
    let mut cols: Vec<i32> = Vec::new();
    for i in 0..max {
        cols.push(i);
    }

    cols
}

pub struct Grid {
    rows: Vec<i32>,
    cols: Vec<i32>,
}

impl Grid {
    pub fn new(row_max: i32, col_max: i32) -> Grid {
        Grid {
            rows: set_up(row_max),
            cols: set_up(col_max),
        }
    }
}

pub mod logic {
    use crate::{Grid, lower_half, upper_half};
    pub fn run(seat_code: &str) -> (i32, i32) {
        let mut grid: Grid = Grid::new(128, 8);

        let code: String = seat_code.to_lowercase();

        let mut row: i32 = 0;
        let mut col: i32 = 0;

        for (i, c) in code.chars().enumerate() {
            if i < 7 {
                if i == 6 {
                    if c == 'f' {
                        row = grid.rows[0];
                    } else {
                        row = grid.rows[1];
                    }
                } else {
                    match c {
                        'f' => lower_half(&mut grid.rows),
                        'b' => {
                            grid.rows = upper_half(&grid.rows);
                        },
                        _ => panic!("The first character shouldn't contain {}", c),
                    }
                }
            } else {
                if i == 9 {
                    if c == 'r' {
                        col = grid.cols[1];
                    } else {
                        col = grid.cols[0];
                    }
                } else {
                    match c {
                        'r' => {
                            grid.cols = upper_half(&grid.cols);
                        },
                        'l' => lower_half(&mut grid.cols),
                        _ => panic!("Last 3 chars shouldn't contain {}", c),
                    }
                }
            }
        }

        (row, col)
    }

    pub fn compute(tup: (i32, i32)) -> i32 {
        (tup.0 * 8) + tup.1
    }
}