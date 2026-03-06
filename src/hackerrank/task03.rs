// https://www.hackerrank.com/challenges/staircase/problem
fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = (n - i) as usize;
        let hashes = i as usize;

        let line = " ".repeat(spaces) + &"#".repeat(hashes);
        println!("{}", line);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_staircase_4() {
    
        staircase(4);
    }
}