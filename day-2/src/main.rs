use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

#[derive(Debug, PartialEq, Clone)]
pub enum MatchResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

#[derive(Debug, Clone)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub trait Throw {
    fn throw(&self, other: &Hand) -> MatchResult;
    fn back_calc(&self, result: MatchResult) -> Hand;
}

impl Throw for Hand {
    fn throw(&self, other: &Hand) -> MatchResult {
        match (self, other) {
            (Hand::Rock, Hand::Scissors) => MatchResult::Win,
            (Hand::Paper, Hand::Rock) => MatchResult::Win,
            (Hand::Scissors, Hand::Paper) => MatchResult::Win,
            (Hand::Rock, Hand::Paper) => MatchResult::Loss,
            (Hand::Paper, Hand::Scissors) => MatchResult::Loss,
            (Hand::Scissors, Hand::Rock) => MatchResult::Loss,
            _ => MatchResult::Draw,
        }
    }
    fn back_calc(&self, result: MatchResult) -> Hand {
        if result == MatchResult::Draw {
            return self.clone();
        }
        match (self, result) {
            (Hand::Rock, MatchResult::Win) => Hand::Paper,
            (Hand::Paper, MatchResult::Win) => Hand::Scissors,
            (Hand::Scissors, MatchResult::Win) => Hand::Rock,
            (Hand::Rock, MatchResult::Loss) => Hand::Scissors,
            (Hand::Paper, MatchResult::Loss) => Hand::Rock,
            (Hand::Scissors, MatchResult::Loss) => Hand::Paper,
            _ => unreachable!()
        }
    }
}

fn calc_total_score_hand(file: &File) -> i32 {
    let reader = BufReader::new(file);
    let mut total_score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let outcomes = line.split(" ").collect::<Vec<&str>>();
        let opponent_hand = match outcomes[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        };
        let my_hand = match outcomes[1] {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        };
        let game_result = my_hand.throw(&opponent_hand);
        let game_score = game_result as i32 + my_hand as i32;
        total_score += game_score;
    }
    return total_score;
}

fn calc_total_score_result(file: &File) -> i32 {
    let reader = BufReader::new(file);
    let mut total_score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let outcomes = line.split(" ").collect::<Vec<&str>>();
        let opponent_hand = match outcomes[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        };
        let game_result = match outcomes[1] {
            "Z" => MatchResult::Win,
            "Y" => MatchResult::Draw,
            "X" => MatchResult::Loss,
            _ => panic!("Invalid result"),
        };
        let result_clone = game_result.clone();
        let my_hand = opponent_hand.back_calc(game_result);
        let game_score = result_clone as i32 + my_hand as i32;
        total_score += game_score;
    }
    return total_score;
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let guide_score = calc_total_score_hand(&file);
    println!("Guide score - Opponent Hand: {}", guide_score);
    file.rewind().unwrap();
    let guide_score = calc_total_score_result(&file);
    println!("Guide score - Result: {}", guide_score);
}
