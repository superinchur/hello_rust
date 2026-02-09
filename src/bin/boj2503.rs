///정보문화진흥원 정보 영재 동아리에서 동아리 활동을 하던 영수와 민혁이는 쉬는 시간을 틈타 숫자야구 게임을 하기로 했다.

//영수는 1에서 9까지의 서로 다른 숫자 세 개로 구성된 세 자리 수를 마음속으로 생각한다. (예: 324)
//민혁이는 1에서 9까지의 서로 다른 숫자 세 개로 구성된 세 자리 수를 영수에게 묻는다. (예: 123)
//민혁이가 말한 세 자리 수에 있는 숫자들 중 하나가 영수의 세 자리 수의 동일한 자리에 위치하면 스트라이크 한 번으로 센다. 숫자가 영수의 세 자리 수에 있긴 하나 다른 자리에 위치하면 볼 한 번으로 센다.
//예) 영수가 324를 갖고 있으면 

//429는 1 스트라이크 1 볼이다.
//241은 0 스트라이크 2 볼이다.
//924는 2 스트라이크 0 볼이다.
//영수는 민혁이가 말한 수가 몇 스트라이크 몇 볼인지를 답해준다.
//민혁이가 영수의 세 자리 수를 정확하게 맞추어 3 스트라이크가 되면 게임이 끝난다. 아니라면 민혁이는 새로운 수를 생각해 다시 영수에게 묻는다.
//현재 민혁이와 영수는 게임을 하고 있는 도중에 있다. 민혁이가 영수에게 어떤 수들을 물어보았는지, 그리고 각각의 물음에 영수가 어떤 대답을 했는지가 입력으로 주어진다. 이 입력을 바탕으로 여러분은 영수가 생각하고 있을 가능성이 있는 수가 총 몇 개인지를 알아맞혀야 한다.

//아래와 같은 경우를 생각해보자.  

//민혁: 123
//영수: 1 스트라이크 1 볼.
//민혁: 356
//영수: 1 스트라이크 0 볼.
//민혁: 327
//영수: 2 스트라이크 0 볼.
//민혁: 489
//영수: 0 스트라이크 1 볼.
//이때 가능한 답은 324와 328, 이렇게 두 가지이다.

//영수는 동아리의 규율을 잘 따르는 착한 아이라 민혁이의 물음에 곧이곧대로 정직하게 답한다. 그러므로 영수의 답들에는 모순이 없다.

//민혁이의 물음들과 각각의 물음에 대한 영수의 답이 입력으로 주어질 때 영수가 생각하고 있을 가능성이 있는 답의 총 개수를 출력하는 프로그램을 작성하시오.

//입력
//첫째 줄에는 민혁이가 영수에게 몇 번이나 질문을 했는지를 나타내는 1 이상 100 이하의 자연수 N이 주어진다. 이어지는 N개의 줄에는 각 줄마다 민혁이가 질문한 세 자리 수와 영수가 답한 스트라이크 개수를 나타내는 정수와 볼의 개수를 나타내는 정수, 이렇게 총 세 개의 정수가 빈칸을 사이에 두고 주어진다.
//
//출력
//첫 줄에 영수가 생각하고 있을 가능성이 있는 답의 총 개수를 출력한다.
/// 

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::cmp::Ordering;

struct Query{
    throw_in: String,
    real_strike: i32,
    real_ball: i32, 
    
}

// 2. 핵심 알고리즘 로직
fn solve<R: Read, W: Write>(input: R, output: W) -> io::Result<()> {
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    // 데이터 입력받기
    let mut first_line = String::new();
    if reader.read_line(&mut first_line)? == 0 { return Ok(()); }
    let n: usize = first_line.trim().parse().unwrap_or(0);

    let mut queries = Vec::with_capacity(n);
    for line in reader.lines().take(n) {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            queries.push(Query {
                throw_in: parts[0].to_string(),
                real_strike: parts[1].parse().unwrap(),
                real_ball: parts[2].parse().unwrap(),
            });
        }
    }

    // 1. 모든 가능한 후보군을 미리 생성 (504개)
    let mut candidates: Vec<String> = Vec::new();
    for i in 1..=9 {
        for j in 1..=9 {
            for k in 1..=9 {
                if i != j && j != k && i != k {
                    candidates.push(format!("{}{}{}", i, j, k));
                }
            }
        }
    }

    // [최적화 포인트] 스트라이크가 많은 순(내림차순)으로 쿼리 정렬
    // 더 강력한 필터를 먼저 적용하여 후보군(search space)을 빠르게 줄입니다.
    queries.sort_by(|a, b| {
        b.real_strike.cmp(&a.real_strike)
            .then_with(|| b.real_ball.cmp(&a.real_ball))
    });

    // 2. 정렬된 쿼리를 순회하며 후보군 필터링 (retain)
    for q in queries {
        // retain은 조건에 true인 것만 남깁니다.
        candidates.retain(|cand| {
            let (s, b) = count_strike_ball(cand, &q.throw_in);
            s == q.real_strike && b == q.real_ball
        });
        
        // 만약 도중에 후보가 0개가 되면 0 리턴
        if candidates.is_empty() { break; }
    }

    // 3. 남은 후보의 개수가 정답
    writeln!(writer, "{}", candidates.len())?;
    writer.flush()?;
    Ok(())
}

// 스트라이크와 볼을 계산하는 보조 함수
fn count_strike_ball(target: &str, query: &str) -> (i32, i32) {
    let t_bytes = target.as_bytes();
    let q_bytes = query.as_bytes();
    
    let mut strike = 0;
    let mut ball = 0;

    for i in 0..3 {
        for j in 0..3 {
            if t_bytes[i] == q_bytes[j] {
                if i == j {
                    strike += 1;
                } else {
                    ball += 1;
                }
            }
        }
    }
    (strike, ball)
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