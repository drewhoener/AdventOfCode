use crate::ChallengeError::InvalidLine;
use std::env;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum ChallengeError {
    InvalidInput,
    InvalidFile,
    InvalidLine(String),
    ParseError(ParseIntError),
}

struct WrappedNum {
    min: i32,
    max: i32,
    val: i32,
}

impl WrappedNum {
    fn new(min: i32, max: i32, value: i32) -> WrappedNum {
        Self {
            min,
            max,
            val: value,
        }
    }

    fn range(&self) -> i32 {
        self.max - self.min + 1
    }

    fn add(&mut self, value: i32) -> &mut Self {
        let mut new_val = self.val + value;
        while new_val > self.max {
            new_val -= self.range();
        }
        while new_val < self.min {
            new_val += self.range()
        }
        self.val = new_val;
        self
    }
}

fn instructions_to_numbers(lines: Vec<String>) -> Result<Vec<i32>, ChallengeError> {
    let mut out: Vec<i32> = Vec::new();
    for str in lines {
        if str.len() < 2 {
            return Err(ChallengeError::InvalidLine(str));
        }
        let first_char = &str[0..1];
        let rest = &str[1..];
        let multiplier = match first_char {
            "L" => Ok(-1),
            "R" => Ok(1),
            _ => Err(InvalidLine(first_char.to_owned())),
        }?;
        let num = rest.parse::<i32>().map_err(ChallengeError::ParseError)?;
        out.push(num * multiplier);
    }

    Ok(out)
}

fn open_input(path: &str) -> Result<Vec<String>, ChallengeError> {
    let mut input_file = File::open(path).map_err(|_| ChallengeError::InvalidFile)?;
    let mut contents = String::new();
    input_file
        .read_to_string(&mut contents)
        .map_err(|_| ChallengeError::InvalidFile)?;
    Ok(contents.split("\n").map(|x| x.to_owned()).collect())
}

fn main() -> Result<(), ChallengeError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(ChallengeError::InvalidInput);
    }

    let lines = open_input(&args[1])?;
    let numbers = instructions_to_numbers(lines)?;
    let mut wrapped_num = WrappedNum::new(0, 99, 50);

    let mut zero_count = 0;

    for num in numbers {
        wrapped_num.add(num);
        if wrapped_num.val == 0 {
            zero_count += 1;
        }
    }

    println!("Result: {}", zero_count);

    Ok(())
}
