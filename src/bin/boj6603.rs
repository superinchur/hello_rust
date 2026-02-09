//!문제
//!독일 로또는 {1, 2, ..., 49}에서 수 6개를 고른다.
//!로또 번호를 선택하는데 사용되는 가장 유명한 전략은 49가지 수 중 k(k>6)개의 수를 골라 집합 S를 만든 다음 그 수만 가지고 번호를 선택하는 것이다.
//!예를 들어, k=8, S={1,2,3,5,8,13,21,34}인 경우 이 집합 S에서 수를 고를 수 있는 경우의 수는 총 28가지이다. ([1,2,3,5,8,13], [1,2,3,5,8,21], [1,2,3,5,8,34], [1,2,3,5,13,21], ..., [3,5,8,13,21,34])
//!집합 S와 k가 주어졌을 때, 수를 고르는 모든 방법을 구하는 프로그램을 작성하시오.
//!입력
//!입력은 여러 개의 테스트 케이스로 이루어져 있다. 각 테스트 케이스는 한 줄로 이루어져 있다. 첫 번째 수는 k (6 < k < 13)이고, 다음 k개 수는 집합 S에 포함되는 수이다. S의 원소는 오름차순으로 주어진다.
//!입력의 마지막 줄에는 0이 하나 주어진다. 
//!출력
//!각 테스트 케이스마다 수를 고르는 모든 방법을 출력한다. 이때, 사전 순으로 출력한다.
//!각 테스트 케이스 사이에는 빈 줄을 하나 출력한다.
//! 

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
/// 수정된 백트래킹 함수
fn backtrack<W: Write>(
    s: &[i32], 
    start: usize, 
    depth: usize, 
    out: &mut Vec<i32>, 
    writer: &mut W // Writer를 가변 참조자로 받음
) -> io::Result<()> {
    // 6개를 모두 골랐다면 출력
    if depth == 6 {
        for (i, &num) in out.iter().enumerate() {
            write!(writer, "{}", num)?;
            if i < 5 { write!(writer, " ")?; }
        }
        writeln!(writer)?;
        return Ok(());
    }

    for i in start..s.len() {
        // 가지치기: 남은 숫자가 부족하면 중단
        if s.len() - i < 6 - depth { break; }

        out[depth] = s[i];
        // 재귀 호출 시 writer를 다시 넘겨줌
        backtrack(s, i + 1, depth + 1, out, writer)?;
    }
    
    Ok(())
}

fn solve<R: Read, W: Write>(input: R, output: W) -> io::Result<()> {
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output); // 여기서 만든 writer를 사용

    let mut first_case = true;

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let k = nums[0];
        if k == 0 { break; }

        if !first_case {
            writeln!(writer)?; // println! 대신 writer 사용
        }
        first_case = false;

        let s = &nums[1..];
        let mut combination = vec![0; 6];
        
        // backtrack 호출 시 writer 전달
        backtrack(s, 0, 0, &mut combination, &mut writer)?;
    }

    // BufWriter는 마지막에 flush를 해주거나 스코프가 끝나야 데이터가 완전히 기록됩니다.
    writer.flush()?; 
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