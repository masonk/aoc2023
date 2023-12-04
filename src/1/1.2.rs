/*
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
 Equipped with this new information, you now need to find the real first and last digit on each line. For example:
 */
use std::fs::read_to_string;

fn main() {
    let mut sum = 0;
    for line in read_to_string("src/1/1.1.input").unwrap().lines() {
        // on each line, find the first and last integer as before,
        // but now, also remember the index of the first and the last integer.
        // If a number word appears before the first index or after the last index,
        // the value of that word reokaces the integer.
        // if there is
        let mut found_first = false;
        let mut first_idx = 0;
        let mut last_idx = 0;
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for (idx, c) in line.chars().enumerate() {
            match c.to_string().parse::<u32>() {
                Ok(i) => {
                    if found_first {
                        last = i;
                        last_idx = idx;
                    } else {
                        found_first = true;
                        first = i;
                        first_idx = idx;
                        last = i;
                        last_idx = idx;
                    }
                }
                Err(_) => {}
            }
        }
        let number_word_prefix: Option<u32> = left_parse_number_word(&line[..first_idx]);
        first = number_word_prefix.unwrap_or(first);
        let number_word_suffix: Option<u32> = right_parse_number_word(&line[last_idx..]);
        last = number_word_suffix.unwrap_or(last);
        sum += (first * 10) + last
    }
    println!("{}", sum)
}

// Return the integer value of the first number word occuring in a string, or None
fn left_parse_number_word(s: &str) -> Option<u32> {
    parse_number_word(s, false)
}
fn right_parse_number_word(s: &str) -> Option<u32> {
    parse_number_word(s,  true)
}

fn parse_number_word(s: &str, reverse: bool) -> Option<u32> {
    let s_ : String = if reverse {  s.chars().rev().collect::<String>() } else { s.to_string() };
    let dict = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
  // This works for ascii but not for unicode, as we should be splitting on graphemes but are splitting on bytes
    let rdict = 
    [
        ("orez", 0),
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7), 
        ("thgie", 8), 
        ("enin", 9),
    ];


    for i in 0..s_.len() {
        for (word, val) in if reverse { rdict } else { dict } {
            if s_[i..].starts_with(word) {
                return Some(val);
            }
        }
    }
    return None;
}