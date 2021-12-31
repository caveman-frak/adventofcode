use {
    anyhow::Result,
    common::{
        convert::to_vec_u32,
        input::{from_path, list},
    },
    std::collections::HashSet,
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

    println!(
        "Day 9 Part 1 => {:?}",
        low_points.iter().map(|p| p.risk()).sum::<u32>()
    );

    Ok(())
}

fn part2(inputs: &[Vec<u32>]) -> Result<()> {
    let grid = Grid::new(inputs);
    let basins = grid.basins(3);

    println!("Day 9 Part 2 => {}", basins.iter().product::<u32>());

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

    fn cell(&self, row: u32, column: u32) -> Point {
        Point::new(row, column, self.cells[row as usize][column as usize])
    }

    fn above(&self, row: u32, column: u32) -> Option<(u32, u32)> {
        if row > 0 {
            Some((row - 1, column))
        } else {
            None
        }
    }

    fn below(&self, row: u32, column: u32) -> Option<(u32, u32)> {
        if row + 1 < self.rows {
            Some((row + 1, column))
        } else {
            None
        }
    }

    fn left(&self, row: u32, column: u32) -> Option<(u32, u32)> {
        if column > 0 {
            Some((row, column - 1))
        } else {
            None
        }
    }

    fn right(&self, row: u32, column: u32) -> Option<(u32, u32)> {
        if column + 1 < self.columns {
            Some((row, column + 1))
        } else {
            None
        }
    }

    fn low_point(&self, row: u32, column: u32) -> Option<Point> {
        let point = self.cell(row, column);
        let low = point.height;
        if low
            < self
                .above(row, column)
                .map_or(10, |(r, c)| self.cell(r, c).height)
            && low
                < self
                    .below(row, column)
                    .map_or(10, |(r, c)| self.cell(r, c).height)
            && low
                < self
                    .left(row, column)
                    .map_or(10, |(r, c)| self.cell(r, c).height)
            && low
                < self
                    .right(row, column)
                    .map_or(10, |(r, c)| self.cell(r, c).height)
        {
            Some(point)
        } else {
            None
        }
    }

    fn low_points(&self) -> Vec<Point> {
        (0..self.rows)
            .map(move |r| {
                (0..self.columns)
                    .filter_map(move |c| self.low_point(r, c))
                    .collect::<Vec<Point>>()
            })
            .flatten()
            .collect()
    }

    fn basin(&self, row: u32, column: u32) -> u32 {
        let mut basin = HashSet::new();
        let mut boundary: HashSet<Point> = HashSet::new();

        self._basin(row, column, &mut basin, &mut boundary);

        basin.len() as u32
    }

    fn _basin(
        &self,
        row: u32,
        column: u32,
        basin: &mut HashSet<Point>,
        boundary: &mut HashSet<Point>,
    ) {
        let point = self.cell(row, column);

        if point.is_boundary() {
            boundary.insert(point);
        } else if basin.insert(point) {
            if let Some((row, column)) = self.above(row, column) {
                self._basin(row, column, basin, boundary);
            }
            if let Some((row, column)) = self.below(row, column) {
                self._basin(row, column, basin, boundary);
            }
            if let Some((row, column)) = self.left(row, column) {
                self._basin(row, column, basin, boundary);
            }
            if let Some((row, column)) = self.right(row, column) {
                self._basin(row, column, basin, boundary);
            }
        }
    }

    fn basins(&self, count: usize) -> Vec<u32> {
        let mut v: Vec<u32> = self
            .low_points()
            .iter()
            .map(|p| self.basin(p.row, p.column))
            .collect();

        v.sort_unstable();

        v.iter().copied().rev().take(count).collect::<Vec<u32>>()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    row: u32,
    column: u32,
    height: u32,
}

impl Point {
    fn new(row: u32, column: u32, height: u32) -> Self {
        Self {
            row,
            column,
            height,
        }
    }

    fn risk(&self) -> u32 {
        self.height + 1
    }

    fn is_boundary(&self) -> bool {
        self.height >= 9
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

        assert_eq!(grid.cell(0, 0), Point::new(0, 0, 2), "cell");
        assert_eq!(grid.above(0, 0), None, "above");
        assert_eq!(grid.below(0, 0), Some((1, 0)), "below");
        assert_eq!(grid.left(0, 0), None, "left");
        assert_eq!(grid.right(0, 0), Some((0, 1)), "right");
    }

    #[test]
    fn check_low_point() {
        let inputs = test_inputs();
        let grid = Grid::new(&inputs);

        assert_eq!(grid.low_point(0, 0), None, "0.0");
        assert_eq!(grid.low_point(0, 1), Some(Point::new(0, 1, 1)), "0.1");
        assert_eq!(grid.low_point(2, 2), Some(Point::new(2, 2, 5)), "2.2");
        assert_eq!(grid.low_point(2, 4), None, "2.4");
        assert_eq!(grid.low_point(4, 9), None, "4.9");
    }

    #[test]
    fn check_low_points() {
        let inputs = test_inputs();
        let grid = Grid::new(&inputs);

        let low_points = grid.low_points();
        assert_eq!(
            low_points,
            vec![
                Point::new(0, 1, 1),
                Point::new(0, 9, 0),
                Point::new(2, 2, 5),
                Point::new(4, 6, 5)
            ]
        );
        assert_eq!(low_points.iter().map(|p| p.risk()).sum::<u32>(), 15);
    }

    #[test]
    fn check_basin() {
        let inputs = test_inputs();
        let grid = Grid::new(&inputs);

        assert_eq!(grid.basin(0, 1), 3, "0.1");
        assert_eq!(grid.basin(0, 9), 9, "0.9");
        assert_eq!(grid.basin(2, 2), 14, "2.2");
        assert_eq!(grid.basin(4, 6), 9, "4.6");
    }

    #[test]
    fn check_basins() {
        let inputs = test_inputs();
        let grid = Grid::new(&inputs);
        let v = grid.basins(3);

        assert_eq!(v, vec![14, 9, 9]);
        assert_eq!(v.iter().product::<u32>(), 1134);
    }
}
