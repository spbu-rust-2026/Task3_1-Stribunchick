pub fn mirror_double(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().rev().map(|x| *x * 2).collect()
}
