fn add(a: u64, b: u64) -> u64 {
    a + b
}
fn mul(a: u64, b: u64) -> u64 {
    a * b
}

pub fn process(input: &'static str) -> u64 {
    // Input processing
    let mut lines: Vec<&str> = input.trim().split("\n").map(|l| l.trim()).collect();
    let operations: Vec<&str> = lines
        .pop()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect();
    let (n_cols, n_rows) = (
        lines[0].split(" ").filter(|s| !s.is_empty()).count(),
        lines.len(),
    );
    let numbers: Vec<u64> = lines
        .join(" ")
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|e| e.trim().parse().unwrap())
        .collect();

    // Operations
    type Operator = fn(u64, u64) -> u64;
    let addition_op: Operator = add;
    let multiplication_op: Operator = mul;

    // Final
    let mut result_total: u64 = 0;
    for (i, op) in operations.into_iter().enumerate() {
        let (mut result_col, op_fn) = if op == "*" {
            (1u64, multiplication_op)
        } else {
            (0u64, addition_op)
        };
        for row in 0..n_rows {
            result_col = op_fn(result_col, numbers[(row * n_cols) + i]);
        }
        result_total += result_col;
    }
    result_total
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_06_part_one() {
        let input = "123 328  51 64
                                   45 64  387 23
                                   6 98  215 314
                                   *   +   *   +";
        assert_eq!(process(input), 4277556);
    }
}
