use std::io::{self, BufRead};

fn insertionSort1(n: i32, arr: &[i32]) {
    let mut vec = arr.to_owned();
    let vec_size = vec.len();
    let last = arr[arr.len() - 1];

    for i in 0..vec_size {
        let mut printer: bool = false;
        let pivot: i32 = vec[i];
        for j in (1..i+1).rev() {
            if vec[j-1] <= pivot { 
                vec[j] = pivot;
                if printer {   
                    vec.iter().for_each(|element|
                        print!("{} ", element)
                    );
                    println!();
                }
                break;
            }
            if vec[j-1] > pivot {
                printer = true;
                vec[j] = vec[j-1];
                if printer {   
                    vec.iter().for_each(|element|
                        print!("{} ", element)
                    );
                    println!();
                }
                if i == vec_size-1 && j == 1 && vec[j-1] > pivot {
                    vec[j-1] = pivot;
                    vec.iter().for_each(|element|
                        print!("{} ", element)
                    );
                    println!();
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>().unwrap()).collect();

    insertionSort1(n, &arr);
}
