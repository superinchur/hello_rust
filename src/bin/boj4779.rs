//!칸토어 집합은 0과 1사이의 실수로 이루어진 집합으로, 구간 [0, 1]에서 시작해서 각 구간을 3등분하여 가운데 구간을 반복적으로 제외하는 방식으로 만든다.
//!전체 집합이 유한이라고 가정하고, 다음과 같은 과정을 통해서 칸토어 집합의 근사를 만들어보자.
//!1. -가 3N개 있는 문자열에서 시작한다.
//!2. 문자열을 3등분 한 뒤, 가운데 문자열을 공백으로 바꾼다. 이렇게 하면, 선(문자열) 2개가 남는다.
//!3. 이제 각 선(문자열)을 3등분 하고, 가운데 문자열을 공백으로 바꾼다. 이 과정은 모든 선의 길이가 1일때 까지 계속 한다.

//!예를 들어, N=3인 경우, 길이가 27인 문자열로 시작한다.

//!---------------------------
//!여기서 가운데 문자열을 공백으로 바꾼다.

//!---------         ---------
//!남은 두 선의 가운데 문자열을 공백으로 바꾼다.
//!---   ---         ---   ---
//!한번 더
//!- -   - -         - -   - -
//!모든 선의 길이가 1이면 멈춘다. N이 주어졌을 때, 마지막 과정이 끝난 후 결과를 출력하는 프로그램을 작성하시오.
//! #입력
//!입력을 여러 줄로 이루어져 있다. 각 줄에 N이 주어진다. 파일의 끝에서 입력을 멈춘다. N은 0보다 크거나 같고, 12보다 작거나 같은 정수이다.
//! #출력
//!입력으로 주어진 N에 대해서, 해당하는 칸토어 집합의 근사를 출력한다.

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
// 2. 핵심 알고리즘 로직
fn solve<R: Read, W: Write>(input: R, output: W) -> io::Result<()> {
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    // 3. 성능을 위해 BufReader로 감싸기 (C#의 StreamReader 역할)

    for line in reader.lines() {
        let line = line?; // 읽기 에러 체크
        if let Ok(n) = line.trim().parse::<u32>() {
            let length = 3usize.pow(n);
            // 3의 n승만큼의 -를 생성
            let mut result = vec!['-'; length];
            
            cantor_recursive(0, length, &mut result);
            writeln!(writer, "{}", result.into_iter().collect::<String>())?;
            
        }
    }

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


/// 칸토어 집합을 bottom-up으로 생성
fn cantor_buttomup(n: usize) -> String{
    // 1. 초기값이 들어있는 벡터 생성 (C#의 new List<uint> { "-" })
    let mut cantor_list: Vec<String> = vec!["-".to_string()];

    // 2. n까지 계산하며 배열에 추가
    for i in 1..=n {
        // 이전 단계의 모양 (N-1 단계)
        let prev = &cantor_list[i - 1];
        
        // 가운데 들어갈 공백의 길이 (3^(i-1))
        let space_len = 3usize.pow((i - 1) as u32);
        let spaces = " ".repeat(space_len);

        // 새로운 모양 생성: [이전 모양] + [공백] + [이전 모양]
        let next_val = format!("{}{}{}", prev, spaces, prev);
        
        cantor_list.push(next_val);
    }
    
    // 최종 결과물인 n번째 문자열 반환
    //cantor_list[n].clone()
    cantor_list.pop().unwrap() // 마지막 요소를 '추출'해서 소유권을 호출자에게 넘김
}

/// 칸토어 집합을 재귀적으로 생성합니다.
///
/// # Arguments
/// * `start` - 현재 구간의 시작 인덱스
/// * `len` - 현재 구간의 길이
/// * `arr` - 결과를 담을 가변 벡터 참조
fn cantor_recursive(start: usize, len: usize, arr: &mut Vec<char>) {
    // 길이가 1이면 더 이상 나눌 수 없음 (기본 케이스)
    if len <= 1 {
        return;
    }

    let third = len / 3;

    // 가운데 부분을 공백으로 바꿈 (start + third ~ start + 2*third - 1)
    for i in (start + third)..(start + 2 * third) {
        arr[i] = ' ';
    }

    // 왼쪽 부분 재귀
    cantor_recursive(start, third, arr);
    // 오른쪽 부분 재귀
    cantor_recursive(start + 2 * third, third, arr);
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

    
    /// 칸토어 집합 결과를 문자열로 반환하는 헬퍼 함수
    fn get_cantor_string(n: u32) -> String {
        let length = 3usize.pow(n);
        let mut result = vec!['-'; length];
        cantor_recursive(0, length, &mut result);
        result.into_iter().collect()
    }
    #[test]
    fn test_cantor_cases() {
        // N = 0: "-"
        assert_eq!(get_cantor_string(0), "-");

        // N = 1: "- -"
        assert_eq!(get_cantor_string(1), "- -");

        // N = 2: "- -   - -"
        assert_eq!(cantor_buttomup(2), "- -   - -");

        // N = 3: "- -   - -         - -   - -"
        assert_eq!(
            cantor_buttomup(3), 
            "- -   - -         - -   - -"
        );
    }

    #[test]
    fn test_cantor_length() {
        // 출력이 항상 3^n의 길이를 가지는지 확인
        for n in 0..=5 {
            let res = get_cantor_string(n);
            assert_eq!(res.len(), 3usize.pow(n));
        }
    }
}

   