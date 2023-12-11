use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let image = load_input(input_path).unwrap();
    eprintln!("Input image:");
    eprintln_image(&image);
    // now expand rows and cols
    // to expand cols let's first calculate which calls are empty
    let expanded_image = expand_image(&image);
    eprintln!("Expanded rows and cols:");
    eprintln_image(&expanded_image);
    // now we have to take all # pairs and calcualate the distance between them
    let galaxy_coords: Vec<(u32, u32)> = expanded_image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(j, _)| (i as u32, j as u32))
        })
        .collect();
    eprintln!("Found galaxies: {:?}", galaxy_coords);
    // now we just have to find min_distance between two galaxies
    //   we take that row1 <= row2 and col1 <= col2
    //   then the distance is: (row2 - row1) + (col2 - col1)
    let mut distances: Vec<u32> = Vec::new();

    let mut count = 0;
    for i in 0..galaxy_coords.len() - 1 {
        for j in (i + 1)..galaxy_coords.len() {
            count += 1;
            let g1 = galaxy_coords[i];
            let g2 = galaxy_coords[j];
            let dist = distance(g1, g2);
            eprintln!(
                "{:02}: distance between {:?} and {:?} = {}",
                count, g1, g2, dist
            );
            distances.push(dist);
        }
    }

    let result: u32 = distances.iter().sum();
    println!("Result: {}", result)
}

pub fn part2(input_path: &Path) {
    let image = load_input(input_path).unwrap();
    let result = calc_expanded_paths(&image, 10);
    println!("Result: {}", result);
}

fn calc_expanded_paths(image: &Vec<Vec<char>>, expansion_scale: u32) -> u32 {
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
    eprintln!("Original galaxy coords: {:?}", galaxy_coords);
    // now expand rows and cols
    // to expand cols let's first calculate which calls are empty
    let expanded_image = expand_image(&image);
    eprintln!("Expanded rows and cols:");
    eprintln_image(&expanded_image);
    // now we have to take all # pairs and calcualate the distance between them
    let galaxy_coords: Vec<(u32, u32)> = expanded_image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(j, _)| (i as u32, j as u32))
        })
        .collect();
    eprintln!("Found galaxies: {:?}", galaxy_coords);
    // now we just have to find min_distance between two galaxies
    //   we take that row1 <= row2 and col1 <= col2
    //   then the distance is: (row2 - row1) + (col2 - col1)
    let mut distances: Vec<u32> = Vec::new();

    let mut count = 0;
    for i in 0..galaxy_coords.len() - 1 {
        for j in (i + 1)..galaxy_coords.len() {
            count += 1;
            let g1 = galaxy_coords[i];
            let g2 = galaxy_coords[j];
            let dist = distance(g1, g2);
            eprintln!(
                "{:02}: distance between {:?} and {:?} = {}",
                count, g1, g2, dist
            );
            distances.push(dist);
        }
    }

    distances.iter().sum()
}

fn distance(g1: (u32, u32), g2: (u32, u32)) -> u32 {
    let result = (g1.0 as i32 - g2.0 as i32).abs() + (g1.1 as i32 - g2.1 as i32).abs();
    result as u32
}

fn is_empty_row(image: &Vec<Vec<char>>, row_idx: usize) -> bool {
    let row = &image[row_idx];
    row.iter().all(|c| *c == '.')
}

fn expand_image(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // to expand cols let's first calculate which calls are empty
    let mut empty_col_idxs = Vec::new();
    for j in 0..image[0].len() {
        if image.iter().all(|r| r[j] == '.') {
            empty_col_idxs.push(j);
        }
    }
    // now update and insert
    let mut expanded_image: Vec<Vec<char>> = Vec::new();
    for i in 0..image.len() {
        let mut new_row = Vec::new();
        for j in 0..image[i].len() {
            new_row.push(image[i][j]);
            if empty_col_idxs.contains(&j) {
                new_row.push('.');
            }
        }
        expanded_image.push(new_row.clone());
        if is_empty_row(&image, i) {
            expanded_image.push(new_row.clone());
        }
    }
    expanded_image
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
