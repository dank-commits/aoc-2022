use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug)]
struct Stacks {
    stacks: HashMap<u32, Vec<String>>
}

impl Stacks {
    fn new() -> Stacks {
        let mut stacks = HashMap::new();
        for i in 1..=9 {
            stacks.insert(i, Vec::new());
        }
        Stacks { stacks }
    }

    fn move_crates(&mut self, from: u32, to: u32, no_crates: u32) {
        let crates = self.stacks.get_mut(&from).unwrap();
        let mut crates_to_move = Vec::new();
        for _ in 0..no_crates {
            crates_to_move.push(crates.pop().unwrap());
        }
        let to_crates = self.stacks.get_mut(&to).unwrap();
        to_crates.append(&mut crates_to_move);
    }

    fn move_ordered_crates(&mut self, from: u32, to: u32, no_crates: u32) {
        let crates = self.stacks.get_mut(&from).unwrap();
        let mut crates_to_move = Vec::new();
        for _ in 0..no_crates {
            crates_to_move.push(crates.pop().unwrap());
        }
        crates_to_move.reverse();
        let to_crates = self.stacks.get_mut(&to).unwrap();
        to_crates.append(&mut crates_to_move);
    }
    
    fn display_top_box(&self) {
        for i in 1..=9 {
            let crates = self.stacks.get(&i).unwrap();
            if crates.len() > 0 {
                println!("{}: {}", i, crates[crates.len() - 1]);
            } else {
                println!("{}: ", i);
            }
        }
    }
}

fn parse_move_cmd(cmd: &str) -> (u32, u32, u32) {
    let mut parts = cmd.split_whitespace();
    let mut command_vec = Vec::new();
    while let Some(s) = parts.next() {
        match s.parse::<u32>() {
            Ok(n) => command_vec.push(n),
            Err(_) => continue
        }
    }
    if command_vec.len() != 3 {
        panic!("Invalid command: {}", cmd);
    }
    (command_vec[0], command_vec[1], command_vec[2])
}



fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut line_no = 1;
    let mut stack_vec = Vec::new();
    let mut stacks_part_one = Stacks::new();
    let mut stacks_part_two = Stacks::new();
    for line in lines {
        if line_no <= 8 {
            let mut char_no = 1;
            let line_chars = line.chars();
            let mut boxes = Vec::new();
            let mut current_str = String::new();
            for char in line_chars {
                if char_no % 4 != 0 {
                    current_str.push(char);
                    char_no += 1;
                } else {
                    boxes.push(current_str);
                    current_str = String::new();
                    char_no += 1; 
                }
            }
            stack_vec.push(boxes);
            line_no += 1; 
        } else if line_no == 9 {
            stack_vec.reverse();
            for stack in &stack_vec {
                for (i, box_str) in stack.iter().enumerate() {
                    if box_str != "   " {
                        stacks_part_one.stacks.get_mut(&(i as u32 + 1)).unwrap().push(box_str.to_string());
                        stacks_part_two.stacks.get_mut(&(i as u32 + 1)).unwrap().push(box_str.to_string());
                    }
                }
            }
            line_no += 1;
            
        } else if line_no == 10 {
            line_no += 1;
            continue;
        } else {
            println!("Command: {}", line);
            let (no_crates, from, to) = parse_move_cmd(&line);
            stacks_part_one.move_crates(from, to, no_crates);
            stacks_part_two.move_ordered_crates(from, to, no_crates);
            line_no += 1;
        }
    }
    println!("Part 1:");
    stacks_part_one.display_top_box();
    println!("\nPart 2:");
    stacks_part_two.display_top_box();
}
