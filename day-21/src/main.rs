use util::*;

#[derive(Clone, Debug)]
struct Puzzle {
    players: Vec<Player>,
    rolls: usize,
    turn: usize,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let players = s.trim().try_from_lines()?;

        Ok(Self {
            players,
            rolls: 0,
            turn: 0,
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Player {
    position: usize,
    score: usize,
}

impl FromStr for Player {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, r) = s.trim().split_once(':').context("Missing position")?;
        let position = usize::from_str(r.trim())? - 1;

        Ok(Self { position, score: 0 })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Quantum {
    players: Vec<Player>,
    turn: usize,
}

impl Quantum {
    fn next(&self) -> Self {
        Self {
            players: self.players.clone(),
            turn: (self.turn + 1) % self.players.len(),
        }
    }

    fn play(&self, rolls: &[usize; 10], games: &mut HashMap<Quantum, Vec<usize>>) -> Vec<usize> {
        if let Some(scores) = games.get(self) {
            return scores.clone();
        }

        let mut counts = vec![0usize; self.players.len()];

        for (roll, count) in rolls.iter().enumerate().skip(3) {
            let mut game = self.clone();
            let mut player = &mut game.players[self.turn];

            player.position = (player.position + roll) % 10;
            player.score += player.position + 1;

            if player.score >= 21 {
                counts[self.turn] += count;
                continue;
            }

            for (i, n) in game.next().play(rolls, games).into_iter().enumerate() {
                counts[i] += count * n;
            }
        }

        games.insert(self.clone(), counts.clone());
        counts
    }
}

impl From<Puzzle> for Quantum {
    fn from(puzzle: Puzzle) -> Self {
        Self {
            players: puzzle.players,
            turn: 0,
        }
    }
}

impl Puzzle {
    fn roll(&mut self) -> usize {
        let roll = (self.rolls % 100) + 1;
        self.rolls += 1;
        roll
    }

    fn part_1(&self) -> usize {
        let mut puzzle = self.clone();

        loop {
            let roll = puzzle.roll() + puzzle.roll() + puzzle.roll();
            let i = puzzle.turn % puzzle.players.len();
            let player = &mut puzzle.players[i];

            player.position = (player.position + roll) % 10;
            player.score += player.position + 1;
            puzzle.turn += 1;

            if player.score >= 1000 {
                break;
            }
        }

        let player = &puzzle.players[puzzle.turn % 2];

        player.score * puzzle.rolls
    }

    fn part_2(&self) -> Result<usize, Error> {
        let rolls = (1usize..=3)
            .flat_map(|a| (1usize..=3).map(move |b| (a, b)))
            .flat_map(|(a, b)| (1usize..=3).map(move |c| a + b + c))
            .fold([0; 10], |mut acc, v| {
                acc[v] += 1;
                acc
            });

        let mut games = HashMap::new();

        Quantum::from(self.clone())
            .play(&rolls, &mut games)
            .into_iter()
            .max()
            .context("No players")
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2: {}", puzzle.part_2()?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"
        Player 1 starting position: 4
        Player 2 starting position: 8
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1(), 739785);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2()?, 444356092776315);
        Ok(())
    }
}
