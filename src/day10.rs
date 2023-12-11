use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    let (s_row, s_col) = find_start(&input);
    eprintln!("Found S position: {}, {}", &s_row, &s_col);
    let path = find_path(&input);
    eprintln!("Full path: {:?}", &path);
    println!("Result: {}", path.len() / 2);
}

pub fn part2(input_path: &Path) {
    // the idea is to use scan-line polygon fill
    // we go in horizontal line until we encounter 'loop'
    // we have a flag which signifies that we are within polygon
    // ever time we cross a border, we invert this flag. if it is enabled - we are in polygon
    // for example let's go with one of the lines:
    // .|..|.|..|.
    // f|tt|f|tt|f
    // we need to remember the whole loop path to achieve that
    let input = load_input(input_path).unwrap();
    let path = find_path(&input);
    eprintln!("Full path: {:?}", path);
    // now let's group borders by row: in map key is row number and value is list of columns at which borders are present
    let mut loop_borders: HashMap<usize, Vec<(usize, char)>> = HashMap::with_capacity(input.len());
    for (row, col, c) in path {
        loop_borders
            .entry(row)
            .or_insert_with(|| Vec::with_capacity(1))
            .push((col, c));
    }
    eprintln!("Collected loop borders map: {:?}", &loop_borders);
    let mut result: usize = 0;

    // we should scan each row for loop elements
    // for each row
    for (row, col_data) in loop_borders {
        let row_line = String::from_iter(&input[row]);
        eprintln!("Scan row {:02}: {}", row, row_line);
        let mut borders = col_data;
        borders.sort_by_key(|x| x.0);
        let mut flag = false;
        // I scan the whole row left to right
        for j in 0..row_line.len() {
            // if current pipe is in the loop
            if let Some((col, pipe)) = borders.iter().find(|(col, _)| *col == j) {
                match pipe {
                    // hit wall - switch flag
                    '|' | 'F' | '7' | 'J' | 'L' | 'S' => {
                        flag = !flag;
                    }
                    _ => {} // do nothing
                }
            } else {
                if flag && input[row][j] == '.' {
                    result += 1
                }
            }
        }
        // for i in 1..borders.len() {
        //     flag = !flag;
        //     if flag {
        //         let inside_tiles = borders[i] - borders[i - 1] - 1;
        //         if inside_tiles != 0 {
        //             eprintln!(
        //                 "Found {} at row {} ({}..{})",
        //                 inside_tiles,
        //                 row,
        //                 borders[i - 1],
        //                 borders[i]
        //             );
        //         }
        //         result += inside_tiles;
        //     }
        // }
    }
    println!("Result: {}", result);
}

fn find_start(input: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in input.iter().enumerate() {
        let found = row.iter().enumerate().find(|(_, c)| **c == 'S');
        if found.is_some() {
            return (i, found.unwrap().0);
        }
    }
    unreachable!()
}

fn find_path(input: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
    let (s_row, s_col) = find_start(input);
    let s_connections = connections(&input, s_row, s_col, &all_directions());
    eprintln!("S connections: {:?}", s_connections);
    if s_connections.len() != 2 {
        panic!(
            "Unexpected: S should have 2 compatible connections, but has {}!",
            s_connections.len()
        );
    }
    let mut full_path: Vec<(usize, usize, char)> = vec![(s_row, s_col, 'S')];
    let random_step = s_connections.iter().next().unwrap();
    let mut direction: Direction = *random_step.0;
    let mut curr_pipe: char = *random_step.1;
    let mut next_row = s_row;
    let mut next_col = s_col;
    while curr_pipe != 'S' {
        let (d_row, d_col) = delta(direction);
        next_row = ((next_row as i16) + d_row) as usize;
        next_col = ((next_col as i16) + d_col) as usize;
        full_path.push((next_row, next_col, curr_pipe));
        // eprintln!(
        //     "Chosen to connect on {} at {}, {}",
        //     curr_pipe, next_row, next_col
        // );
        // we should build a list of directions without previous step
        let next_dirs = directions_from(direction);
        let next_conns = connections(&input, next_row, next_col, &next_dirs);
        // eprintln!("New connections: {:?}", next_conns);
        if next_conns.len() != 1 {
            panic!("We have more than 1 next connection!");
        }
        let next_step = next_conns.iter().next().unwrap();
        direction = *next_step.0;
        curr_pipe = *next_step.1;
    }
    full_path
}

// Check available directions from the pipe reached by direction d.
// So, if pipe were reached in northern direction, we should ignore south
fn directions_from(d: Direction) -> Vec<Direction> {
    let anti_d = match d {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    };
    all_directions()
        .iter()
        .filter(|d| **d != anti_d)
        .cloned()
        .collect()
}

// Get all possible steps from given position in given directions
fn connections(
    field: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    directions: &[Direction],
) -> HashMap<Direction, char> {
    if field[row][col] == '.' {
        eprintln!("Unexpected: search for direction from '.'");
        return HashMap::new();
    }
    // let's restart this Idea again
    // so we do have a character at position row/col. I want to
    // 1. Get an intersection of available directions from that char with given directions
    // From which pipes we could go in which directions
    let pipe_directions: HashMap<char, Vec<Direction>> = HashMap::from([
        ('|', vec![Direction::North, Direction::South]),
        ('-', vec![Direction::West, Direction::East]),
        ('L', vec![Direction::North, Direction::East]),
        ('J', vec![Direction::West, Direction::North]),
        ('7', vec![Direction::West, Direction::South]),
        ('F', vec![Direction::South, Direction::East]),
        ('S', all_directions()),
    ]);
    let curr_pipe = field[row][col];
    let possible_directions: Vec<(Direction, char)> = pipe_directions[&curr_pipe]
        .iter()
        .filter(|d| directions.contains(d))
        // we peek in each direction and collect positive values with related directions
        .filter_map(|d| peek(&field, row, col, *d).map(|c| (*d, c)))
        // TODO: remove this hack in separate step
        .filter(|(_, c)| *c != '.')
        .collect();
    // now we have a list of possible neighbouring chars
    // TODO: the right thing to do is to check that characters in that direction are compatible with us,
    //   but for perfect loop we can skip it

    // 2. Make sure that characters in that directions are the ones which allowed for pipe to continue
    // so (East, '-') is ok, but (West, '7') is not
    // this is the map from direction to pipe, which can be connected to us on that direction
    let direction_connectors: HashMap<Direction, Vec<char>> = HashMap::from([
        (Direction::North, vec!['|', '7', 'F', 'S']),
        (Direction::East, vec!['-', '7', 'J', 'S']),
        (Direction::South, vec!['|', 'J', 'L', 'S']),
        (Direction::West, vec!['-', 'L', 'F', 'S']),
    ]);
    let actual_connections: Vec<(Direction, char)> = possible_directions
        .iter()
        .filter(|(d, c)| direction_connectors[d].contains(c))
        .cloned()
        .collect();
    let mut result: HashMap<Direction, char> = HashMap::new();
    for (d, c) in actual_connections {
        result.insert(d, c);
    }
    return result;
}

// Return an adjacent character to the given position according to direction
fn peek(field: &Vec<Vec<char>>, row: usize, col: usize, direction: Direction) -> Option<char> {
    let step = try_step(field.len(), field[0].len(), row, col, direction);
    step.map(|(new_row, new_col)| field[new_row][new_col])
}

fn try_step(
    max_rows: usize,
    max_cols: usize,
    row: usize,
    col: usize,
    direction: Direction,
) -> Option<(usize, usize)> {
    if direction == Direction::North && row == 0 {
        return None;
    }
    if direction == Direction::East && col == max_cols - 1 {
        return None;
    }
    if direction == Direction::South && row == max_rows - 1 {
        return None;
    }
    if direction == Direction::West && col == 0 {
        return None;
    }
    match direction {
        Direction::North => {
            if row == 0 {
                None
            } else {
                Some((row - 1, col))
            }
        }
        Direction::East => {
            if col == max_cols - 1 {
                None
            } else {
                Some((row, col + 1))
            }
        }
        Direction::South => {
            if row == max_rows - 1 {
                None
            } else {
                Some((row + 1, col))
            }
        }
        Direction::West => {
            if col == 0 {
                None
            } else {
                Some((row, col - 1))
            }
        }
    }
}

/**
 * The enum parameters are deltas by
 */
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn delta(d: Direction) -> (i16, i16) {
    match d {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    }
}

fn all_directions() -> Vec<Direction> {
    vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
}

// Load 2d slice
fn load_input(input_path: &Path) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    lines
        .map(|line| line.map(|l| Vec::from_iter(l.chars())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn peek_is_correct() {
        let input = load_input(Path::new("day102.test")).unwrap();
        // We know that S resides at (2, 0)
        assert_eq!(Some('.'), peek(&input, 2, 0, Direction::North));
        assert_eq!(Some('J'), peek(&input, 2, 0, Direction::East));
        assert_eq!(Some('|'), peek(&input, 2, 0, Direction::South));
        assert_eq!(None, peek(&input, 2, 0, Direction::West));
    }

    #[test]
    fn test_no_possible_directions() {
        let input = load_input(Path::new("day102.test")).unwrap();
        let result = connections(&input, 0, 0, &all_directions());
        eprintln!("{:?}", &result);
        assert!(result.is_empty());
    }

    #[test]
    fn test_good_directions() {
        let input = load_input(Path::new("day102.test")).unwrap();
        let result1 = connections(&input, 2, 0, &all_directions());
        assert_eq!(2, result1.len());
        assert!(result1.contains_key(&Direction::East));
        assert!(result1.contains_key(&Direction::South));
        assert_eq!('J', result1[&Direction::East]);
        assert_eq!('|', result1[&Direction::South]);
        let result2 = connections(&input, 3, 2, &all_directions());
        assert_eq!(2, result2.len());
        assert!(result2.contains_key(&Direction::East));
        assert!(result2.contains_key(&Direction::West));
        assert_eq!('-', result2[&Direction::East]);
        assert_eq!('F', result2[&Direction::West]);
    }

    #[test]
    fn test_compatible_connections() {
        let input = load_input(Path::new("day101.test")).unwrap();
        let result = connections(&input, 1, 1, &all_directions());
        // we need to make sure that only compatible pipes are returned
        assert_eq!(2, result.len());
        assert!(result.contains_key(&Direction::East));
        assert!(result.contains_key(&Direction::South));
        assert_eq!('-', result[&Direction::East]);
        assert_eq!('|', result[&Direction::South]);
    }

    #[test]
    fn test_anti_directions() {
        let result = directions_from(Direction::North);
        assert_eq!(3, result.len());
        assert!(result.contains(&Direction::East));
        assert!(result.contains(&Direction::North));
        assert!(result.contains(&Direction::West));
    }
}
