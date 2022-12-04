use std::fs::File;
use std::io::{BufReader, BufRead, Seek};

fn split_line(line: &str) -> (&str, &str) {
    let line_len = line.len();
    let split = line.split_at(line_len / 2);
    split
}

fn find_common_chars(str_one: &str, str_two: &str) -> Option<char> {
    let mut common_chars = Vec::new();
    let str_1_chars = str_one.chars();
    for char in str_two.chars() {
        if str_1_chars.clone().any(|c| c == char) {
            let is_present = common_chars.iter().any(|c| c == &char);
            if !is_present {
                common_chars.push(char);
            }
        }
    }
    match common_chars.len() {
        0 => None,
        1 => Some(common_chars[0]),
        _ => {
            println!("Found more than one common char: {:?}", common_chars);
            panic!("More than one common char found")
        },
    }
}

fn find_group_char(bag_one: &str, bag_two: &str, bag_3: &str) -> Option<char> {
    let mut common_chars_one = Vec::new();
    let mut common_chars_two = Vec::new();
    let bag_1_chars = bag_one.chars();
    for char in bag_two.chars() {
        if bag_1_chars.clone().any(|c| c == char) {
            let is_present = common_chars_one.iter().any(|c| c == &char);
            if !is_present {
                common_chars_one.push(char);
            }
        }
    }
    let chars_string = common_chars_one.iter().collect::<String>();
    for char in bag_3.chars() {
        if chars_string.chars().any(|c| c == char) {
            let is_present = common_chars_two.iter().any(|c| c == &char);
            if !is_present {
                common_chars_two.push(char);
            }
        }
    }
    match common_chars_two.len() {
        0 => None,
        1 => Some(common_chars_two[0]),
        _ => {
            println!("Found more than one common char: {:?}", common_chars_two);
            panic!("More than one common char found")
        },
    }
}

fn get_prio(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Unknown char: {}", c),
    }
}


fn main() {
    let mut file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines = reader.lines();
    let mut prio_sum = 0;
    for line in lines {
        let line = line.unwrap();
        let (left, right) = split_line(&line);
        let common_char = find_common_chars(left, right);
        match common_char {
            Some(c) => {
                prio_sum += get_prio(c);
            },
            None => println!("No common char found"),
        }
    }
    println!("Line Priority sum: {}", prio_sum);
    file.rewind().unwrap();
    prio_sum = 0;
    let reader = BufReader::new(&file);
    let lines = reader.lines();
    let mut group = Vec::new();
    for line in lines {
        let line = line.unwrap();
        group.push(line);
        if group.len() == 3 {
            let common_char = find_group_char(&group[0], &group[1], &group[2]);
            match common_char {
                Some(c) => {
                    prio_sum += get_prio(c);
                },
                None => println!("No common char found"),
            }
            group.clear();
        }
    }
    println!("Group Priority sum: {}", prio_sum);
}
