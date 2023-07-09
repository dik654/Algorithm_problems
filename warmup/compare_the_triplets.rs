// env파일 관련 라이브러리
use std::env;
// 파일 열기, 파일 읽고 쓰기 관련 라이브러리
use std::fs:File;
// stream단위 읽기 쓰기에 관련된 라이브러리
use std::io::{self, BufRead, Write};

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut A = 0;
    let mut B = 0;
    for i in 0..a.len() {
        // rust에는 increment(++), decrement(--) 연산자가 없다
        // 함수형 프로그래밍 형태를 많이 띄기 때문에 사용할 일이 적어서 그렇다고 한다
        if a[i] > b[i]  {
            A = A + 1
        } else if a[i] == b[i] {
            continue
        } else if a[i] < b[i] {
            B = B + 1
        };
    }

    vec![A, B]
}

fn main() {
    // 사용자 입력
    let stdin = io::stdin();
    // lock을 걸어 동시에 읽는 것을 제한
    // lines로 줄 단위로 stdin읽기
    let mut stdin_iterator = stdin.lock().lines();
    // OUTPUT_PATH로 저장한 위치에 파일 생성
    // fptr는 파일에 대한 포인터
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // 첫 번째 줄 vector
    // next()로 값을 차례대로 읽기 
    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        // 읽은 값의 끝의 공백이나 공백문자 제거
        .trim_end()
        // 공백 단위로 값을 구분하여
        .split(' ')
        // string으로 읽었던 값을 i32로 변환
        // err를 unwrap하면 패닉
        .map(|s| s.to_string().parse::<i32>().unwrap())
        // 각각의 값을 모아서 Vec<i32>로 생성
        .collect();
    // 두 번째 줄 vector
    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0...result.len() {
        write!(&mut fptr,  "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }
    
    writeln!(&mut fptr).ok();
}