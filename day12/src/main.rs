use {
    anyhow::Result,
    common::{
        convert::to_vec_u32,
        input::{from_path, list},
    },
    std::collections::{HashMap, HashSet},
};

fn main() -> Result<()> {
    let inputs = list(to_vec_u32, from_path("day12/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(_inputs: &[Vec<u32>]) -> Result<()> {
    println!("Day 12 Part 1 => {:?}", "");

    Ok(())
}

fn part2(_inputs: &[Vec<u32>]) -> Result<()> {
    println!("Day 12 Part 2 => {}", "");

    Ok(())
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Copy)]
struct Cave<'a> {
    id: &'a str,
    small: bool,
}

impl<'a> Cave<'a> {
    fn new(id: &'a str) -> Self {
        let small = id.chars().all(|c| c.is_lowercase());
        Self { id, small }
    }

    fn is_small(&self) -> bool {
        self.small
    }
}

#[derive(Debug, Default)]
struct Caves<'a> {
    start: Option<Cave<'a>>,
    end: Option<Cave<'a>>,
    links: HashMap<Cave<'a>, HashSet<Cave<'a>>>,
}

impl<'a> Caves<'a> {
    fn new(routes: Vec<Vec<Cave<'a>>>) -> Self {
        let mut caves = Self::default();

        for route in routes {
            caves.add(route[0], route[1]);
            caves.add(route[1], route[0]);
        }
        caves
    }

    fn add(&mut self, cave: Cave<'a>, link: Cave<'a>) {
        if self.start.is_none() && cave.id == "start".to_string() {
            self.start = Some(cave)
        }
        if self.end.is_none() && cave.id == "end".to_string() {
            self.end = Some(cave)
        }
        self.links.entry(cave).or_default().insert(link);
    }

    fn routes(&self) -> Vec<Vec<Cave<'_>>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cave() {
        assert_eq!(Cave::new("aa").is_small(), true);
        assert_eq!(Cave::new("AA").is_small(), false);
    }

    #[test]
    fn check_caves() {
        let caves = Caves::new(vec![vec![Cave::new("aa"), Cave::new("end")]]);

        assert_eq!(caves.start, None);
        assert_eq!(caves.end, Some(Cave::new("end")));
        assert_eq!(
            caves.links,
            HashMap::from([
                (Cave::new("aa"), HashSet::from([Cave::new("end")])),
                (Cave::new("end"), HashSet::from([Cave::new("aa")])),
            ])
        );
    }

    fn test_input<'a>() -> Caves<'a> {
        let routes = vec![
            vec![Cave::new("start"), Cave::new("A")],
            vec![Cave::new("start"), Cave::new("b")],
            vec![Cave::new("A"), Cave::new("c")],
            vec![Cave::new("A"), Cave::new("b")],
            vec![Cave::new("b"), Cave::new("d")],
            vec![Cave::new("A"), Cave::new("end")],
            vec![Cave::new("b"), Cave::new("end")],
        ];
        Caves::new(routes)
    }

    fn test_caves<'a>(ids: &'a str) -> Vec<Cave<'a>> {
        ids.split(',').map(|s| Cave::new(s)).collect()
    }

    #[test]
    fn check_input() {
        let caves = test_input();

        assert_eq!(caves.start, Some(Cave::new("start")));
        assert_eq!(caves.end, Some(Cave::new("end")));
        println!("{:#?}", caves.links);
        assert_eq!(
            caves.links,
            HashMap::from([
                (
                    Cave::new("start"),
                    HashSet::from([Cave::new("A"), Cave::new("b")])
                ),
                (
                    Cave::new("end"),
                    HashSet::from([Cave::new("A"), Cave::new("b")])
                ),
                (
                    Cave::new("A"),
                    HashSet::from([
                        Cave::new("start"),
                        Cave::new("end"),
                        Cave::new("b"),
                        Cave::new("c"),
                    ])
                ),
                (
                    Cave::new("b"),
                    HashSet::from([
                        Cave::new("start"),
                        Cave::new("end"),
                        Cave::new("A"),
                        Cave::new("d"),
                    ])
                ),
                (Cave::new("c"), HashSet::from([Cave::new("A")])),
                (Cave::new("d"), HashSet::from([Cave::new("b")])),
            ])
        );
    }

    #[test]
    fn check_routes() {
        let caves = test_input();
        let routes = caves.routes();

        assert_eq!(routes.len(), 10);
        assert_eq!(
            routes,
            vec![
                test_caves("start,A,b,A,c,A,end"),
                test_caves("start,A,b,A,end"),
                test_caves("start,A,b,end"),
                test_caves("start,A,c,A,b,A,end"),
                test_caves("start,A,c,A,b,end"),
                test_caves("start,A,c,A,end"),
                test_caves("start,A,end"),
                test_caves("start,b,A,c,A,end"),
                test_caves("start,b,A,end"),
                test_caves("start,b,end"),
            ]
        )
    }
}
