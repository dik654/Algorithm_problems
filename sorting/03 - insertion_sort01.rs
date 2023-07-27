use std::io::{self, BufRead};

fn insertionSort1(n: i32, arr: &[i32]) {
    let mut vec = arr.to_owned();
    let vec_size = vec.len();
    let last = arr[arr.len() - 1];

    for i in 1..vec_size {
        let mut pivot = vec
        for j in (1..i + 1).rev() {
            if vec[j - 1] <= vec[j] { break; }
            vec.swap(j - 1, j);
            for k in 0..vec_size {
                print!("{} ", vec[k]);
            }
            println!();
        }
    }
}

// Your Output (stdout)
// 2 4 6 3 8 
// 2 4 3 6 8 
// 2 3 4 6 8 
// Expected Output
// 2 4 6 8 8
// 2 4 6 6 8
// 2 4 4 6 8
// 2 3 4 6 8

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    insertionSort1(n, &arr);
}
