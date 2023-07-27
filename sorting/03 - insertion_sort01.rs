use std::io::{self, BufRead};

fn insertionSort1(n: i32, arr: &[i32]) {
    // 정렬을 하려면 수정해야하므로 소유권을 이전해서 복사
    let mut vec = arr.to_owned();
    // arr length
    let vec_size = vec.len();
    // 배열의 마지막 값
    let last = arr[arr.len() - 1];
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    insertionSort1(n, &arr);
}
