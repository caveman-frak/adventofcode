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
    println!("Day 9 Part 1 => {:?}", inputs.len());

    Ok(())
}

fn part2(inputs: &[Vec<u32>]) -> Result<()> {
    println!("Day 9 Part 2 => {:?}", inputs.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_inputs() -> Result<()> {
        let inputs = list(to_vec_u32, from_path("test/test.txt")?);

        assert_eq!(inputs.len(), 5);
        assert_eq!(inputs[0], vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0]);

        Ok(())
    }
}
