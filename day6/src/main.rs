use {
    anyhow::{Error, Result},
    common::input::{from_path, list},
    std::str::FromStr,
};

fn main() -> Result<()> {
    let inputs = list(to_fish, from_path("day6/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(inputs: &[Fish]) -> Result<()> {
    let mut school = School::new(inputs.to_vec());
    school.next_days(80);
    println!("Day 6 Part 1 => {}", school.count());

    Ok(())
}

fn part2(inputs: &[Fish]) -> Result<()> {
    let mut school = FastSchool::new(inputs);
    school.next_days(256);
    println!("Day 6 Part 2 => {}", school.count());

    Ok(())
}

fn to_fish(s: String) -> Option<Fish> {
    s.parse().ok()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Fish {
    count: u32,
}

impl Fish {
    fn new(count: u32) -> Self {
        Self { count }
    }

    fn spawn(&self) -> Fish {
        Fish::new(8)
    }

    fn next_day(&mut self) -> Option<Fish> {
        if self.count == 0 {
            self.count = 6;
            Some(self.spawn())
        } else {
            self.count -= 1;
            None
        }
    }
}

impl FromStr for Fish {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let count = s.parse()?;

        Ok(Fish::new(count))
    }
}

#[derive(Debug)]
struct School {
    fishes: Vec<Fish>,
}

impl School {
    fn new(fishes: Vec<Fish>) -> Self {
        Self { fishes }
    }

    fn next_day(&mut self) {
        let mut spawn = vec![];
        self.fishes.iter_mut().for_each(|fish| {
            if let Some(fish) = fish.next_day() {
                spawn.push(fish);
            }
        });
        self.fishes.append(&mut spawn);
    }

    fn next_days(&mut self, days: u32) {
        for _ in 0..days {
            self.next_day();
            // println!("day {} => {:?}", day, self);
        }
    }

    fn count(&self) -> usize {
        self.fishes.len()
    }
}

#[derive(Debug)]
struct FastSchool {
    counters: Vec<usize>,
}

impl FastSchool {
    fn new(fishes: &[Fish]) -> Self {
        let mut counters = vec![0; 9];

        for fish in fishes {
            counters[fish.count as usize] += 1;
        }

        Self { counters }
    }

    fn next_day(&mut self) {
        let zero = self.counters[0];
        for i in 1..9 {
            self.counters[i - 1] = self.counters[i];
        }
        self.counters[8] = zero;

        self.counters[6] += zero;
    }

    fn next_days(&mut self, days: u32) {
        for _ in 0..days {
            self.next_day();
        }
    }

    fn count(&self) -> usize {
        self.counters.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_from_str() -> Result<()> {
        assert_eq!("1".parse::<Fish>()?, Fish::new(1));

        Ok(())
    }

    #[test]
    fn check_fish_next_day_no_spawn() {
        let mut fish = Fish::new(6);
        let spawn = fish.next_day();
        assert_eq!(fish.count, 5);
        assert_eq!(spawn, None);
    }

    #[test]
    fn check_fish_next_day_with_spawn() {
        let mut fish = Fish::new(0);
        let spawn = fish.next_day();
        assert_eq!(fish.count, 6);
        assert_eq!(spawn, Some(Fish::new(8)));
    }

    #[test]
    fn check_school_next_day_no_spawn() {
        let mut school = School::new(vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);
        school.next_day();
        assert_eq!(school.count(), 5);
        assert_eq!(
            school.fishes,
            vec![
                Fish::new(2),
                Fish::new(3),
                Fish::new(2),
                Fish::new(0),
                Fish::new(1),
            ]
        );
    }

    #[test]
    fn check_school_next_day_with_spawn() {
        let mut school = School::new(vec![
            Fish::new(2),
            Fish::new(3),
            Fish::new(2),
            Fish::new(0),
            Fish::new(1),
        ]);
        school.next_day();
        assert_eq!(school.count(), 6);
        assert_eq!(
            school.fishes,
            vec![
                Fish::new(1),
                Fish::new(2),
                Fish::new(1),
                Fish::new(6),
                Fish::new(0),
                Fish::new(8),
            ]
        );
    }

    #[test]
    fn check_school_next_18_days() {
        let mut school = School::new(vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);
        school.next_days(18);
        assert_eq!(school.count(), 26);
    }

    #[test]
    fn check_school_next_80_days() {
        let mut school = School::new(vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);
        school.next_days(80);
        assert_eq!(school.count(), 5934);
    }

    #[test]
    fn check_fast_school_new() {
        let school = FastSchool::new(&vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);
        assert_eq!(school.counters, vec![0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn check_fast_school_next_day() {
        let mut school = FastSchool::new(&vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);
        school.next_day();
        assert_eq!(school.count(), 5);
        assert_eq!(school.counters, vec![1, 1, 2, 1, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn check_fast_school_next_3_days() {
        let mut school = FastSchool::new(&vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);
        school.next_days(3);
        assert_eq!(school.count(), 7);
        assert_eq!(school.counters, vec![2, 1, 0, 0, 0, 1, 1, 1, 1]);
    }

    #[test]
    fn check_fast_school_next_80_days() {
        let mut school = FastSchool::new(&vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);
        school.next_days(80);
        assert_eq!(school.count(), 5934);
    }

    #[test]
    fn check_fast_school_next_256_days() {
        let mut school = FastSchool::new(&vec![
            Fish::new(3),
            Fish::new(4),
            Fish::new(3),
            Fish::new(1),
            Fish::new(2),
        ]);
        school.next_days(256);
        assert_eq!(school.count(), 26984457539);
    }
}
