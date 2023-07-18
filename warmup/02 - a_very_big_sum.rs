use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn aVeryBigSum(ar: &[i64]) -> i64 {
    // for문을 이용한 합과
    // ar.iter().sum()도 가능
    // fold를 사용하는 경우 누적값의 초기값을 첫 번째 인수로 설정할 수 있으며, 두 번째 인수로 vector 원소들이 차례대로 계산될 식을 제공할 수 있다
    // fold가 reduce와 다른 점은 초기값을 정할 수 있다는 점이다 
    let sum = ar.iter().fold(0, |acc, x| acc + x);
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    // 입력 원소 개수
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    // 입력 원소 vector
    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i64>().unwrap()).collect();

    let result = aVeryBigSum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}