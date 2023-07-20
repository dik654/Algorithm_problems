use std::env;
use std::fs:File;
use std::io::{self, BufRead, Write};

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let mut max: i32 = 0;
    let mut count: i32 = 0;
    for i in 0..candles.len() {
        if (max < candles[i]) {
            count = 1;
            max = candles[i]; 
        } else if (max == candles[i]) {
            count += 1;
        };
    };
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>.unwrap();

    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}