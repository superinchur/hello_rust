//! # 피보나치 계산 프로그램
//! 이 모듈은 백준 10870번 문제 해결을 위한 
//! 피보나치 수열 계산 로직을 담고 있습니다.

/// 문제
//피보나치 수는 0과 1로 시작한다. 0번째 피보나치 수는 0이고, 1번째 피보나치 수는 1이다. 그 다음 2번째 부터는 바로 앞 두 피보나치 수의 합이 된다.
//이를 식으로 써보면 Fn = Fn-1 + Fn-2 (n ≥ 2)가 된다.
//n=17일때 까지 피보나치 수를 써보면 다음과 같다.
//0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597
//n이 주어졌을 때, n번째 피보나치 수를 구하는 프로그램을 작성하시오.
//입력
//첫째 줄에 n이 주어진다. n은 20보다 작거나 같은 자연수 또는 0이다.
//출력
//첫째 줄에 n번째 피보나치 수를 출력한다.

use std::io;

fn main() {
    // 1. 사용자 입력 받기
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력을 읽지 못했습니다.");
    
    // 2. 숫자로 변환 (C#의 int.Parse와 유사하지만 Result 타입을 반환함)
    let n: usize = input.trim().parse().expect("숫자를 입력해주세요.");

    // 3. 피보나치 계산
    let result = fibo(n);
    
    // 4. 결과 출력
    println!("{}", result);
}


/// 0부터 n까지의 피보나치 수열 전체를 For문을 사용해서 수행
///
/// # Arguments
/// * `n` - 계산하고자 하는 순서 (0 ~ 20)
///
/// # Returns
/// * 계산된 피보나치 수 (u32 타입)
/// 
/// # Panics
/// * `n`이 20을 초과하면 패닉이 발생합니다.
///
/// # Examples
/// ```
/// let res = fibo(10);
/// assert_eq!(res, 55);
/// ```
fn fibo(n: usize) -> u32 {

    // 2.5 panic! 사용 (비즈니스 로직 검사)
    if n > 20 {
        panic!("피보나치는 20까지만 계산 가능합니다.");
    }
    
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n { // 2부터 n까지 포함(inclusive)
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}


/// 0부터 n까지의 피보나치 수열 전체를 Recursive를 사용해서 수행
fn fibo_recursive(n: usize) -> u32 {   
    match n {
        // base case
        0 => 0,
        1 => 1,
        // recursive case
        _ => fibo_recursive(n - 1) + fibo_recursive(n - 2),
    }
}

// 전형적인 메모이제이션(재귀 + 캐시) 구조
fn fibo_memo(n: usize, cache: &mut Vec<i32>) -> i32 {
    if cache[n] != -1 { return cache[n]; } // 이미 있으면 반환
    cache[n] = fibo_memo(n - 1, cache) + fibo_memo(n - 2, cache); // 없으면 재귀로 계산
    cache[n]
}

/// 0부터 n까지의 피보나치 수열 전체를 배열에 넣고 사용해서 수행
fn fibo_custom(n: usize) -> u32 {   
    // 1. 초기값이 들어있는 벡터 생성 (C#의 new List<uint> { 0, 1 })
    let mut fib_list = vec![0, 1];

    // 2. n까지 계산하며 배열에 추가
    for i in 2..=n {
        // 배열의 이전 값들을 참조하여 새 값 계산
        let next_val = fib_list[i - 1] + fib_list[i - 2];
        fib_list.push(next_val);
    }

    fib_list[n]
}

#[cfg(test)] // 테스트할 때만 이 모듈을 컴파일하라는 뜻
mod tests {
    use super::*; // 부모 모듈(위의 fibonacci 함수)을 가져옴
    use std::time::Instant; // 시간 측정을 위한 도구 가져오기

    #[test] // 이 함수가 테스트 케이스임을 나타냄
    fn test_fibonacci() {
        // assert_eq!(결과값, 기대값)
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo_recursive(2), 1);
        assert_eq!(fibo_recursive(3), 2);
        assert_eq!(fibo_custom(10), 55);
        assert_eq!(fibo_custom(17), 1597);
    }

    #[test]
    #[should_panic(expected = "피보나치는 20까지만 계산 가능합니다.")] // 특정 에러 메시지가 포함되어야 성공
    fn test_invalid_input() {
        // 만약 입력값 검증 로직이 있다면 여기서 테스트합니다.
        fibo(21); // 여기서 패닉이 발생해야 이 테스트는 'Pass'가 됩니다.
    }

    #[test]
    fn test_fibo_performance() {
        let n = 20;
        
        // 1. 측정 시작
        let start = Instant::now();
        
        // 2. 알고리즘 실행
        let result = fibo_custom(n);
        
        // 3. 측정 종료 및 경과 시간 계산
        let duration = start.elapsed();
        
        println!("\n[성능 테스트 결과]");
        println!("입력값: n = {}", n);
        println!("결과값: {}", result);
        println!("실행 시간: {:?}", duration); // 마이크로초(µs)나 나노초(ns) 단위까지 나옵니다.

        assert_eq!(result, 6765);
    }
}