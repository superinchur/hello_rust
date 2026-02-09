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


use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    // 1. 입력이 여러 줄로 들어오므로 stdin의 lock을 사용하여 효율적으로 읽습니다.
    let path = "examples/inputs/boj4779_input.txt";
    // 2. 파일 열기 (C#의 FileStream과 유사)
    let file = File::open(path).map_err(|e| {
        eprintln!("파일 열기 실패: {} (경로: {:?})", e, std::env::current_dir().unwrap().join(path));
        e
    })?;

    // 3. 성능을 위해 BufReader로 감싸기 (C#의 StreamReader 역할)
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?; // 읽기 에러 체크
        if let Ok(n) = line.trim().parse::<u32>() {
            let length = 3usize.pow(n);
            // 3의 n승만큼의 -를 생성
            let mut result = vec!['-'; length];
            
            cantor_recursive(0, length, &mut result);
            
            println!("{}", result.into_iter().collect::<String>());
        }
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

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