Rust의 정렬 시스템 이해

🔄 비교 트레이트 계층도
Rust에서 정렬이 가능하려면 아래 4가지 트레이트가 유기적으로 연결되어야 합니다.

PartialEq / Eq: 두 값이 같은지 판단 (등호)

PartialOrd: 두 값의 크기를 비교 (partial_cmp 사용, Option 반환)

Ord: 모든 값의 순서를 보장 (cmp 사용, Ordering 반환)

⚖️ cmp 메서드와 Ordering
cmp는 두 값을 비교하여 세 가지 상태 중 하나를 반환합니다:

Ordering::Less: 작음 (<)

Ordering::Equal: 같음 (==)

Ordering::Greater: 큼 (>)

3. 커스텀 정렬 구현 방법 (3가지)
방법 A: derive(Ord) 활용 (가장 간결)
구조체 필드 순서대로 정렬됩니다. 오름차순이 기본입니다.

Rust
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Player {
    score: i32, // 1순위
    id: i32,    // 2순위
}
방법 B: sort_by_key (가장 실용적)
구조체 정의를 건드리지 않고 정렬 시점에 기준을 정합니다.

Rust
// (곱셈 점수, 합산 점수, ID) 순으로 정렬
players.sort_by_key(|p| (p.mul, p.sum, p.id));

// 만약 합산 점수만 내림차순(큰 순서)으로 하고 싶다면?
use std::cmp::Reverse;
players.sort_by_key(|p| (p.mul, Reverse(p.sum), p.id));
방법 C: impl Ord 직접 구현 (가장 강력)
복잡한 비교 로직을 구조체 안에 내장시킵니다.

Rust
impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.mul.cmp(&other.mul)
            .then_with(|| self.sum.cmp(&other.sum))
            .then_with(|| self.id.cmp(&other.id))
    }
}
// 주의: PartialOrd, PartialEq, Eq도 함께 구현해야 함 (템플릿 참고)