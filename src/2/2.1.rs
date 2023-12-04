use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let mut sum = 0;
    for line in read_to_string("src/2/2.input").unwrap().lines() {
        let sample = line.parse::<Sample>().unwrap();
        if is_possible(&sample, (12, 13, 14)) {
            sum += sample.id;
        }
    }
    println!("{}", sum);
}

fn is_possible(sample: &Sample, maxes: (u32, u32, u32)) -> bool {
    for v in &sample.counts {
        if v.0 > maxes.0 || v.1 > maxes.1 || v.2 > maxes.2 {
            return false;
        }
    }
    true
}

#[derive(Debug, PartialEq)]
struct Sample {
    id: u32,
    counts: Vec<(u32, u32, u32)> // (red, green, blue)
}

#[derive(Debug, PartialEq, Eq)]
enum SampleParseError {
    NoId,
    UnexpectedFormat
}

impl FromStr for Sample {
    type Err = SampleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, body) = s.split_once(": ").unwrap();
        let id = header.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();

        let entries = body.split("; ");

        let mut counts: Vec<(u32, u32, u32)> = vec!();

        for e in entries {
            let mut red = 0u32;
            let mut blue = 0u32;
            let mut green = 0u32;
            for color in e.split(", ") {
                if color.ends_with(" red") {
                    red = color.strip_suffix(" red").unwrap().parse::<u32>().unwrap();
                }
                else if color.ends_with(" blue") {
                    blue = color.strip_suffix(" blue").unwrap().parse::<u32>().unwrap();
                } else if color.ends_with(" green") {
                    green = color.strip_suffix(" green").unwrap().parse::<u32>().unwrap();
                }
            }
            counts.push((red, green, blue));
        }

        Ok(Sample {id, counts })
    }
}