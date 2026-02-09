//문제
//스포츠 클라이밍은 1986 년에 시작된 실내 암벽 등반 스포츠이다. 선수들은 원래 리드 클라이밍 종목에서만 겨루었는데, 1989 년에 스피드 클라이밍이 추가되었고, 
// 10 년 후인 1999 년에 볼더링 종목이 추가되었다. 올림픽 게임에서는 금, 은, 동메달을 결정하기 위하여 선수들은 세 종목에서 겨루어 종합 순위를 매긴다. 
// 종합 순위는 세 종목에서 거둔 순위를 곱한 점수로 결정된다. 
// 예를 들어, 어떤 선수가 리드에서 1 위, 스피드에서 5 위, 볼더링에서 2 위를 했다면 점수는 10 점이 된다. 곱한 점수가 낮은 선수가 종합 순위에서 앞선다.

//선수 
//$n$명의 등번호와 이들이 세 종목에서 거둔 순위가 주어질 때, 금, 은, 동메달을 받을 선수를 결정하는 프로그램을 작성하시오. 
// 두 선수의 곱한 점수가 같을 수도 있다. 
// 이 경우, 세 종목 순위의 합산 점수가 낮은 선수가 이긴다. 
// 두 선수의 곱한 점수와 합산 점수가 모두 같으면 등번호가 낮은 선수가 이긴
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::cmp::Ordering;

struct Player {
    score_mul: i32,    // 1순위: 곱 (낮을수록 좋음)
    score_sum: i32,    // 2순위: 합 (낮을수록 좋음)
    back_number: i32,  // 3순위: 등번호 (낮을수록 좋음)
}

// Ord를 구현하려면 아래 4개 트레이트가 '세트'로 구현되어야 합니다.
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.score_mul == other.score_mul && self.score_sum == other.score_sum && self.back_number == other.back_number
    }
}
impl Eq for Player {} 

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // Ord의 cmp를 호출하도록 연결
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        // 1. 핵심: then_with를 사용한 연쇄 비교
        self.score_mul.cmp(&other.score_mul) // 첫 번째 기준: 곱
            .then_with(|| self.score_sum.cmp(&other.score_sum)) // 같으면 두 번째 기준: 합
            .then_with(|| self.back_number.cmp(&other.back_number))   // 그것도 같으면 세 번째 기준: ID
    }
}

// 2. 핵심 알고리즘 로직
fn solve<R: Read, W: Write>(input: R, output: W) -> io::Result<()> {
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    // N 읽기
    let mut first_line = String::new();
    if reader.read_line(&mut first_line)? == 0 { return Ok(()); }
    let n: usize = first_line.trim().parse().unwrap_or(0);

    let mut players = Vec::with_capacity(n);
    for line in reader.lines().take(n) {
        let line = line?;
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("숫자 변환 실패"))
            .collect();
            
        if nums.len() == 4 {
            players.push(Player {
                back_number: nums[0],
                score_mul: nums[1] * nums[2] * nums[3],
                score_sum: nums[1] + nums[2] + nums[3],
            });
        }
    }

    // 3. 정렬 (derive(Ord) 덕분에 한 줄로 해결)
    players.sort();

    // 4. 결과 출력 (상위 3명 등번호만)
    for (i, p) in players.iter().take(3).enumerate() {
        write!(writer, "{}", p.back_number)?;
        if i < 2 { write!(writer, " ")?; } // 번호 사이 공백
    }
    writeln!(writer)?;

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