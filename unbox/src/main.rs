use std::env;
use std::fs;

use tinyset::Set64;

fn main() {
    let argv: Vec<String> = env::args().collect();

    // Validate arguments: four sides, three unique letters each.
    let (_, sides) = &argv.split_first().unwrap();
    if sides.len() != 4 {
        panic!("must pass 4 arguments (one for letters of each side)")
    }
    // While validating, construct a puzzle struct.
    let puzzle = {
        let mut puzzle: Puzzle = [Set64::new(), Set64::new(), Set64::new(), Set64::new()];
        for (i, side) in sides.iter().enumerate() {
            let s = side.chars().collect::<Set64<char>>();
            if side.len() != 3 || s.len() != 3 {
                panic!("each argument must be three unique letters (one side of a puzzle), but {:?} is not", side)
            }
            puzzle[i] = side.chars().collect::<Set64<char>>();
        }
        puzzle
    };

    // Read the list of English words.
    // TODO: do not hardcode path to file. Maybe use an environment variable to avoid flag parsing? Maybe embed the dictionary within the binary?
    let contents = fs::read_to_string("../third_party/english-words/words_alpha.txt").unwrap();

    // TODO: stream this? Is streaming necessary if we embed the value?
    let mut candidates: Vec<&str> = Vec::new();
    for word in contents.lines() {
        // Check whether the word could have been validly generated.
        if validate(word, &puzzle) && word.len() >= 3 {
            candidates.push(word);
        }
    }

    // Compute aligning words.
    println!("{:?}", candidates)
}

type Puzzle = [Set64<char>; 4];

fn validate(word: &str, puzzle: &Puzzle) -> bool {
    // Determine which, if any, of the puzzle sides contains the first letter of the word.
    let chars: Vec<char> = word.chars().collect();
    let (first, rest) = chars.split_first().unwrap();
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

    // Iterate through the rest of the word, checking that each character may be constructed by a different puzzle side than the last.
    for c in rest.iter() {
        let mut ok = false;
        for (i, side) in puzzle.iter().enumerate() {
            if i == last_side {
                continue;
            }
            if side.contains(c) {
                ok = true;
                last_side = i;
                break;
            }
        }
        if !ok {
            return false;
        }
    }
    return true;
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
