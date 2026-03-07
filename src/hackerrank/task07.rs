// https://www.hackerrank.com/challenges/between-two-sets/problem
pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    fn lcm(a: i32, b: i32) -> i32 {
        a * b / gcd(a, b)
    }

    let mut l = a[0];
    for &x in a.iter().skip(1) { l = lcm(l, x) }

    let mut g = b[0];
    for &x in b.iter().skip(1) { g = gcd(g, x) }

    (1..=g / l).filter(|&k| g % (l * k) == 0).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_cases() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
        assert_eq!(get_total_x(&[3, 4], &[24, 48]), 2);
        assert_eq!(get_total_x(&[2, 3, 6], &[42, 84]), 2);
        assert_eq!(get_total_x(&[2, 3], &[7, 11]), 0);
        assert_eq!(get_total_x(&[1], &[100]), 9);
    }
}