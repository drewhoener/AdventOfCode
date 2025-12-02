use std::fmt::Write;
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use std::env;
use std::fs::File;
use std::io::Read;

pub(crate) struct InvalidIdSum {
    pub(crate) repeated_twice: i64,
    pub(crate) repeated_tiling: i64,
}

#[derive(PartialEq, Debug)]
enum ChallengeError {
    InvalidFile,
    InvalidInput,
}

fn parse_ranges(path: &str) -> Result<Vec<(i64, i64)>, ChallengeError> {
    let mut file = File::open(path).map_err(|_| ChallengeError::InvalidFile)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| ChallengeError::InvalidFile)?;

    let ranges: Vec<(i64, i64)> = contents
        .split(",")
        .filter_map(|str: &str| {
            let split: Vec<&str> = str.split("-").collect();
            if split.len() != 2 {
                println!("Invalid range {}", str);
                return None;
            }
            let start = split[0].parse::<i64>().ok();
            let end = split[1].parse::<i64>().ok();

            if start.is_none() || end.is_none() {
                println!("Invalid range {}", str);
                return None;
            }

            Some((start.unwrap(), end.unwrap()))
        })
        .collect();

    Ok(ranges)
}

fn substring_tiles_string(repeat_slice: &str, source: &str) -> bool {
    if repeat_slice.is_empty() {
        return false;
    }

    // Must cleanly tile the string
    if !source.len().is_multiple_of(repeat_slice.len()) {
        return false;
    }

    let num_slices = source.len() / repeat_slice.len();

    for i in 0..num_slices {
        let start = repeat_slice.len() * i;
        let end = start + repeat_slice.len();
        let check_slice = &source[start..end];
        if check_slice != repeat_slice {
            return false;
        }
    }
    true
}

fn is_repeated(str: &str) -> bool {
    if str.len() < 2 {
        return false;
    }

    let max_check = str.len() / 2;
    // More likely to find the answer quickly if we check the large ones first
    // More iterations for less gain if we go the other way around (in theory)
    for i in (1..=max_check).rev() {
        let repeat_slice = &str[0..i];
        if substring_tiles_string(repeat_slice, str) {
            return true;
        }
    }

    false
}

// Returns if the input string is made of a sequence repeated twice
fn is_repeated_twice(str: &str) -> bool {
    if str.is_empty() || !str.len().is_multiple_of(2) {
        return false;
    }

    let mid = str.len() / 2;
    let start = &str[0..mid];
    let end = &str[mid..];

    start == end
}

pub(crate) fn check_invalid_ids(ranges: Vec<(i64, i64)>) -> InvalidIdSum {
    let mut sums = InvalidIdSum {
        repeated_twice: 0,
        repeated_tiling: 0,
    };

    let mut num_str = String::new();
    for (start, end) in ranges {
        for i in start..=end {
            num_str.clear();
            write!(&mut num_str, "{}", i).unwrap();
            if is_repeated_twice(&num_str) {
                sums.repeated_twice += i;
            }

            if is_repeated(&num_str) {
                sums.repeated_tiling += i;
            }
        }
    }

    sums
}

fn main() -> Result<(), ChallengeError> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(ChallengeError::InvalidFile);
    }

    let ranges = parse_ranges(&args[1])?;

    let result = check_invalid_ids(ranges);
    println!();
    println!("Repeated Twice Sum: {}", result.repeated_twice);
    println!("Repeated Twice+ Sum: {}", result.repeated_tiling);

    Ok(())
}
