use {
    anyhow::{anyhow, Error, Result},
    common::input::{from_path, inputs},
    std::{collections::HashMap, iter::FusedIterator, str::FromStr},
};

fn main() -> Result<()> {
    let inputs = inputs(to_line, from_path("day5/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(inputs: &[Line]) -> Result<()> {
    let mut grid = Grid::default();
    inputs.iter().for_each(|l| grid.plot(&l, false));
    println!("Day 5 Part 1 => {}", grid.points(2).count());

    Ok(())
}

fn part2(inputs: &[Line]) -> Result<()> {
    let mut grid = Grid::default();
    inputs.iter().for_each(|l| grid.plot(&l, true));
    println!("Day 5 Part 2 => {}", grid.points(2).count());

    Ok(())
}

fn to_line(s: String) -> Option<Line> {
    s.parse().ok()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Ord)]
struct Point {
    x: u32,
    y: u32,
}

#[allow(dead_code)]
impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn step(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: Self::adjust(self.x, dx, dy),
            y: Self::adjust(self.y, dy, dx),
        }
    }

    fn adjust(start: u32, delta: i32, other: i32) -> u32 {
        let start = start as i32;
        let max = (delta.abs().max(other.abs())) as f32;
        let delta = delta as f32;
        (start + (delta / max).round() as i32) as u32
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(',');
        if let Some(x) = parts.next() {
            if let Some(y) = parts.next() {
                return Ok(Point {
                    x: x.trim().parse()?,
                    y: y.trim().parse()?,
                });
            }
        }
        Err(anyhow!("Missing coordinates"))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Line {
    start: Point,
    end: Point,
}

#[allow(dead_code)]
impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn coords(startx: u32, starty: u32, endx: u32, endy: u32) -> Self {
        Self::new(Point::new(startx, starty), Point::new(endx, endy))
    }

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_diagonal(&self) -> bool {
        (self.end.y as i32 - self.start.y as i32).abs()
            == (self.end.x as i32 - self.start.x as i32).abs()
    }

    fn points(&self) -> PointIterator {
        PointIterator::new(self)
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" -> ");
        if let Some(start) = parts.next() {
            if let Some(end) = parts.next() {
                return Ok(Line {
                    start: start.trim().parse()?,
                    end: end.trim().parse()?,
                });
            }
        }
        Err(anyhow!("Missing coordinates"))
    }
}

struct PointIterator {
    current: Option<Point>,
    end: Point,
}

impl PointIterator {
    fn new(line: &Line) -> Self {
        Self {
            current: Some(line.start),
            end: line.end,
        }
    }

    fn dx(&self) -> i32 {
        if let Some(current) = self.current {
            self.end.x as i32 - current.x as i32
        } else {
            0
        }
    }

    fn dy(&self) -> i32 {
        if let Some(current) = self.current {
            self.end.y as i32 - current.y as i32
        } else {
            0
        }
    }
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        if let Some(current) = self.current {
            let dx = self.dx();
            let dy = self.dy();

            self.current = if dx == 0 && dy == 0 {
                None
            } else {
                Some(current.step(dx, dy))
            };
        }
        result
    }
}

impl FusedIterator for PointIterator {}

#[derive(Debug)]
struct Grid {
    points: HashMap<Point, u32>,
}

impl Grid {
    fn plot(&mut self, line: &Line, allow_diagonal: bool) {
        if line.is_horizontal() || line.is_vertical() || (allow_diagonal && line.is_diagonal()) {
            for p in line.points() {
                *self.points.entry(p).or_default() += 1;
            }
        }
    }

    fn points(&self, threshold: u32) -> impl Iterator<Item = &Point> {
        self.points
            .iter()
            .filter_map(move |(k, v)| if *v >= threshold { Some(k) } else { None })
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            points: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_point_fromstr() {
        assert_eq!(" 10,2 ".parse::<Point>().unwrap(), Point { x: 10, y: 2 })
    }

    #[test]
    fn check_line_from_str() {
        assert_eq!(
            "0,9 -> 5,9".parse::<Line>().unwrap(),
            Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            }
        )
    }

    #[test]
    fn check_horizontal() {
        assert_eq!(Line::coords(0, 0, 0, 5).is_horizontal(), true);
        assert_eq!(Line::coords(5, 0, 0, 0).is_horizontal(), false);
        assert_eq!(Line::coords(0, 0, 5, 5).is_horizontal(), false);
        assert_eq!(Line::coords(0, 0, 2, 5).is_horizontal(), false);
    }

    #[test]
    fn check_vertical() {
        assert_eq!(Line::coords(0, 0, 0, 5).is_vertical(), false);
        assert_eq!(Line::coords(5, 0, 0, 0).is_vertical(), true);
        assert_eq!(Line::coords(0, 0, 5, 5).is_vertical(), false);
        assert_eq!(Line::coords(0, 0, 2, 5).is_vertical(), false);
    }

    #[test]
    fn check_diagonal() {
        assert_eq!(Line::coords(0, 0, 0, 5).is_diagonal(), false);
        assert_eq!(Line::coords(5, 0, 0, 0).is_diagonal(), false);
        assert_eq!(Line::coords(0, 0, 5, 5).is_diagonal(), true);
        assert_eq!(Line::coords(0, 0, 2, 5).is_diagonal(), false);
    }

    #[test]
    fn check_point_iter_up() {
        assert_eq!(
            Line::coords(0, 0, 0, 5).points().collect::<Vec<Point>>(),
            vec![
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
                Point::new(0, 4),
                Point::new(0, 5)
            ]
        );
    }

    #[test]
    fn check_point_iter_across() {
        assert_eq!(
            Line::coords(0, 0, 3, 0).points().collect::<Vec<Point>>(),
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
            ]
        );
    }

    #[test]
    fn check_point_iter_diagonal() {
        assert_eq!(
            Line::coords(0, 0, 2, 5).points().collect::<Vec<Point>>(),
            vec![
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(1, 2),
                Point::new(1, 3),
                Point::new(2, 4),
                Point::new(2, 5)
            ]
        );
    }

    #[test]
    fn check_point_iter_backwards() {
        assert_eq!(
            Line::coords(0, 5, 0, 0).points().collect::<Vec<Point>>(),
            vec![
                Point::new(0, 5),
                Point::new(0, 4),
                Point::new(0, 3),
                Point::new(0, 2),
                Point::new(0, 1),
                Point::new(0, 0)
            ]
        );
    }

    #[test]
    fn check_grid_plot() {
        let mut grid = Grid::default();
        grid.plot(&Line::coords(0, 9, 5, 9), false);
        assert_eq!(grid.points.len(), 6);
        let mut points = grid.points.iter().collect::<Vec<(&Point, &u32)>>();
        points.sort();
        assert_eq!(
            points,
            vec![
                (&Point::new(0, 9), &1),
                (&Point::new(1, 9), &1),
                (&Point::new(2, 9), &1),
                (&Point::new(3, 9), &1),
                (&Point::new(4, 9), &1),
                (&Point::new(5, 9), &1)
            ]
        );
    }

    #[test]
    fn check_grid_plot_diagonal() {
        let mut grid = Grid::default();
        grid.plot(&Line::coords(0, 4, 5, 9), false);
        assert_eq!(grid.points.len(), 0);
    }

    #[test]
    fn check_grid_plot_diagonal_allowed() {
        let mut grid = Grid::default();
        grid.plot(&Line::coords(0, 4, 5, 9), true);
        assert_eq!(grid.points.len(), 6);
        let mut points = grid.points.iter().collect::<Vec<(&Point, &u32)>>();
        points.sort();
        assert_eq!(
            points,
            vec![
                (&Point::new(0, 4), &1),
                (&Point::new(1, 5), &1),
                (&Point::new(2, 6), &1),
                (&Point::new(3, 7), &1),
                (&Point::new(4, 8), &1),
                (&Point::new(5, 9), &1)
            ]
        );
    }

    #[test]
    fn check_grid_plot_backwards() {
        let mut grid = Grid::default();
        grid.plot(&Line::coords(5, 9, 0, 9), false);
        assert_eq!(grid.points.len(), 6);
        let mut points = grid.points.iter().collect::<Vec<(&Point, &u32)>>();
        points.sort();
        assert_eq!(
            points,
            vec![
                (&Point::new(0, 9), &1),
                (&Point::new(1, 9), &1),
                (&Point::new(2, 9), &1),
                (&Point::new(3, 9), &1),
                (&Point::new(4, 9), &1),
                (&Point::new(5, 9), &1)
            ]
        );
    }

    #[test]
    fn check_grid_points() {
        let mut grid = Grid::default();
        let lines = vec![
            Line::coords(0, 9, 5, 9),
            Line::coords(8, 0, 0, 8),
            Line::coords(9, 4, 3, 4),
            Line::coords(2, 2, 2, 1),
            Line::coords(7, 0, 7, 4),
            Line::coords(6, 4, 2, 0),
            Line::coords(0, 9, 2, 9),
            Line::coords(3, 4, 1, 4),
            Line::coords(0, 0, 8, 8),
            Line::coords(5, 5, 8, 2),
        ];
        lines.iter().for_each(|l| grid.plot(&l, false));
        let mut points = grid.points(2).collect::<Vec<&Point>>();
        points.sort();
        assert_eq!(points.len(), 5);
        assert_eq!(
            points,
            vec![
                &Point::new(0, 9),
                &Point::new(1, 9),
                &Point::new(2, 9),
                &Point::new(3, 4),
                &Point::new(7, 4)
            ]
        );
    }
}
