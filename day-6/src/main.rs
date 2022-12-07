fn find_markers(input: &str) -> (u32, u32) {
    let mut character_on = 0;
    let mut active_sequence = Vec::new();
    let mut first_packet_mark: u32 = 0;
    let mut first_packet_found = false;
    let mut first_msg_mark: u32 = 0;
    for char in input.chars() {
        if active_sequence.contains(&char) {
            let idx = active_sequence
                .iter()
                .position(|&x| x == char)
                .unwrap();
            active_sequence = active_sequence[idx + 1..].to_vec();
            active_sequence.push(char);
            character_on += 1;
        } else {
            active_sequence.push(char);
            character_on += 1;
        }
        if active_sequence.len() == 4 && !first_packet_found {
            first_packet_mark = character_on;
            first_packet_found = true;
        }
        if active_sequence.len() == 14 {
            first_msg_mark = character_on;
            break;
        }
    }
    (first_packet_mark, first_msg_mark)
}

fn main() {
    let input = include_str!("../input.txt");
    let (sequence_idx, msg_idx) = find_markers(input);
    println!("Sequence index: {} msg index: {}",sequence_idx, msg_idx);
}
