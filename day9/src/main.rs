use {
    anyhow::Result,
    common::{
        convert::to_vec_u32,
        input::{from_path, list},
    },
};

fn main() -> Result<()> {
    let inputs = list(to_vec_u32, from_path("day9/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(inputs: &[Vec<u32>]) -> Result<()> {
    let grid = Grid::new(inputs);

    let low_points = grid.low_points();
    println!("Day 9 Part 1 => {:?}", low_points.iter().sum::<u32>());

    Ok(())
}

fn part2(_inputs: &[Vec<u32>]) -> Result<()> {
    println!("Day 9 Part 2 => ");

    Ok(())
}

struct Grid<'a> {
    cells: &'a [Vec<u32>],
    rows: u32,
    columns: u32,
}

impl<'a> Grid<'a> {
    fn new(cells: &'a [Vec<u32>]) -> Self {
        let rows = cells.len() as u32;
        let columns = cells.iter().map(|v| v.len()).max().unwrap_or(0) as u32;
        Self {
            cells,
            rows,
            columns,
        }
    }

    fn cell(&self, row: u32, column: u32) -> u32 {
        self.cells[row as usize][column as usize]
    }

    fn above(&self, row: u32, column: u32) -> Option<u32> {
        if row > 0 {
            Some(self.cell(row - 1, column))
        } else {
            None
        }
    }

    fn below(&self, row: u32, column: u32) -> Option<u32> {
        if row + 1 < self.rows {
            Some(self.cell(row + 1, column))
        } else {
            None
        }
    }

    fn left(&self, row: u32, column: u32) -> Option<u32> {
        if column > 0 {
            Some(self.cell(row, column - 1))
        } else {
            None
        }
    }

    fn right(&self, row: u32, column: u32) -> Option<u32> {
        if column + 1 < self.columns {
            Some(self.cell(row, column + 1))
        } else {
            None
        }
    }

    fn low_point(&self, row: u32, column: u32) -> Option<u32> {
        let low = self.cell(row, column);
        if low < self.above(row, column).unwrap_or(10)
            && low < self.below(row, column).unwrap_or(10)
            && low < self.left(row, column).unwrap_or(10)
            && low < self.right(row, column).unwrap_or(10)
        {
            Some(low + 1)
        } else {
            None
        }
    }

    fn low_points(&self) -> Vec<u32> {
        (0..self.rows)
            .map(move |r| {
                (0..self.columns)
                    .filter_map(move |c| self.low_point(r, c))
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_inputs() -> Result<()> {
        let inputs = list(to_vec_u32, from_path("test/test.txt")?);

        let grid = Grid::new(&inputs);
        assert_eq!(grid.rows, 5);
        assert_eq!(grid.columns, 10);
        assert_eq!(grid.cells[0], vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0]);

        Ok(())
    }

    fn test_inputs() -> Vec<Vec<u32>> {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
    }

    #[test]
    fn check_grid() {
        let inputs = test_inputs();
        let grid = Grid::new(&inputs);
        assert_eq!(grid.rows, 5);
        assert_eq!(grid.columns, 10);
        assert_eq!(grid.cells[0], vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn check_cell() {
        let inputs = test_inputs();
        let grid = Grid::new(&inputs);

        assert_eq!(grid.cell(0, 0), 2, "cell");
        assert_eq!(grid.above(0, 0), None, "above");
        assert_eq!(grid.below(0, 0), Some(3), "below");
        assert_eq!(grid.left(0, 0), None, "left");
        assert_eq!(grid.right(0, 0), Some(1), "right");
    }

    #[test]
    fn check_low_point() {
        let inputs = test_inputs();
        let grid = Grid::new(&inputs);

        assert_eq!(grid.low_point(0, 0), None, "0.0");
        assert_eq!(grid.low_point(0, 1), Some(2), "0.1");
        assert_eq!(grid.low_point(2, 2), Some(6), "2.2");
        assert_eq!(grid.low_point(2, 4), None, "2.4");
        assert_eq!(grid.low_point(4, 9), None, "4.9");
    }

    #[test]
    fn check_low_points() {
        let inputs = test_inputs();
        let grid = Grid::new(&inputs);

        let low_points = grid.low_points();
        assert_eq!(low_points, vec![2, 1, 6, 6]);
        assert_eq!(low_points.iter().sum::<u32>(), 15);
    }
}
