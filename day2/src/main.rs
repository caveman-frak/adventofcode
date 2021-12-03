use {
    anyhow::{anyhow, Error, Result},
    common::input::Inputs,
    std::{convert::TryFrom, str::FromStr},
};

fn main() -> Result<()> {
    let mut submarine = Submarine::default();
    let directions = Inputs::from_path(to_direction, "day2/data/input.txt")?;
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
    aim: i32,
}

impl Submarine {
    fn adjust(&mut self, direction: Direction) -> &mut Self {
        match direction {
            Direction::Forward(x) => {
                self.horizontal += x;
                self.depth = ((self.depth as i32) + (x as i32 * self.aim)) as u32;
            }
            Direction::Up(x) => self.aim -= x as i32,
            Direction::Down(x) => self.aim += x as i32,
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

    fn absolute(&self) -> u64 {
        self.horizontal as u64 * self.depth as u64
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
                    dir => Err(anyhow!("Invalid direction '{}'", dir)),
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
        assert_eq!(sub.depth, 60);
        assert_eq!(sub.absolute(), 900);
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
