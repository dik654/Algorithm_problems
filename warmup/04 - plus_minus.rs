use std::io::{self, BufRead};


fn plusMinus(arr: &[i32]) {
    let iterator = arr.iter();
    // vector는 크기가 동적으로 변하며 배열과 다르다
    // vector로 선언하려면 to_vec()메서드를 이용해야한다. let mut result: Vec<i32> = [0, 0, 0].to_vec();
    let mut result: [i32; 3] = [0, 0, 0];
    
    for element in arr {
        match element {
            // for문에서 원소는 값 그자체가 아니라 참조를 이용하므로 integer인 0이 아닌 0의 참조인 &0과 비교해야한다
            n if n > &0 => result[0]+=1,
            0 => result[2]+=1,
            _ => result[1]+=1,
        }
    }
    // 정수값을 리턴하길 바라므로 f64로 변환
    let length = arr.len() as f64;
    // for_each와 map과 동일하게 동작하지만 결과값으로 ()를 리턴한다는 차이가 있다.
    // flat_map과 map의 차이는 아직 이해하지 못했다.
    result.iter().for_each(|x| println!("{}", *x as f64 / length))
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next()unwrap().unwrap().trim_end().split(' ').map(|s| s.to_string().parse::<i32>.unwrap()).collect();
    plusMinus(&arr);
}