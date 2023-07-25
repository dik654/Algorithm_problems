use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn bigSorting(unsorted: &[String]) -> Vec<String> {
    let mut sorted = unsorted.to_vec();
    sorted.sort_by(|a, b| {
        // 길이가 다르면 먼저 길이로 비교
        if a.len() != b.len() {
            return a.len().cmp(&b.len());
        }
        // 길이가 같으면 값의 크기를 비교
        a.cmp(&b)
    });
    sorted
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut unsorted: Vec<String> = Vec::with_capacity(n as usize);
    // input 값들 배열에 담기
    for _ in 0..n {
        let unsorted_item = stdin_iterator.next().unwrap().unwrap();
        unsorted.push(unsorted_item);
    }
    // sorting 작업
    let result = bigSorting(&unsorted);
    // output 쓰기
    for i in 0..result.len() {
        // 파일에 쓰기
        write!(&mut fptr, "{}", result[i]).ok();
        // 마지막 원소까지 썼으면 \n\n으로 파일의 종료를 의미(표준은 아님)
        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
        writeln!(&mut fptr).ok();
    }
}