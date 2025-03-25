mod cpu;

fn main() {}

#[cfg(test)]
mod tests {
    use num::ToPrimitive;
    use crate::cpu::CPU;

    #[test]
    fn test_part_one_test() {
        let mut cpu = CPU::new("src/bin/day17/data/day17_test_part2.txt");
        assert_eq!(cpu.run(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_one_data() {
        let mut cpu = CPU::new("src/bin/day17/data/day17_data.txt");
        assert_eq!(cpu.run(), "6,5,7,4,5,7,3,1,0");
    }
    
    // #[test]
    // fn test_part_two_test() {
    //     let mut cpu = CPU::new("src/bin/day17/data/day17_test_part2.txt");
    //     // Save initial state for rerun
    //     let mut reg_a = -1;
    //     let reg_b = cpu.reg_b;
    //     let reg_c = cpu.reg_c;
    // 
    //     while cpu.program != cpu.output {
    //         reg_a += 1;
    //         cpu.reg_a = reg_a.to_usize().unwrap();
    //         cpu.reg_b = reg_b;
    //         cpu.reg_c = reg_c;
    //         cpu.instruction_pointer = 0;
    //         cpu.output.clear();
    //         _ = cpu.run();
    //     }
    //     assert_eq!(reg_a, 117440);
    // }

    // #[test]
    // fn test_part_two_data() {
    //     let mut cpu = CPU::new("src/bin/day17/data/day17_data.txt");
    // 
    //     let mut reg_a = 1000000000000000;
    //     let reg_b = cpu.reg_b;
    //     let reg_c = cpu.reg_c;
    //     while cpu.program != cpu.output {
    //         reg_a += 1;
    //         cpu.reg_a = reg_a;
    //         if cpu.reg_a % 100000000 == 0 {
    //             dbg!(&cpu.reg_a);
    //         }
    //         cpu.reg_b = reg_b;
    //         cpu.reg_c = reg_c;
    //         cpu.instruction_pointer = 0;
    //         cpu.output.clear();
    //         _ = cpu.run();
    //     }
    //     assert_eq!(reg_a, 117440);
    // }
    
}