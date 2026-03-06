// https://www.hackerrank.com/challenges/apple-and-orange/problem
fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {

    let apple_count = apples
        .iter()
        .filter(|&&d| {
            let pos = a + d;
            pos >= s && pos <= t
        })
        .count() as i32;

    let orange_count = oranges
        .iter()
        .filter(|&&d| {
            let pos = b + d;
            pos >= s && pos <= t
        })
        .count() as i32;

    (apple_count, orange_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        let apples = [-2, 2, 1];
        let oranges = [5, -6];

        let result = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);

        assert_eq!(result, (1, 1));
    }
}
//