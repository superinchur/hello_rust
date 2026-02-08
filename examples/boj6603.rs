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

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let path = "examples/inputs/boj6603_input.txt";
    let file = File::open(path).expect("파일을 찾을 수 없습니다.");
    let reader = BufReader::new(file);

    let mut first_case = true;

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // 첫 번째 숫자가 k (0이면 종료)
        let k = nums[0];
        if k == 0 { break; }

        // 각 테스트 케이스 사이에 빈 줄 출력 (첫 케이스 제외)
        if !first_case {
            println!();
        }
        first_case = false;

        let s = &nums[1..]; // 실제 집합 S
        let mut combination = vec![0; 6]; // 선택된 6개를 담을 공간
        
        backtrack(s, 0, 0, &mut combination);
    }

    Ok(())
}

/// 조합을 찾기 위한 백트래킹 함수
///
/// # Arguments
/// * `s` - 전체 숫자 집합
/// * `start` - 탐색을 시작할 인덱스
/// * `depth` - 현재까지 선택한 숫자의 개수
/// * `out` - 선택된 숫자를 저장할 배열
fn backtrack(s: &[i32], start: usize, depth: usize, out: &mut Vec<i32>) {
    // 6개를 모두 골랐다면 출력
    if depth == 6 {
        let res: String = out.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", res);
        return;
    }

    // 남은 숫자가 골라야 할 숫자보다 적으면 더 이상 진행할 필요 없음 (가지치기)
    for i in start..s.len() {
        if s.len() - i < 6 - depth { break; }

        out[depth] = s[i];
        backtrack(s, i + 1, depth + 1, out);
    }
}