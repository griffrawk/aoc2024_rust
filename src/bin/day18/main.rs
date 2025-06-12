
fn main() {}

#[cfg(test)]
mod tests {
    use crate::vm::VM;

    #[test]
    fn test_part_one_test() {
        let mut vm = VM::new("src/bin/day17/data/day17_test.txt");
        assert_eq!(vm.run(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_one_data() {
        let mut vm = VM::new("src/bin/day17/data/day17_data.txt");
        assert_eq!(vm.run(), "6,5,7,4,5,7,3,1,0");
    }

    #[test]
    fn test_part_two_test() {
        let mut vm = VM::new("src/bin/day17/data/day17_test_part2.txt");
        assert_eq!(vm.repeat_run(0, 2000000), Some(117440));
    }

    #[test]
    fn test_part_two_data() {
        let mut vm = VM::new("src/bin/day17/data/day17_data.txt");
        let fred: u64 = 0o3553461305751420;
        let bob = fred - 0;
        let alice = fred + 10;
        
        assert_eq!(vm.repeat_run(bob, alice), Some(117440));
    }
}