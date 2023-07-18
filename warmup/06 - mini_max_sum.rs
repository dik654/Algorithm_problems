use std::io::{self, BufRead};

fn miniMaxSum(arr: &[i32]) {
    // 오버플로우 방지를 위해 map을 이용해 원소들을 i64로 변환한 뒤에 합 구하기
    let sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let max: i32 = *arr.iter().max().unwrap();
    let min: i32 = *arr.iter().min().unwrap();
    // i64인 sum과 계산하기 위해 max와 min도 i64로 변환
    println!("{} {}", sum - max as i64, sum - min as i64);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
