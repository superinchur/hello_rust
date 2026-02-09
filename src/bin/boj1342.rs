//#문제
//#민식이와 준영이는 자기 방에서 문자열을 공부하고 있다. 민식이가 말하길 인접해 있는 모든 문자가 같지 않은 문자열을 행운의 문자열이라고 한다고 한다. 준영이는 문자열 S를 분석하기 시작했다. 준영이는 문자열 S에 나오는 문자를 재배치하면 서로 다른 행운의 문자열이 몇 개 나오는지 궁금해졌다. 만약 원래 문자열 S도 행운의 문자열이라면 그것도 개수에 포함한다.

//#입력
//#첫째 줄에 문자열 S가 주어진다. S의 길이는 최대 10이고, 알파벳 소문자로만 이루어져 있다.

//#출력
//#첫째 줄에 위치를 재배치해서 얻은 서로 다른 행운의 문자열의 개수를 출력한다.


//# TestCase는 현재 개발환경에 맞춰서 examples/inputs/....txt로 설정

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

// 2. 핵심 알고리즘 로직
fn solve<R: Read, W: Write>(input: R, output: W) -> io::Result<()> {
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    // N 읽기
    let mut first_line = String::new();
    if reader.read_line(&mut first_line)? == 0 { return Ok(()); }
    let n: usize = first_line.trim().parse().unwrap_or(0);

    // 1. character를 쪼개서 하나씩 배열에 넣기.
    

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