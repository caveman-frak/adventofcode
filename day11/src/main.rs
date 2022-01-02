use {
    anyhow::Result,
    common::{
        convert::to_vec_u32,
        input::{from_path, list},
    },
};

fn main() -> Result<()> {
    let inputs = list(to_vec_u32, from_path("day11/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(inputs: &[Vec<u32>]) -> Result<()> {
    let mut grid = Grid::new(inputs.to_vec());

    println!("Day 11 Part 1 => {:?}", grid.steps(100));

    Ok(())
}

fn part2(inputs: &[Vec<u32>]) -> Result<()> {
    let mut grid = Grid::new(inputs.to_vec());

    println!("Day 11 Part 2 => {}", grid.steps_until(100).unwrap_or(0));

    Ok(())
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<u32>>,
    rows: usize,
    columns: usize,
}

impl Grid {
    fn new(cells: Vec<Vec<u32>>) -> Self {
        let rows = cells.len();
        let columns = cells.iter().map(|v| v.len()).max().unwrap_or(0);
        Self {
            cells,
            rows,
            columns,
        }
    }

    fn step(&mut self) -> u32 {
        for r in 0..self.rows {
            for c in 0..self.columns {
                self.increment(r, c);
            }
        }
        self.reset()
    }

    fn increment(&mut self, row: usize, column: usize) {
        if self.cells[row][column] < 10 {
            self.cells[row][column] += 1;
            if self.cells[row][column] == 10 {
                self.flash(row, column);
            }
        }
    }

    fn flash(&mut self, row: usize, column: usize) {
        self.cells[row][column] = 10;
        let above = row > 0;
        let below = row + 1 < self.rows;
        let left = column > 0;
        let right = column + 1 < self.columns;

        if above && left {
            self.increment(row - 1, column - 1)
        }
        if above {
            self.increment(row - 1, column)
        }
        if above && right {
            self.increment(row - 1, column + 1)
        }
        if left {
            self.increment(row, column - 1)
        }
        if right {
            self.increment(row, column + 1)
        }
        if below && left {
            self.increment(row + 1, column - 1)
        }
        if below {
            self.increment(row + 1, column)
        }
        if below && right {
            self.increment(row + 1, column + 1)
        }
    }

    fn reset(&mut self) -> u32 {
        let mut count = 0u32;
        for r in 0..self.rows {
            for c in 0..self.columns {
                if self.cells[r][c] > 9 {
                    self.cells[r][c] = 0;
                    count += 1;
                }
            }
        }
        count
    }

    fn steps(&mut self, steps: u32) -> u32 {
        (1..=steps).fold(0, |acc, _| acc + self.step())
    }

    fn steps_until(&mut self, count: u32) -> Option<u32> {
        (1..).find(|_| self.step() == count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_inputs() -> Result<()> {
        let inputs = list(to_vec_u32, from_path("test/test.txt")?);

        let grid = Grid::new(inputs);
        assert_eq!(grid.rows, 10);
        assert_eq!(grid.columns, 10);
        assert_eq!(grid.cells[0], vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3]);

        Ok(())
    }

    fn test_inputs() -> Vec<Vec<u32>> {
        vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ]
    }

    #[test]
    fn check_grid() {
        let inputs = test_inputs();
        let grid = Grid::new(inputs);
        assert_eq!(grid.rows, 5);
        assert_eq!(grid.columns, 5);
        assert_eq!(grid.cells[0], vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn check_step() {
        let inputs = test_inputs();
        let mut grid = Grid::new(inputs);
        assert_eq!(grid.step(), 9);

        assert_eq!(
            grid.cells,
            vec![
                vec![3, 4, 5, 4, 3],
                vec![4, 0, 0, 0, 4],
                vec![5, 0, 0, 0, 5],
                vec![4, 0, 0, 0, 4],
                vec![3, 4, 5, 4, 3],
            ]
        );
    }

    #[test]
    fn check_steps() {
        let inputs = test_inputs();
        let mut grid = Grid::new(inputs);
        assert_eq!(grid.steps(2), 9);

        assert_eq!(
            grid.cells,
            vec![
                vec![4, 5, 6, 5, 4],
                vec![5, 1, 1, 1, 5],
                vec![6, 1, 1, 1, 6],
                vec![5, 1, 1, 1, 5],
                vec![4, 5, 6, 5, 4],
            ]
        );
    }

    #[test]
    fn check_input_steps() -> Result<()> {
        let inputs = list(to_vec_u32, from_path("test/test.txt")?);

        let mut grid = Grid::new(inputs);
        assert_eq!(grid.steps(100), 1656);
        assert_eq!(
            grid.cells,
            vec![
                vec![0, 3, 9, 7, 6, 6, 6, 8, 6, 6],
                vec![0, 7, 4, 9, 7, 6, 6, 9, 1, 8],
                vec![0, 0, 5, 3, 9, 7, 6, 9, 3, 3],
                vec![0, 0, 0, 4, 2, 9, 7, 8, 2, 2],
                vec![0, 0, 0, 4, 2, 2, 9, 8, 9, 2],
                vec![0, 0, 5, 3, 2, 2, 2, 8, 7, 7],
                vec![0, 5, 3, 2, 2, 2, 2, 9, 6, 6],
                vec![9, 3, 2, 2, 2, 2, 8, 9, 6, 6],
                vec![7, 9, 2, 2, 2, 8, 6, 8, 6, 6],
                vec![6, 7, 8, 9, 9, 9, 8, 7, 6, 6],
            ]
        );

        Ok(())
    }

    #[test]
    fn check_input_steps_until() -> Result<()> {
        let inputs = list(to_vec_u32, from_path("test/test.txt")?);

        let mut grid = Grid::new(inputs);
        assert_eq!(grid.steps_until(100), Some(195));
        assert_eq!(
            grid.cells,
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ]
        );

        Ok(())
    }
}
