use std::env;
use std::fs:File;
use std::io::{self, BufRead, Write};

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut sum;
    for i in 0..arr.len() {
        = arr[i][i] - arr[arr.len()-1 - i][i]);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    // 입력 배열 개수
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    // 2차원 vector 선언
    // with_capacity는 행의 개수 
    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i i in 0..n as usize {
        // arr에 입력으로 들어온 배열들 차례대로 저장
        arr[i] = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();

}