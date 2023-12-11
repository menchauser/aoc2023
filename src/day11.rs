use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let image = load_input(input_path).unwrap();
    let result = calc_expanded_paths(&image, 2);
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    let image = load_input(input_path).unwrap();
    let result = calc_expanded_paths(&image, 1_000_000);
    println!("Result: {}", result);
}

fn calc_expanded_paths(image: &Vec<Vec<char>>, expansion_scale: u32) -> u64 {
    eprintln!("Input image:");
    eprintln_image(&image);
    let galaxy_coords: Vec<(u32, u32)> = image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(j, _)| (i as u32, j as u32))
        })
        .collect();
    eprintln!("Original galaxy coords: {:?}", &galaxy_coords);
    // now we find empty rows and cols
    let empty_rows: Vec<u32> = empty_rows(&image);
    let empty_cols: Vec<u32> = empty_cols(&image);
    let expanded_galaxy_coords =
        expand_coords(&galaxy_coords, &empty_rows, &empty_cols, expansion_scale);
    eprintln!("Expanded galaxy coords: {:?}", &expanded_galaxy_coords);

    // now we have to take all # pairs and calcualate the distance between them
    let mut distances: Vec<u32> = Vec::new();

    let mut count = 0;
    for i in 0..expanded_galaxy_coords.len() - 1 {
        for j in (i + 1)..expanded_galaxy_coords.len() {
            count += 1;
            let g1 = expanded_galaxy_coords[i];
            let g2 = expanded_galaxy_coords[j];
            let dist = distance(g1, g2);
            eprintln!(
                "{:02}: distance between {:?} and {:?} = {}",
                count, g1, g2, dist
            );
            distances.push(dist);
        }
    }

    distances.iter().map(|x| *x as u64).sum()
}

fn distance(g1: (u32, u32), g2: (u32, u32)) -> u32 {
    let result = (g1.0 as i32 - g2.0 as i32).abs() + (g1.1 as i32 - g2.1 as i32).abs();
    result as u32
}

fn empty_rows(image: &Vec<Vec<char>>) -> Vec<u32> {
    image
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|c| *c == '.'))
        .map(|(i, _)| i as u32)
        .collect()
}

fn empty_cols(image: &Vec<Vec<char>>) -> Vec<u32> {
    // TODO: iterators
    let mut empty_col_idxs: Vec<u32> = Vec::new();
    for j in 0..image[0].len() {
        if image.iter().all(|r| r[j] == '.') {
            empty_col_idxs.push(j as u32);
        }
    }
    empty_col_idxs
}

fn expand_coords(
    galaxy_coords: &Vec<(u32, u32)>,
    empty_rows: &Vec<u32>,
    empty_cols: &Vec<u32>,
    expansion_scale: u32,
) -> Vec<(u32, u32)> {
    // each coord's i (and j) is increased by number of empty rows (or cols) before this galaxy's coord multiplied by scale
    galaxy_coords
        .iter()
        .map(|(i, j)| {
            let rows_before = empty_rows.iter().filter(|r| *r < i).count() as u32;
            let cols_before = empty_cols.iter().filter(|c| *c < j).count() as u32;
            (
                i - rows_before + rows_before * expansion_scale,
                j - cols_before + cols_before * expansion_scale,
            )
        })
        .collect()
}

fn eprintln_image(image: &Vec<Vec<char>>) {
    for row in image {
        for c in row {
            eprint!("{}", c);
        }
        eprintln!()
    }
}

fn load_input(input_path: &Path) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|line| line.map(|l| Vec::from_iter(l.chars())))
        .collect()
}
