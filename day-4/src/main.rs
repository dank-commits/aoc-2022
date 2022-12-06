use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Section {
    min: u32,
    max: u32,
}

impl Section {
    fn from_str_range(range: &str) -> Section {
        let mut parts = range.split('-');
        let min = parts.next().unwrap().parse().unwrap();
        let max = parts.next().unwrap().parse().unwrap();
        Section { min, max }
    }
    
    fn contains_other(&self, other: &Section) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps_other(&self, other: &Section) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}




fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut contain_counter = 0;
    let mut overlap_counter = 0;
    for line in lines {
        let line = line.unwrap();
        let (elf_one, elf_two) = line.split_at(line.find(',').unwrap());
        let elf_two = elf_two.replace(",", "");
        let sect_one = Section::from_str_range(elf_one);
        let sect_two = Section::from_str_range(&elf_two);
        if sect_one.contains_other(&sect_two) {
            contain_counter += 1;
        } else if sect_two.contains_other(&sect_one) {
            contain_counter += 1;
        }
        if sect_one.overlaps_other(&sect_two) {
            overlap_counter += 1;
        } else if sect_two.overlaps_other(&sect_one) {
            overlap_counter += 1;
        }
    }
    println!("{} sections contain other sections", contain_counter);
    println!("{} sections overlap other sections", overlap_counter);
}
