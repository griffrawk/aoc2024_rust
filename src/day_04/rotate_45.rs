// #include <bits/stdc++.h>
// using namespace std;

// Function to rotate matrix by 45 degree
fn rotate_45(li: Vec<Vec<usize>>) {
    let n = li.len();
    let m = li[0].len();
    let mut ctr: usize = 0;
    while ctr < 2 * n - 1 {
        let mut lst: Vec<usize> = Vec::new();
        for i in 0..m as usize{
            for j in 0..n as usize{
                if i + j == ctr {
                    lst.push(li[i][j])
                }
            }
        }
        println!("{:?}", lst);
        ctr += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::day_04::rotate_45::rotate_45;

    #[test]
    fn test_rotate_45() {
        let n = 8;
        let m = n;

        let li = vec![
            vec![4, 5, 6, 9, 8, 7, 1, 4],
            vec![1, 5, 9, 7, 5, 3, 1, 6],
            vec![7, 5, 3, 1, 5, 9, 8, 0],
            vec![6, 5, 4, 7, 8, 9, 3, 7],
            vec![3, 5, 6, 4, 8, 9, 2, 1],
            vec![3, 1, 6, 4, 7, 9, 5, 0],
            vec![8, 0, 7, 2, 3, 1, 0, 8],
            vec![7, 5, 3, 1, 5, 9, 8, 5],
        ];
        rotate_45(li);

    }
}