use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let contraption = load_input(input_path).unwrap();
    let beam_energies = fill_energies(&contraption, Beam::new(0, 0, Direction::East));
    eprintln!("Resulting energy map:");
    for row in &beam_energies {
        for c in row {
            eprint!("{}", c);
        }
        eprintln!();
    }
    let result: usize = beam_energies
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum();
    println!("Result: {}", result)
}

pub fn part2(input_path: &Path) {
    let contraption = load_input(input_path).unwrap();
    let mut total_energies = Vec::<usize>::new();
    // top and bottom edge
    for j in 0..contraption[0].len() {
        let beam_energies = fill_energies(&contraption, Beam::new(0, j, Direction::South));
        total_energies.push(
            beam_energies
                .iter()
                .map(|row| row.iter().filter(|c| **c == '#').count())
                .sum(),
        );
        let beam_energies = fill_energies(
            &contraption,
            Beam::new(contraption.len() - 1, j, Direction::North),
        );
        total_energies.push(
            beam_energies
                .iter()
                .map(|row| row.iter().filter(|c| **c == '#').count())
                .sum(),
        );
    }
    // left and right edge
    for i in 0..contraption.len() {
        let beam_energies = fill_energies(&contraption, Beam::new(i, 0, Direction::East));
        total_energies.push(
            beam_energies
                .iter()
                .map(|row| row.iter().filter(|c| **c == '#').count())
                .sum(),
        );
        let beam_energies = fill_energies(
            &contraption,
            Beam::new(i, contraption[0].len() - 1, Direction::West),
        );
        total_energies.push(
            beam_energies
                .iter()
                .map(|row| row.iter().filter(|c| **c == '#').count())
                .sum(),
        );
    }
    let result = total_energies.iter().max().unwrap();
    println!("Result: {}", result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Beam {
    row: usize,
    col: usize,
    dir: Direction,
}

impl Beam {
    fn new(row: usize, col: usize, dir: Direction) -> Self {
        Self { row, col, dir }
    }

    fn with_dir(&self, dir: Direction) -> Self {
        Self { dir: dir, ..*self }
    }
}

fn fill_energies(contraption: &Vec<Vec<char>>, start_beam: Beam) -> Vec<Vec<char>> {
    let max_row = contraption.len() - 1;
    let max_col = contraption[0].len() - 1;
    // so stupid idea is:
    //  we have a vec of beams and step until all elements of list stop
    //  each beam state consists of: Direction (N/E/S/W or STOP)
    //  each beam should go its path until it finishes (in wall) or starts looping
    let mut beams = VecDeque::from([start_beam]);
    // let mut result_energies = empty_energy_template(&contraption);
    // we have a resulting plan of eneergized cells
    let mut beam_energies = empty_energy_template(&contraption);
    // and we remember which cells were passed in each directions
    let mut passes: HashSet<Beam> = HashSet::new();
    while !beams.is_empty() {
        let beam = beams.pop_front().unwrap();
        // We check that we haven't pass current point yet, and if we had we double-check that our direction is new one,
        // to avoid looping
        if beam_energies[beam.row][beam.col] == '.' {
            beam_energies[beam.row][beam.col] = '#';
            passes.insert(beam.clone());
        } else {
            if passes.contains(&beam) {
                // we are in loop, continue with next beams
                continue;
            } else {
                passes.insert(beam.clone());
            }
        }
        match contraption[beam.row][beam.col] {
            '.' => {
                // If we can step in given direction, we continue beam
                if let Some(new_beam) = try_step_beam(max_row, max_col, &beam) {
                    beams.push_back(new_beam)
                }
            }
            '\\' => {
                let new_dir = match beam.dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                };
                if let Some(new_beam) = try_step_beam(max_row, max_col, &beam.with_dir(new_dir)) {
                    beams.push_back(new_beam)
                }
            }
            '/' => {
                let new_dir = match beam.dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                if let Some(new_beam) = try_step_beam(max_row, max_col, &beam.with_dir(new_dir)) {
                    beams.push_back(new_beam)
                }
            }
            '-' => {
                match beam.dir {
                    Direction::West | Direction::East => {
                        // just continue that way
                        if let Some(new_beam) = try_step_beam(max_row, max_col, &beam) {
                            beams.push_back(new_beam)
                        }
                    }
                    _ => {
                        // or generate two new beams
                        if let Some(new_beam) =
                            try_step_beam(max_row, max_col, &beam.with_dir(Direction::West))
                        {
                            beams.push_back(new_beam)
                        }
                        if let Some(new_beam) =
                            try_step_beam(max_row, max_col, &beam.with_dir(Direction::East))
                        {
                            beams.push_back(new_beam)
                        }
                    }
                }
            }
            '|' => {
                match beam.dir {
                    Direction::North | Direction::South => {
                        // just continue that way
                        if let Some(new_beam) = try_step_beam(max_row, max_col, &beam) {
                            beams.push_back(new_beam)
                        }
                    }
                    _ => {
                        // or generate two new beams
                        if let Some(new_beam) =
                            try_step_beam(max_row, max_col, &beam.with_dir(Direction::North))
                        {
                            beams.push_back(new_beam)
                        }
                        if let Some(new_beam) =
                            try_step_beam(max_row, max_col, &beam.with_dir(Direction::South))
                        {
                            beams.push_back(new_beam)
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        // merge new energies to existing
        // merge_energies(&mut result_energies, &beam_energies);
    }
    beam_energies
}

// this structure contains list of chars with directions in which this point was passed
fn empty_energy_template(contraption: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    vec![vec!['.'; contraption[0].len()]; contraption.len()]
}

fn try_step_beam(max_row: usize, max_col: usize, beam: &Beam) -> Option<Beam> {
    match beam.dir {
        Direction::North if beam.row > 0 => Some(Beam {
            row: beam.row - 1,
            ..*beam
        }),
        Direction::East if beam.col < max_col => Some(Beam {
            col: beam.col + 1,
            ..*beam
        }),
        Direction::South if beam.row < max_row => Some(Beam {
            row: beam.row + 1,
            ..*beam
        }),
        Direction::West if beam.col > 0 => Some(Beam {
            col: beam.col - 1,
            ..*beam
        }),
        _ => None,
    }
}

fn load_input(input_path: &Path) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|line| line.map(|l| l.chars().collect()))
        .collect()
}
