pub fn process(input: &'static str) -> u64 {
    let rows: Vec<&str> = input.trim().split("\n").map(|l| l.trim()).collect();
    let mut debug: Vec<&str> = Vec::new();
    let row_size = rows[0].len() as isize;
    let offsets: [isize; 8] = [
        -1 * row_size - 1,
        -1 * row_size,
        -1 * row_size + 1,
        -1,
        1,
        row_size - 1,
        row_size,
        row_size + 1,
    ];
    let rolls_flatten: Vec<char> = rows.join("").chars().collect();
    let mut n_accessible = 0;
    for i in 0..rolls_flatten.len() {
        if rolls_flatten[i] == '.' {
            debug.push(".");
            continue;
        }
        let mut count = 0;
        for off in offsets {
            let adj_pos = (i as isize) + off;
            // Out of bound
            if (adj_pos < 0) || (adj_pos >= (rolls_flatten.len() as isize)) {
                continue;
            }
            // Borders
            if ((i as isize % row_size) - (adj_pos % row_size)).abs() > 1 {
                continue;
            }
            if rolls_flatten[adj_pos as usize] == '@' {
                count += 1;
            }
        }
        if count < 4 {
            debug.push("x");
            n_accessible += 1;
        } else {
            debug.push("@");
        }
    }
    let debug_table: String = debug
        .chunks(row_size as usize)
        .map(|c| c.concat())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{debug_table}");
    n_accessible
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_04_part_one() {
        let input = "..@@.@@@@.
                                   @@@.@.@.@@
                                   @@@@@.@.@@
                                   @.@@@@..@.
                                   @@.@@@@.@@
                                   .@@@@@@@.@
                                   .@.@.@.@@@
                                   @.@@@.@@@@
                                   .@@@@@@@@.
                                   @.@.@@@.@.";
        assert_eq!(process(input), 13);
    }
}
