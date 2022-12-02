use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn get_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}


fn main() {
    let lines = get_lines("input.txt").unwrap();
    let mut elf_idx = 1;
    for line in lines {
        if line.unwrap() == "" {
            elf_idx += 1;
        }
    }
    println!("Number of elves: {}", elf_idx);
    let mut elf_array = vec![0; elf_idx];
    let mut elf_idx = 0;
    let lines = get_lines("input.txt").unwrap();
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
