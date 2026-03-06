fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    grades
        .into_iter()
        .map(|g| {
            if g < 38 {
                g
            } else {
                let next = ((g / 5) + 1) * 5;
                if next - g < 3 { next } else { g }
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(
            grading_students(vec![73, 67, 38, 33]),
            vec![75, 67, 40, 33]
        );
    }
}
//