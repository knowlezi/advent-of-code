/*
--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

use anyhow::Result;

static INPUT: &'static str = include_str!("./input");

const DIGITS_AS_WORDS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_first_digit(chars: &std::str::Chars) -> Result<u32> {
    let mut m_chars = chars.clone();
    for c in m_chars.by_ref() {
        if c.is_ascii_digit() {
            let number = Some(
                c.to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Invalid digit: {}", c))?,
            );
            return number
                .ok_or_else(|| anyhow::anyhow!("No digit found: {}", c));
        }
    }

    anyhow::bail!("Unable to find first digit: {}", chars.as_str())
}

fn find_last_digit(chars: &std::str::Chars) -> Result<u32> {
    let mut m_chars = chars.clone();
    while let Some(c) = m_chars.next_back() {
        if c.is_ascii_digit() {
            let number = Some(
                c.to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Invalid digit: {}", c))?,
            );
            return number
                .ok_or_else(|| anyhow::anyhow!("No digit found: {}", c));
        }
    }

    anyhow::bail!("Unable to find last digit: {}", chars.as_str())
}

fn resolve_digits(chars: &std::str::Chars) -> Result<String> {
    let mut resolved = String::new();

    let mut m_chars = chars.clone();
    for (index, c) in m_chars.by_ref().enumerate() {
        if c.is_ascii_digit() {
            resolved.push(c);
        } else {
            let word = DIGITS_AS_WORDS.iter().position(|&w| w.starts_with(c));

            if word.is_some() {
                let line = chars.as_str();
                let line = &line[index..];
                if let Some(word) =
                    DIGITS_AS_WORDS.iter().find(|&w| line.starts_with(w))
                {
                    resolved.push_str(match *word {
                        "zero" => "0",
                        "one" => "1",
                        "two" => "2",
                        "three" => "3",
                        "four" => "4",
                        "five" => "5",
                        "six" => "6",
                        "seven" => "7",
                        "eight" => "8",
                        "nine" => "9",
                        _ => anyhow::bail!("Unable to resolve '{}'", word),
                    });
                }
            }
        }
    }

    Ok(resolved)
}

fn solve() -> Result<u32> {
    let mut calibration_values = Vec::new();

    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }

        let resolved_line = resolve_digits(&line.chars())?;
        let chars = resolved_line.chars();
        let first = find_first_digit(&chars)?;
        let last = find_last_digit(&chars)?;
        calibration_values.push(format!("{}{}", first, last).parse::<u32>()?);
    }

    Ok(calibration_values.iter().sum())
}

fn main() {
    let sum = solve().unwrap();
    println!("Calibration value: {}", sum);
}
