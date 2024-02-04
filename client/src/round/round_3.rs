// Matrix Fiesta
// Parse Matrix if the line number is an odd number find the greatest number within each row if the
// line number is and even number find th greatest number within each column in either case find
// the product of each great number and retrieve its sum of digits to form the super number the
// task is to return the sum of all such super numbers

pub(super) fn compute(input_string: &str) -> i32 {
    for line in input_string.lines() {
        let line = line.trim_matches(|c| c=='[' || c==']');
        let mut matrix: Vec<Vec<u32>> = Vec::new();
        let mut rows: Vec<u32> = Vec::new();
        for chr in line.chars() {
            if chr == '(' { continue }

            if chr == ')' {
                matrix.push(rows.clone());
            } else {
                rows.push(chr.to_digit(10).unwrap())
            }
        }
        println!("{matrix:?}");
    }
    0
}
