use std::fs;

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
    Con,
}

fn binary_operations(bits: usize) -> Vec<Vec<Operator>> {
    let mut opsteps: Vec<Vec<Operator>> = Vec::new();
    let z = 2u32.pow(bits as u32);
    for n in 0..z {
        let y = format!("{:0b$b}", n, b = bits as usize);
        let mut opseq: Vec<Operator> = Vec::new();
        for s in y.chars() {
            match s {
                '0' => opseq.push(Operator::Add),
                '1' => opseq.push(Operator::Mul),
                _ => panic!(),
            }
        }
        opsteps.push(opseq);
    }
    opsteps
}

// need another way to do operations for 3 states. Jost talked about a recursive
// method, so might need to have a peek at is code
fn ternary_operations(digits: usize) -> Vec<Vec<Operator>> {
    let mut opsteps: Vec<Vec<Operator>> = Vec::new();
    let base: u32 = 3;
    let z = base.pow(digits as u32);
    // for n in 0..z {
        // let mut opseq: Vec<Operator> = Vec::new();
        // convert n base 10 to base 3, but this only works for n > 0 so we are ignoring 0
        // and not filling in the leading zeros
        // let mut i = n;
        // let mut seq = String::new();
        // while i > 0 {
        //     let rem = i % base;
        //     i /= base;
        //     seq.push(char::try_from(rem).unwrap())
        //     }
        // }
        // for s in y.chars() {
        //     match s {
        //         '0' => opseq.push(Operator::Add),
        //         '1' => opseq.push(Operator::Mul),
        //         _ => panic!(),
        //     }
        // }
        // opsteps.push(opseq);
    // }
    opsteps
}

#[allow(dead_code)]
fn part_one(file: &str) -> i64 {
    let mut res = 0;
    let contents: String = fs::read_to_string(file).expect("Can't read the file");
    for line in contents.lines() {
        let eqn: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.replace(":", "").parse::<i64>().ok())
            .collect();
        // result is eqn[0], others are operands, so we need as many opsteps
        // as there are gaps between operands
        let opsteps = binary_operations(eqn.len() - 2);
        for opseq in opsteps {
            let mut sum: i64 = eqn[1];
            for (idx, op) in opseq.into_iter().enumerate() {
                match op {
                    Operator::Add => sum += eqn[idx + 2],
                    Operator::Mul => sum *= eqn[idx + 2],
                    _ => {},
                }
            }
            if sum == eqn[0] {
                res += sum;
                break;
            }
        }
    }
    res
}

fn part_two(file: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day_07::day07::{part_one, ternary_operations};

    #[test]
    fn test_operations() {
        dbg!(ternary_operations(3));
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_07/day07_test.txt");
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_07/day07_data.txt");
        assert_eq!(result, 1430271835320);
    }
}
