use std::collections::HashMap;
use std::fs;

use clap::Clap;

use tinyset::Set64;

#[derive(Clap, Debug)]
struct Opts {
    /// File path to dictionary of words.
    #[clap(short, long)]
    dictionary: String,

    /// Maximum word count of answer.
    #[clap(short, long, default_value = "4")]
    max_words: i8,

    /// Letter Boxed puzzle sides: four arguments of 3 unique letters each.
    // TODO: use array syntax once https://github.com/clap-rs/clap/issues/1682 lands.
    // TODO: use number_of_values once https://github.com/clap-rs/clap/issues/2229 is fixed.
    #[clap(min_values=4, max_values=4, required=true, validator=side_validator)]
    side: Vec<String>,
}

fn side_validator(side: &str) -> Result<(), String> {
    let s = side.chars().collect::<Set64<char>>();
    if side.len() != 3 || s.len() != 3 {
        Err("puzzle sides must be three unique letters (one side of a puzzle)".to_string())
    } else {
        Ok(())
    }
}

fn main() {
    let opts = Opts::parse();

    // Construct a puzzle struct.
    let puzzle = {
        let mut puzzle: Puzzle = [Set64::new(), Set64::new(), Set64::new(), Set64::new()];
        for (i, side) in opts.side.iter().enumerate() {
            puzzle[i] = side.chars().collect::<Set64<char>>();
        }
        puzzle
    };

    // Read the list of English words.
    // TODO: embed the dictionary within the binary?
    let contents = fs::read_to_string(opts.dictionary).unwrap();

    // Build a map of "character" to "puzzle-valid words that start with that character".
    // TODO: since the keys are known statically, we can use a struct instead of a map if we need to.
    let mut starts_with = HashMap::<char, Vec<&str>>::with_capacity(26);
    for c in 'a'..='z' {
        starts_with.insert(c, Vec::new());
    }

    // Add puzzle-valid words into the map.
    // TODO: stream the computation? Is streaming necessary if we embed the value?
    for word in contents.lines() {
        // Check whether the word could have been validly generated.
        if validate(word, &puzzle) && word.len() >= 3 {
            let first = word.chars().next().unwrap();
            let words = starts_with.get_mut(&first).unwrap();
            words.push(word);
        }
    }

    // Iterating over all possible starting characters, construct puzzle-valid word strings of length at most max_words.
    let missing_chars = puzzle
        .iter()
        .fold(Set64::<char>::new(), |acc, side| &acc | side);

    // Start recursion: try every word as beginning of puzzle-valid word string.
    let max_words = opts.max_words;
    let answer = starts_with
        .keys()
        .filter_map(|c| {
            starts_with
                .get(c)
                .unwrap()
                .iter()
                .filter_map(|word| {
                    word_strings_recurse(&starts_with, word, missing_chars.clone(), max_words - 1)
                        .map(|mut ws| {
                            ws.insert(0, word);
                            ws
                        })
                })
                .fold(None, shorter_word_string)
        })
        .fold(None, shorter_word_string);

    match answer {
        Some(a) => println!("{:?}", a),
        None => println!("No answer found."),
    }
}

type Puzzle = [Set64<char>; 4];

fn validate(word: &str, puzzle: &Puzzle) -> bool {
    // Determine which, if any, of the puzzle sides contains the first letter of the word.
    let mut chars = word.chars();
    let first = chars.next().unwrap();
    let rest = chars;
    let mut last_side = 4;
    for (i, side) in puzzle.iter().enumerate() {
        if side.contains(first) {
            last_side = i;
            break;
        }
    }
    if last_side == 4 {
        return false;
    }

    // Iterate through the rest of the word, checking that each character is a member of a puzzle side AND a member of a different puzzle side than the previous character.
    for c in rest {
        let mut ok = false;
        for (i, side) in puzzle.iter().enumerate() {
            // Ignore the last puzzle side.
            if i == last_side {
                continue;
            }
            if side.contains(c) {
                ok = true;
                last_side = i;
                break;
            }
        }
        // If the character is not a member of any puzzle side, the word is invalid.
        if !ok {
            return false;
        }
    }
    return true;
}

fn word_strings_recurse<'a>(
    starts_with: &HashMap<char, Vec<&'a str>>,
    word: &'a str,
    missing_chars: Set64<char>,
    remaining_words: i8,
) -> Option<Vec<&'a str>> {
    // Compute new missing character set given this word.
    let mut chars = word.chars();
    let last = chars.next_back().unwrap();
    let mut next_missing_chars = missing_chars.clone();
    next_missing_chars.remove(&last);
    for c in chars {
        next_missing_chars.remove(&c);
    }

    // Base case: there are no more characters missing. We've succeeded! Return
    // a result.
    if next_missing_chars.len() == 0 {
        return Some(vec![word]);
    }

    // Base case: there are no words remaining. We've failed to find words that
    // cover the puzzle.
    if remaining_words - 1 <= 0 {
        return None;
    }

    // Recursive case: there are still characters missing, but we have more
    // words remaining.
    let starts_with_last_char = starts_with.get(&last).unwrap();

    return starts_with_last_char
        .iter()
        .filter_map(|next_word| {
            word_strings_recurse(
                starts_with,
                next_word,
                next_missing_chars.clone(),
                remaining_words - 1,
            )
        })
        .fold(None, shorter_word_string);
}

fn shorter_word_string<'a>(prev: Option<Vec<&'a str>>, next: Vec<&'a str>) -> Option<Vec<&'a str>> {
    match prev {
        None => Some(next),
        Some(prev) => Some(if next.len() < prev.len() { next } else { prev }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        // TODO: once https://github.com/rust-lang/rust/issues/75243 lands, use `.map(|x| x.chars().collect::<Set64<char>>()`.
        let puzzle: super::Puzzle = [
            "uoa".chars().collect::<Set64<char>>(),
            "qtl".chars().collect::<Set64<char>>(),
            "ein".chars().collect::<Set64<char>>(),
            "ysm".chars().collect::<Set64<char>>(),
        ];
        assert!(validate("melony", &puzzle));
        assert!(validate("yeast", &puzzle));
        assert!(validate("tequila", &puzzle));
        assert!(!validate("quinoa", &puzzle));
    }
}
