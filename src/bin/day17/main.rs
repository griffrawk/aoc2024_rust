mod cpu;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    #[test]
    fn test_part_one_test() {
        let mut cpu = CPU::new("src/bin/day17/data/day17_test.txt");
        assert_eq!(cpu.run(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_one_data() {
        let mut cpu = CPU::new("src/bin/day17/data/day17_data.txt");
        assert_eq!(cpu.run(), "6,5,7,4,5,7,3,1,0");
    }
}