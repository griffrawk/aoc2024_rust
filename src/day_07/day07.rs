use std::fs;

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
    Con,
}

pub fn binary_operations(bits: usize) -> Vec<Vec<Operator>> {
    let mut opsteps: Vec<Vec<Operator>> = Vec::new();
    let z = (bits as f32).exp2() as usize;
    for n in 0..z {
        let y = format!("{:0b$b}", n, b = bits);
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
fn operations(bits: usize) -> Vec<Vec<Operator>> {
    let mut opsteps: Vec<Vec<Operator>> = Vec::new();
    let mut opseq: Vec<Operator> = Vec::new();
    opsteps.push(opseq);

    // www

    //  add add add
    //          mul
    //          con
    //      mul add
    //          mul
    //          con
    //      con add
    //          mul
    //          con     add con con might be a prob, the last con is redundant
    //

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
                    Operator::Con => {}
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
    use crate::day_07::day07::{operations, part_one};

    #[test]
    fn playing() {
        let fred = String::from("abc123");
        let mut john: &str = "xyz987";
        let mut janet = String::from(fred);
        for c in john.chars() {
            println!("{}", c);
        }
        println!("{}", janet);
    }

    #[test]
    fn test_operations() {
        dbg!(operations(2));
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
