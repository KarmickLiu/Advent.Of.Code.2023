use std::collections::HashMap;

fn main() {
    let contents = include_str!("./input/day3.txt");

    let mut row: i32 = 0;
    let mut col: i32 = 0;

    let mut pos_map: HashMap<(i32, i32), char> = HashMap::new();
    let mut symbol_pos: Vec<(i32, i32)> = Vec::new();
    let mut adj_num_pos: Vec<(i32, i32)> = Vec::new();
    let mut ast_adj_num_pos: Vec<[(i32, i32); 2]> = Vec::new();

    for line in contents.lines() {
        for char in line.chars() {
            pos_map.insert((col, row), char);

            if !(char.is_digit(10) || char == '.') {
                symbol_pos.push((col, row));
            }
            col += 1;
        }
        col = 0;
        row += 1;
    }

    for (col, row) in symbol_pos.clone() {
        let mut count:u32 = 0;

        let mut pair:[(i32, i32); 2] = [(-1, -1), (-1, -1)];

        for j in row - 1..row + 2 {
            for i in col - 1..col + 2 {
                if !(i == col && j == row) {
                    if pos_map.get(&(i, j)).map_or(false, |c| c.is_digit(10)) {
                        let last_element: Option<&(i32, i32)> = adj_num_pos.last();
                        match last_element {
                            Some((last_col, last_row)) => {
                                if !((i, j) == (last_col + 1, *last_row)) &&
                                    !((i, j) == (last_col + 2, *last_row) && pos_map.get(&(i - 1, j)).map_or(false, |c| c.is_digit(10)))
                                {
                                    if !adj_num_pos.contains(&(i, j)) {
                                        adj_num_pos.push((i, j));

                                        if pos_map.get(&(col, row)).unwrap() == &'*' {
                                            count += 1;
                                            match count {
                                                1 => pair[0] = (i, j),
                                                2 => pair[1] = (i, j),
                                                _ => unreachable!("bad count!")
                                            }
                                        }
                                    }
                                }
                            }
                            None => {
                                adj_num_pos.push((i, j));
                            }
                        }
                    }
                }
                if pair[1] != (-1, -1) && !ast_adj_num_pos.contains(&pair) {
                    //println!("{:?}", pair);
                    ast_adj_num_pos.push(pair);
                }
            }
        }
        count = 0;
    }

    fn sum_numbers(v:Vec<(i32, i32)>, h:HashMap<(i32, i32), char>) {
        let mut sum: u32 = 0;

        for (col, row) in v {
            let num = find_number(col, row, &h).unwrap();

            sum = sum + num;
        }
        
        // Part One
        println!("{}", sum);
    }

    sum_numbers(adj_num_pos, pos_map.clone());

    let mut mul_sum:u32 = 0;

    for pair in ast_adj_num_pos {
        let mut mul:u32 = 1;
        
        for (col, row) in pair {
            let number:u32 = find_number(col, row, &pos_map).unwrap();

            mul = mul * number;
        }
        mul_sum += mul;
    }

    // Part Two
    println!("{}", mul_sum);

    // refactor this
    fn find_number(col:i32, row:i32, h:&HashMap<(i32, i32), char>) -> Option<u32> {
        let mut ones: u32 = 0;
        let mut tens: u32 = 0;
        let mut hundreds: u32 = 0;

        if h.get(&(col - 1, row))?.is_digit(10) {
            if h.get(&(col - 2, row))?.is_digit(10) {
                hundreds = h.get(&(col - 2, row))?.to_digit(10)?;
                tens = h.get(&(col - 1, row))?.to_digit(10)?;
                ones = h.get(&(col, row))?.to_digit(10)?;
            } else {
                if h.get(&(col + 1, row))?.is_digit(10) {
                    hundreds = h.get(&(col - 1, row))?.to_digit(10)?;
                    tens = h.get(&(col, row))?.to_digit(10)?;
                    ones = h.get(&(col + 1, row))?.to_digit(10)?;
                } else {
                    tens = h.get(&(col - 1, row))?.to_digit(10)?;
                    ones = h.get(&(col, row))?.to_digit(10)?;
                }
            }
        } else {
            if h.get(&(col + 1, row))?.is_digit(10) {
                if h.get(&(col + 2, row))?.is_digit(10) {
                    hundreds = h.get(&(col, row))?.to_digit(10)?;
                    tens = h.get(&(col + 1, row))?.to_digit(10)?;
                    ones = h.get(&(col + 2, row))?.to_digit(10)?;
                } else {
                    tens = h.get(&(col, row))?.to_digit(10)?;
                    ones = h.get(&(col + 1, row))?.to_digit(10)?;
                }
            } else {
                ones = h.get(&(col, row))?.to_digit(10)?;
            }
        }
        Some(hundreds * 100 + tens * 10 + ones)
    }
}
