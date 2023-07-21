use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String {
    // 시 분 초 정보에서 시 정보만 파싱
    let mut hour: u8 = (&s[0..2]).parse().expect("Couldn't parse");
    // 만약 PM이라면
    hour = if &s[8..9]=="P" {
        // PM 12시라면 24시가 아닌 12시를 나타야하므로 hour % 12로 계산
        12 + hour % 12 
    } else {
        // AM이라면 00 ~ 12시로 표기되므로 
        hour % 12
    };
    // {:0>2}는 들어온 변수를 2자리수로 나타내고, 빈 자리는 0으로 채운다. 만약 hour가 2라면 02로 표시된다
    let hour = format!("{:0>2}{}", hour, &s[2..8]);
    hour
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}