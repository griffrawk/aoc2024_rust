use std::cmp::max;

// Function to rotate matrix by 45 degree
// not very efficient though
// for an 8*8 array it hits the push 1:14
// for 9*9 it's 1:16
// so the loss gets worse as size increases
// for a big array in the data file, it will be huge
//
// another problem, if the array isn't square it panics with out-of-bounds errors
#[allow(dead_code)]
fn rotate_45_nope(li: Vec<Vec<usize>>) {
    let n = li.len();
    let m = li[0].len();
    let mut hits = 0;
    let mut misses = 0;
    let mut ctr: usize = 0;
    while ctr < 2 * n - 1 {
        let mut lst: Vec<usize> = Vec::new();
        for i in 0..m as usize{
            for j in 0..n as usize{
                if i + j == ctr {
                    hits += 1;
                    lst.push(li[i][j])
                } else {
                    misses += 1;
                }
            }
        }
        println!("{:?}", lst);
        ctr += 1;
    }
    println!("Hits: {}, Misses: {}", hits, misses);
}

// works with non-square arrays
fn rotate_45_alt(li: Vec<Vec<usize>>) {
    let max_x = li[0].len();
    let max_y =li.len();
    let n = max(max_y, max_x);
    println!("First half\n");
    for base_y in 0..n {
        let mut y= base_y;
        for x in 0..=base_y {
            // need to check bounds and ignore
            if x < max_x && y < max_y {
                println!("x: {} y: {}, li {}", x, y, li[y][x]);
            }
            if y > 0 {y -= 1}
        }
        print!("\n");
    }
    println!("Second half\n");
    for base_x in 1..n {
        let mut x = base_x;
        for y in (base_x..n).rev()  {
            if x < max_x && y < max_y {
                println!("x: {} y: {}, li {}", x, y, li[y][x]);
            }
            x += 1;
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use crate::day_04::rotate_45::rotate_45_alt;

    #[test]
    fn test_rotate_45_alt() {

        let li = vec![
            vec![3, 9, 4, 5, 6, 9, 8],
            vec![3, 9, 4, 5, 6, 9, 8],
            vec![3, 9, 4, 5, 6, 9, 8],
            vec![3, 9, 4, 5, 6, 9, 8],
            vec![2, 3, 6, 3, 8, 1, 4],
        ];
        rotate_45_alt(li);

    }
}