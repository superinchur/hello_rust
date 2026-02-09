//#문제
//#N개의 정수로 이루어진 수열이 있을 때, 크기가 양수인 부분수열 중에서 그 수열의 원소를 다 더한 값이 S가 되는 경우의 수를 구하는 프로그램을 작성하시오.

//#입력
//#첫째 줄에 정수의 개수를 나타내는 N과 정수 S가 주어진다. (1 ≤ N ≤ 20, |S| ≤ 1,000,000) 둘째 줄에 N개의 정수가 빈 칸을 사이에 두고 주어진다. 
//#주어지는 정수의 절댓값은 100,000을 넘지 않는다.

//#출력
//#첫째 줄에 합이 S가 되는 부분수열의 개수를 출력한다.

//- **분수열 알고리즘을 직접 구현**
// - `풀이1` - $O(2^N)$
// - **조합 알고리즘을 이용하여** 부분수열 알고리즘을 구현
// - `풀이2` - $O(2^N)$

//# Tempalte
//# Main은 backjun에 맞게 stdin/stdout을 사용
//# TestCase는 현재 개발환경에 맞춰서 examples/inputs/....txt로 설정

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::cmp::Ordering;

// 2. 핵심 알고리즘 로직
fn solve<R: Read, W: Write>(input: R, output: W) -> io::Result<()> {
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    // N 읽기
    let mut first_line = String::new();
    if reader.read_line(&mut first_line)? == 0 { return Ok(()); }
    let n: usize = first_line.trim().parse().unwrap_or(0);

    Ok(())
}

// 2. 실제 제출용 메인 함수 (표준 입출력 사용)
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    
    // 표준 입출력을 solve 함수에 전달
    solve(stdin.lock(), stdout.lock())
}

// 4. 테스트 케이스 (파일에서 읽어오기)
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs::{self, File};

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