// Matrix Fiesta
// Parse Matrix if the line number is an odd number find the greatest number within each row if the
// line number is and even number find th greatest number within each column in either case find
// the product of each great number and retrieve its sum of digits to form the super number the
// task is to return the sum of all such super numbers

pub(super) fn compute(input_string: &str) -> u32 {
    let mut super_sum = 0;

    for (line_number, line) in input_string.lines().enumerate() {
        // println!("no: {line_number}, line:{line}");
        let line = line.trim_matches(|c| c=='[' || c==']');
        let mut matrix: Vec<Vec<u32>> = Vec::new();

        get_matrix(line, &mut matrix);

        let mut max: Vec<u32> = Vec::new();
        // enumerate starts from zero so yeah
        if (line_number + 1) % 2 == 0 {
            // transpose
            let mut new_matrix: Vec<Vec<u32>> = vec![vec![0 ; matrix.len()]; matrix[0].len()];

            for (i, row) in matrix.clone().iter().enumerate() {
                for (j, _) in row.iter().enumerate() {
                    new_matrix[j][i] = matrix[i][j];
                }
            }
            matrix = new_matrix
        } 

        for row in matrix {
            max.push(*row.iter().max().unwrap());
            // println!("{}", max.last().unwrap());
        }
        let product = max.iter().product::<u32>();
        super_sum += sum_of_digits(product);
    }
    super_sum
}

fn sum_of_digits(product: u32) -> u32 {
    if product.to_string().len() <= 1 { return product }
    product
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum::<u32>()
}

fn get_matrix(line: &str, matrix: &mut Vec<Vec<u32>>) {
    let mut rows: Vec<u32> = Vec::new();
    for chr in line.chars() {
        if chr == '(' { continue }
        if chr == ')' {
            matrix.push(rows.clone());
            rows.clear();
        } else {
            rows.push(chr.to_digit(10).unwrap())
        }
    }
}
