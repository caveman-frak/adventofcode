use {
    anyhow::Result,
    common::{
        convert::to_u32,
        input::{from_path, list},
    },
    std::collections::HashMap,
};

fn main() -> Result<()> {
    let inputs = list(to_u32, from_path("day7/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(inputs: &[u32]) -> Result<()> {
    let cast = Cast::new(inputs);
    println!("Day 7 Part 1 => {:?}", cast.min_distance(|x| x));

    Ok(())
}

fn part2(inputs: &[u32]) -> Result<()> {
    let cast = Cast::new(inputs);
    println!("Day 7 Part 1 => {:?}", cast.min_distance(cost));

    Ok(())
}

fn diff(one: u32, other: u32) -> u32 {
    if one > other {
        one - other
    } else {
        other - one
    }
}

fn cost(distance: u32) -> u32 {
    (0u32..=distance).sum::<u32>()
}

#[derive(Debug)]
struct Cast {
    distribution: HashMap<u32, u32>,
    min: u32,
    max: u32,
}

impl Cast {
    fn new(crabs: &[u32]) -> Self {
        let mut distribution = HashMap::new();
        let mut min = u32::MAX;
        let mut max = 0u32;

        crabs.iter().for_each(|x| {
            let x = *x;
            *distribution.entry(x).or_default() += 1;
            min = min.min(x);
            max = max.max(x);
        });

        Self {
            distribution,
            min,
            max,
        }
    }

    fn distance(&self, from: u32, cost: fn(u32) -> u32) -> u32 {
        self.distribution
            .iter()
            .fold(0, |acc, (k, v)| acc + v * cost(diff(from, *k)))
    }

    fn min_distance(&self, cost: fn(u32) -> u32) -> (u32, u32) {
        let mut min_position = self.min;
        let mut min_distance = u32::MAX;

        for position in self.min..self.max {
            let distance = self.distance(position, cost);
            if distance < min_distance {
                min_distance = distance;
                min_position = position;
            }
        }
        (min_position, min_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cast() {
        let input: Vec<u32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let cast = Cast::new(&input);

        assert_eq!(cast.min, 0);
        assert_eq!(cast.max, 16);
        assert_eq!(cast.distribution.len(), 7);
        assert_eq!(cast.distribution.values().sum::<u32>(), 10u32);
    }

    #[test]
    fn check_distance() {
        let input: Vec<u32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let cast = Cast::new(&input);

        assert_eq!(cast.distance(1, |x| x), 41);
        assert_eq!(cast.distance(2, |x| x), 37);
        assert_eq!(cast.distance(3, |x| x), 39);
        assert_eq!(cast.distance(10, |x| x), 71);
    }

    #[test]
    fn check_min_position() {
        let input: Vec<u32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let cast = Cast::new(&input);

        assert_eq!(cast.min_distance(|x| x), (2, 37));
    }

    #[test]
    fn check_cost() {
        assert_eq!(cost(1), 1);
        assert_eq!(cost(2), 3);
        assert_eq!(cost(3), 6);
        assert_eq!(cost(4), 10);
        assert_eq!(cost(5), 15);
        assert_eq!(cost(10), 55);
    }

    #[test]
    fn check_min_cost() {
        let input: Vec<u32> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let cast = Cast::new(&input);

        assert_eq!(cast.min_distance(cost), (5, 168));
    }
}
