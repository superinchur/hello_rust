//!#문제
//!2차원 평면 위의 점 N개가 주어진다. 좌표를 x좌표가 증가하는 순으로, x좌표가 같으면 y좌표가 증가하는 순서로 정렬한 다음 출력하는 프로그램을 작성하시오.

//!#입력
//!첫째 줄에 점의 개수 N (1 ≤ N ≤ 100,000)이 주어진다. 둘째 줄부터 N개의 줄에는 i번점의 위치 xi와 yi가 주어진다. (-100,000 ≤ xi, yi ≤ 100,000) 좌표는 항상 정수이고, 위치가 같은 두 점은 없다.

//!#출력
//!첫째 줄부터 N개의 줄에 점을 정렬한 결과를 출력한다.


use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

// 1. 점을 표현하는 구조체 정의 (또는 단순 튜플 사용 가능)
//1. Debug: "출력할 수 있게 해줘"
//2. PartialEq와 Eq: "같은지 비교할 수 있게 해줘"
//3. PartialOrd와 Ord: "줄 세우기(정렬)를 할 수 있게 해줘"
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}
// 2. 실제 제출용 메인 함수 (표준 입출력 사용)
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    
    // 표준 입출력을 solve 함수에 전달
    solve(stdin.lock(), stdout.lock())
}

// 3. 공통 로직 
fn solve<R: Read, W: Write>(input: R, output: W) -> io::Result<()> {
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    // 데이터 읽기
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    let n: usize = first_line.trim().parse().unwrap_or(0);

    let mut points = Vec::with_capacity(n);
    for line in reader.lines().take(n) {
        let line = line?;
        let mut coords = line.split_whitespace();
        if let (Some(x_s), Some(y_s)) = (coords.next(), coords.next()) {
            points.push(Point {
                x: x_s.parse().unwrap(),
                y: y_s.parse().unwrap(),
            });
        }
    }

    // 정렬
    points.sort();

    // 결과 쓰기
    for pt in points {
        writeln!(writer, "{} {}", pt.x, pt.y)?;
    }
    writer.flush()
}

// 4. 테스트 케이스 (파일에서 읽어오기)
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    #[test]
    fn test_from_file() -> io::Result<()> {

        // 1. 현재 파일의 전체 경로 가져오기 (예: "src/main.rs")
        let file_path: &str = file!(); 
        
        // 2. 경로에서 파일명만 분리하고 확장자(.rs) 제거하기
        // rust-analyzer: ignore
        let base_name = Path::new(file_path)
            .file_stem()            // 확장자 제거 (예: "main")
            .and_then(|s| s.to_str())
            .unwrap_or("default");

        let dir = "examples/inputs";

        // 3. 규칙에 따라 경로 생성
        let input_path = format!("{}/{}_input.txt", dir, base_name);
        let expected_path = format!("{}/{}_expected.txt", dir, base_name);
        let result_path = format!("{}/{}_result.txt", dir, base_name);

        println!("Testing with base_name: {}", base_name);

        // 4. 파일 열기
        let input_file = File::open(input_path).expect("테스트 입력 파일이 없습니다.");
        let output_file = File::create(&result_path)?;

        // 5. solve 함수 실행 (실제 알고리즘 수행)
        solve(input_file, output_file)?;

        // 6. 결과값(Result)과 정답지(Expected) 읽어오기
        let result = fs::read_to_string(result_path)?;
        let expected = fs::read_to_string(expected_path).expect("정답지 파일이 없습니다.");

        // 7. 검증 (두 내용이 일치하는지 확인)
        assert_eq!(result.trim(), expected.trim(), "결과가 정답지와 일치하지 않습니다!");
        Ok(())
    }
}