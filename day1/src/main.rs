use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

fn main() -> Result<(), Error> {
    let depths = Depths::new(false, file(Path::new("day1/test/input.txt"))?);

    let variance = depths.variances();
    let count = depths.count(&variance, &Variance::Increased);
    println!("No of increases = {}", count);

    Ok(())
}

fn file(path: &Path) -> Result<Vec<u32>, Error> {
    Ok(inputs(BufReader::new(File::open(path)?)))
}

fn inputs<R: BufRead>(reader: R) -> Vec<u32> {
    reader
        .lines()
        .filter_map(|result| result.ok())
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[derive(Debug)]
struct Depths {
    debug: bool,
    depths: Vec<u32>,
}

impl Depths {
    fn new(debug: bool, depths: Vec<u32>) -> Self {
        Depths { debug, depths }
    }

    fn variances(&self) -> Vec<Variance> {
        let mut variances = Vec::new();
        let mut previous: Option<&u32> = None;

        for current in &self.depths {
            if self.debug {
                println!("{}", current);
            }
            variances.push(Variance::compare(previous, Some(current)));

            previous = Some(current);
        }

        variances
    }

    fn count(&self, variances: &Vec<Variance>, variance: &Variance) -> usize {
        variances
            .iter()
            .inspect(|v| self.debug(v))
            .filter(|v| v == &variance)
            .count()
    }

    fn debug(&self, variance: &&Variance) {
        if self.debug {
            println!("{:?}", variance)
        }
    }
}

#[derive(Debug, PartialEq)]
enum Variance {
    Unchanged,
    Increased,
    Decreased,
    NotApplicable,
}

impl Variance {
    fn compare(first: Option<&u32>, second: Option<&u32>) -> Self {
        match (first, second) {
            (Some(first), Some(second)) => match first.cmp(&second) {
                Ordering::Less => Variance::Increased,
                Ordering::Greater => Variance::Decreased,
                _ => Variance::Unchanged,
            },
            (_, _) => Variance::NotApplicable,
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::io::Cursor};

    #[test]
    fn check_compare_lt() {
        assert_eq!(Variance::compare(Some(&10), Some(&20)), Variance::Increased);
    }

    #[test]
    fn check_compare_gt() {
        assert_eq!(Variance::compare(Some(&20), Some(&10)), Variance::Decreased);
    }

    #[test]
    fn check_compare_eq() {
        assert_eq!(Variance::compare(Some(&10), Some(&10)), Variance::Unchanged);
    }

    #[test]
    fn check_compare_first() {
        assert_eq!(Variance::compare(None, Some(&10)), Variance::NotApplicable);
    }

    #[test]
    fn check_compare_last() {
        assert_eq!(Variance::compare(Some(&10), None), Variance::NotApplicable);
    }

    #[test]
    fn check_variance() {
        let depth = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(
            depth.variances(),
            vec![
                Variance::NotApplicable,
                Variance::Increased,
                Variance::Increased,
                Variance::Increased,
                Variance::Decreased,
                Variance::Increased,
                Variance::Increased,
                Variance::Increased,
                Variance::Decreased,
                Variance::Increased
            ]
        );
    }

    #[test]
    fn check_count() {
        let depth = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        let variances = depth.variances();

        assert_eq!(depth.count(&variances, &Variance::Increased), 7);
    }

    #[test]
    fn check_reader() {
        let buffer = Cursor::new(b"100\n200\n300");
        assert_eq!(inputs(buffer), vec![100, 200, 300]);
    }

    #[test]
    fn check_file() {
        assert_eq!(
            file(Path::new("test/test.txt")).unwrap(),
            vec![101, 201, 301]
        );
    }
}
