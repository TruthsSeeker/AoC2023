use std::{vec, collections::VecDeque, ops::Index};

use crate::utils;

pub fn part1(input_path: &str) {
    let calibrations = utils::load_file_lines(input_path);

    let mut total: u32 = 0;
    for calibration in calibrations {
        total += parse_calibration(&calibration);
    }
    println!("Total: {}", total);
}

pub fn part2(input_path: &str) {
    let calibrations = utils::load_file_lines(input_path);

    let mut total: u32 = 0;
    
    for calibration in calibrations {
        total += parse_calibration_letters(&calibration);
    }
    println!("Total: {}", total);
}

fn parse_calibration(calibration: &String) -> u32 {
    let mut value: Vec<u32> = vec![];
    for c in calibration.chars() {
        match c.to_digit(10) {
            Some(d) => value.push(d),
            None => continue,
        }
    }
    value.first().unwrap() * 10 + value.last().unwrap()
}

fn parse_calibration_letters(calibration: &String) -> u32 {
    let mut value: Vec<u32> = vec![];
    let mut buffer: VecDeque<char> = VecDeque::new();
    for c in calibration.chars() {
        if c.is_digit(10) {
            value.push(c.to_digit(10).unwrap());
            buffer.clear();
        } else {
            buffer.push_back(c);
            if let Some((v, i)) = check_buffer(&buffer) {
                value.push(v);
                buffer.drain(0..i+1);
            }
        }

    }
    value.first().unwrap() * 10 + value.last().unwrap()
}

fn check_buffer(buffer: &VecDeque<char>) -> Option<(u32, usize)> {
    let string = buffer.iter().cloned().collect::<String>();
    if let Some(i) = string.find("one") {
        return Some((1, i));
    } else if let Some(i) = string.find("two") {
        return Some((2, i));
    } else if let Some(i) = string.find("three") {
        return Some((3, i));
    } else if let Some(i) = string.find("four") {
        return Some((4, i));
    } else if let Some(i) = string.find("five") {
        return Some((5, i));
    } else if let Some(i) = string.find("six") {
        return Some((6, i));
    } else if let Some(i) = string.find("seven") {
        return Some((7, i));
    } else if let Some(i) = string.find("eight") {
        return Some((8, i));
    } else if let Some(i) = string.find("nine") {
        return Some((9, i));
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_calibration() {
        assert_eq!(parse_calibration(&String::from("asd6qllfg9")), 69);
        assert_eq!(parse_calibration(&String::from("asd4gdf9gfdakl2ad")), 42);
        assert_eq!(parse_calibration(&String::from("asdfkl;,cm3laksdfjh")), 33);
    }

    #[test]
    fn test_parse_calibration_letter() {
        assert_eq!(parse_calibration_letters(&String::from("asd6qllfg9")), 69);
        assert_eq!(parse_calibration_letters(&String::from("atwo1nine")), 29);
        assert_eq!(parse_calibration_letters(&String::from("zoneight234")), 14);
    }
}