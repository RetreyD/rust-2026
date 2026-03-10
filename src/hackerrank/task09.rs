// https://www.hackerrank.com/challenges/migratory-birds/problem
fn migratory_birds(arr: &[i32]) -> i32 {
    let mut freq = [0; 6];

    for &x in arr {
        freq[x as usize] += 1;
    }

    let mut max = 0;
    let mut ans = 0;

    for i in 1..=5 {
        if freq[i] > max {
            max = freq[i];
            ans = i as i32;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migratory_birds_test() {
        assert_eq!(migratory_birds(&[1,4,4,4,5,3]), 4);
        assert_eq!(migratory_birds(&[1,1,2,2,3]), 1);
        assert_eq!(migratory_birds(&[2,2,3,3,3]), 3);
    }
}