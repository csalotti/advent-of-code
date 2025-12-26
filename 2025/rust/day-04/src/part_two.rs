use std::mem::replace;

pub fn remove_rolls(rolls: &[u8], offsets: [isize; 8], row_size: isize) -> (u64, String) {
    let mut rolls_updated: Vec<&str> = Vec::new();

    let mut n_removed = 0;
    for i in 0..rolls.len() {
        if rolls[i] != b'@' {
            rolls_updated.push(".");
            continue;
        }
        let mut count = 0;
        for off in offsets {
            let adj_pos = (i as isize) + off;
            // Out of bound
            if (adj_pos < 0) || (adj_pos >= (rolls.len() as isize)) {
                continue;
            }
            // Borders
            if ((i as isize % row_size) - (adj_pos % row_size)).abs() > 1 {
                continue;
            }
            if rolls[adj_pos as usize] == b'@' {
                count += 1;
            }
        }
        if count < 4 {
            rolls_updated.push("x");
            n_removed += 1;
        } else {
            rolls_updated.push("@");
        }
    }
    let debug: String = rolls_updated
        .chunks_exact(row_size as usize)
        .map(|c| c.concat())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{debug}\n");

    (n_removed, rolls_updated.join(""))
}

pub fn process(input: &'static str) -> u64 {
    let rows: Vec<&str> = input.trim().split("\n").map(|l| l.trim()).collect();
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
    let mut rolls_flatten = rows.join("");

    let mut total_removed = 0;
    loop {
        let (removed, rolls_updated) = remove_rolls(rolls_flatten.as_bytes(), offsets, row_size);
        total_removed += removed;
        let _ = replace(&mut rolls_flatten, rolls_updated);

        if removed == 0 {
            return total_removed;
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_04_part_two() {
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
        assert_eq!(process(input), 43);
    }
}
