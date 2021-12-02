use {
    anyhow::{anyhow, Error, Result},
    common::input::Inputs,
    std::{convert::TryFrom, path::Path, str::FromStr},
};

fn main() -> Result<()> {
    let mut submarine = Submarine::default();
    submarine.debug = true;
    let directions = Inputs::from_file(to_direction, Path::new("day2/data/input.txt"))?;
    submarine.adjustments(directions);
    println!(
        "submarine = {:?}, absolute = {}",
        submarine,
        submarine.absolute()
    );

    Ok(())
}

#[derive(Debug, Default)]
struct Submarine {
    debug: bool,
    horizontal: u32,
    depth: u32,
}

impl Submarine {
    fn adjust(&mut self, direction: Direction) -> &mut Self {
        match direction {
            Direction::Forward(x) => self.horizontal += x,
            Direction::Up(x) => self.depth -= x,
            Direction::Down(x) => self.depth += x,
        }

        if self.debug {
            println!("{:?} => {:?}", direction, self);
        }

        self
    }

    fn adjustments(&mut self, directions: Vec<Direction>) -> &mut Self {
        for direction in directions {
            self.adjust(direction);
        }

        self
    }

    fn absolute(&self) -> u32 {
        self.horizontal * self.depth
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl TryFrom<String> for Direction {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        Direction::try_from(&*s)
    }
}

impl TryFrom<&str> for Direction {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        if let Some(direction) = parts.next() {
            if let Some(distance) = parts.next() {
                let distance = distance.parse()?;
                match direction {
                    "forward" => Ok(Direction::Forward(distance)),
                    "up" => Ok(Direction::Up(distance)),
                    "down" => Ok(Direction::Down(distance)),
                    _ => Err(anyhow!("Invalid direction")),
                }
            } else {
                Err(anyhow!("Missing distance"))
            }
        } else {
            Err(anyhow!("Missing direction"))
        }
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Direction::try_from(s)
    }
}

fn to_direction(s: String) -> Option<Direction> {
    s.parse().ok()
}

#[cfg(test)]
mod tests {
    use {self::Direction::*, super::*};

    #[test]
    fn check_default() {
        let sub = Submarine::default();
        assert_eq!(sub.horizontal, 0);
        assert_eq!(sub.depth, 0);
        assert_eq!(sub.absolute(), 0);
    }

    #[test]
    fn check_adjust() {
        let mut sub = Submarine::default();
        sub.adjust(Forward(5));
        assert_eq!(sub.horizontal, 5);
        assert_eq!(sub.depth, 0);
        assert_eq!(sub.absolute(), 0);
    }

    #[test]
    fn check_adjustments() {
        let mut sub = Submarine::default();
        sub.adjustments(vec![
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2),
        ]);
        assert_eq!(sub.horizontal, 15);
        assert_eq!(sub.depth, 10);
        assert_eq!(sub.absolute(), 150);
    }

    #[test]
    fn check_direction_success() {
        assert_eq!(to_direction("forward 100".to_string()), Some(Forward(100)));
        assert_eq!(to_direction("up 20".to_string()), Some(Up(20)));
        assert_eq!(to_direction("down 10".to_string()), Some(Down(10)));
    }

    #[test]
    fn check_direction_failure() {
        assert_eq!(to_direction("  ".to_string()), None);
        assert_eq!(to_direction("10".to_string()), None);
        assert_eq!(to_direction("ABC 10".to_string()), None);
        assert_eq!(to_direction("forward".to_string()), None);
        assert_eq!(to_direction("forward ABC".to_string()), None);
    }
}
