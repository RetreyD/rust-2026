// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
pub fn breaking_records(scores: &[i32]) -> (i32, i32) {
    let (mut best, mut worst) = (scores[0], scores[0]);
    let (mut best_count, mut worst_count) = (0, 0);

    for &score in &scores[1..] {
        if score > best {
            best = score;
            best_count += 1;
        }
        if score < worst {
            worst = score;
            worst_count += 1;
        }
    }

    (best_count, worst_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_cases() {
        assert_eq!(breaking_records(&[10, 5, 20, 20, 4, 5, 2, 25, 1]), (2, 4));
        assert_eq!(breaking_records(&[3, 4, 21, 36, 10, 28, 35, 5, 24, 42]), (4, 0));
        assert_eq!(breaking_records(&[100, 100, 100]), (0, 0));
        assert_eq!(breaking_records(&[5, 1, 2, 10, 1]), (1, 1));
    }
}