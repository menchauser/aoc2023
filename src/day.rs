use std::path::Path;

pub trait Day {
    fn part1(&self, input_path: &Path);
    fn part2(&self, input_path: &Path);
}
