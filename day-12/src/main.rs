use util::*;

struct Puzzle {
    map: HashMap<String, Vec<String>>,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::<_, Vec<_>>::new();

        for line in s.trim().lines() {
            let (l, r) = line.trim().split_once('-').context("Invalid path")?;

            map.entry(l.to_owned()).or_default().push(r.to_owned());
            map.entry(r.to_owned()).or_default().push(l.to_owned());
        }

        Ok(Self { map })
    }
}

impl Puzzle {
    fn run(&self, visited: &mut HashSet<String>, mut double_visit: bool, name: &str) -> usize {
        if name == "end" {
            return 1;
        }

        let is_double_visit = if visited.contains(name) {
            if double_visit || name == "start" {
                return 0;
            }

            double_visit = true;
            true
        } else {
            false
        };

        let dirs = if let Some(dirs) = self.map.get(name) {
            dirs
        } else {
            return 0;
        };

        if name == name.to_lowercase() {
            visited.insert(name.to_string());
        }

        let mut total = 0;

        for dir in dirs.iter() {
            total += self.run(visited, double_visit, dir);
        }

        if !is_double_visit {
            visited.remove(name);
        }

        total
    }

    fn part_1(&self) -> usize {
        let mut visited = HashSet::new();

        self.run(&mut visited, true, "start")
    }

    fn part_2(&self) -> usize {
        let mut visited = HashSet::new();

        self.run(&mut visited, false, "start")
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2: {}", puzzle.part_2());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1(), 226);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2(), 3509);
        Ok(())
    }
}
