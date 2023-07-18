use std::io::{self, BufRead};

// fn staircase(n: i32) {
//     let mut sharp = String::new();
//     for _ in 0..n {
//         sharp.push_str("#");
//         println!("{}", sharp);
//     }
// }
// Your Output (stdout)
// #
// ##
// ###
// ####
// #####
// ######
// Expected Output
//      #
//     ##
//    ###
//   ####
//  #####
// ######

fn staircase(n: i32) {
    let sharp = '#';
    let space = ' ';
    // 맨 끝의 값이 반복에 포함되지않아서 n+1
    for i in 1..n+1 {
        println!(
            "{}{}",
            // repeat의 인자로 usize를 요구하여 as를 이용해 i32 -> usize 타입변경
            space.to_string().repeat((n - i) as usize),
            sharp.to_string().repeat((i) as usize)
        );
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}