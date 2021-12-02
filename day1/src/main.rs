use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

fn main() -> Result<(), Error> {
    let depths = Depths::new(false, file(Path::new("day1/test/input.txt"))?);

    let windows = depths.windows(depths.depths());
    let variances = depths.variances(&windows);
    let count = depths.count(&variances, &Variance::Increased);
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

    fn depths(&self) -> &Vec<u32> {
        &self.depths
    }

    fn windows(&self, depths: &Vec<u32>) -> Vec<u32> {
        let mut windows = Vec::new();
        let mut window = Window::new();

        for current in depths {
            window.push(Some(*current));
            if window.is_full() {
                if self.debug {
                    println!("{}", current);
                }
                windows.push(window.sum());
            }
        }

        windows
    }

    fn variances(&self, depths: &Vec<u32>) -> Vec<Variance> {
        let mut variances = Vec::new();
        let mut previous: Option<&u32> = None;

        for current in depths {
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

#[derive(Debug)]
struct Window {
    a: Option<u32>,
    b: Option<u32>,
    c: Option<u32>,
}

impl Window {
    fn new() -> Self {
        Self {
            a: None,
            b: None,
            c: None,
        }
    }

    fn push(&mut self, item: Option<u32>) {
        self.a = self.b;
        self.b = self.c;
        self.c = item;
    }

    fn sum(&self) -> u32 {
        self.a.unwrap_or(0) + self.b.unwrap_or(0) + self.c.unwrap_or(0)
    }

    fn is_full(&self) -> bool {
        self.a.map_or(false, |_| true)
            && self.b.map_or(false, |_| true)
            && self.c.map_or(false, |_| true)
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
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(
            depths.variances(depths.depths()),
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
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        let variances = depths.variances(depths.depths());

        assert_eq!(depths.count(&variances, &Variance::Increased), 7);
    }

    #[test]
    fn check_windows() {
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(
            depths.windows(depths.depths()),
            vec![607, 618, 618, 617, 647, 716, 769, 792]
        )
    }

    #[test]
    fn check_window_variances() {
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        let windows = depths.windows(depths.depths());

        assert_eq!(
            depths.variances(&windows),
            vec![
                Variance::NotApplicable,
                Variance::Increased,
                Variance::Unchanged,
                Variance::Decreased,
                Variance::Increased,
                Variance::Increased,
                Variance::Increased,
                Variance::Increased,
            ]
        )
    }

    #[test]
    fn check_window_count() {
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        let windows = depths.windows(depths.depths());
        let variances = depths.variances(&windows);

        assert_eq!(depths.count(&variances, &Variance::Increased), 5)
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
