use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

fn count_elves(file: &File) -> usize {
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut elf_idx = 1;
    for line in lines {
        if line.unwrap() == "" {
            elf_idx += 1;
        }
    }
    elf_idx
}

fn get_cals(file: &File, elf_idx: usize) {
    let mut elf_array = vec![0; elf_idx];
    let mut elf_idx = 0;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            elf_idx += 1;
        } else {
            let cals = line.parse::<i32>().unwrap();
            elf_array[elf_idx] += cals;
        }
    }
    let max = elf_array.iter().max().unwrap();
    println!("Max: {}", max);
    elf_array.sort();
    elf_array.reverse();
    let top_three_sum = elf_array[0] + elf_array[1] + elf_array[2];
    println!("Top three sum: {}", top_three_sum);

}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let elf_idx = count_elves(&file);
    println!("Number of elves: {}", elf_idx);
    file.rewind().unwrap();
    get_cals(&file, elf_idx);
}
