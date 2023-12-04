use std::fs::read_to_string;
use std::str::FromStr;

/*
The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.
 */
fn main() {
    let mut sum = 0;
    for line in read_to_string("src/2/2.input").unwrap().lines() {
        let sample = line.parse::<Sample>().unwrap();
        let mins = min_counts(&sample);
        let power = mins.0 * mins.1 * mins.2;
        println!("{}", power);
        sum += power;
    }
    println!("{}", sum);
}

fn min_counts(sample: &Sample) -> (u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for count in &sample.counts {
        if count.0 > red {
            red = count.0
        }
        if count.1 > green {
            green = count.1
        }
        if count.2 > blue {
            blue = count.2
        }
    }
    return (red, green, blue)
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