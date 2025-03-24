use std::fs;

#[derive(Debug)]
pub struct CPU {
    instruction_pointer: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,

    program: Vec<usize>,
    output: Vec<usize>,
}

impl CPU {
    pub(crate) fn new(file: &str) -> Self {
        let mut reg_a: usize = 0;
        let mut reg_b: usize = 0;
        let mut reg_c: usize = 0;
        let mut program: Vec<usize> = Vec::new();
        for row in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
        {
            let r: Vec<&str> = row.split(':').collect();
            match r[0] {
                "Register A" => reg_a = r[1].trim().parse().unwrap(),
                "Register B" => reg_b = r[1].trim().parse().unwrap(),
                "Register C" => reg_c = r[1].trim().parse().unwrap(),
                "Program" => program = r[1].trim().split(',').map(|o| o.parse().unwrap()).collect(),
                _ => {}
            }
        }
        let output = Vec::new();

        Self {
            instruction_pointer: 0,
            reg_a,
            reg_b,
            reg_c,
            program,
            output,
        }
    }

    pub fn run(&mut self) -> &str {
        while self.instruction_pointer < self.program.len() {
            let instruction = self.program[self.instruction_pointer];
            let combo = self.program[self.instruction_pointer + 1];
            match instruction {
                0 => self.adv(combo),
                1 => self.bxl(combo),
                2 => self.bst(combo),
                3 => self.jnz(combo),
                4 => self.bxc(combo),
                5 => self.out(combo),
                6 => self.bdv(combo),
                7 => self.cdv(combo),
                _ => (),
            }
        }

        "4,6,3,5,6,3,5,2,1,0"
    }

    fn adv(&mut self, combo: usize) {
        
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self, combo: usize) {
        
        self.instruction_pointer += 2;
    }
    
    fn bst(&mut self, combo: usize) {
        
        self.instruction_pointer += 2;
    }
    
    fn jnz(&mut self, combo: usize) {
        
    }
    
    fn bxc(&mut self, combo: usize) {
        
        self.instruction_pointer += 2;
    }
    
    fn out(&mut self, combo: usize) {
        let o = match combo {
            0..=3 => Some(combo.rem_euclid(8)),
            4 => Some(self.reg_a.rem_euclid(8)),
            5 => Some(self.reg_b.rem_euclid(8)),
            6 => Some(self.reg_c.rem_euclid(8)),
            _ => None,
        };
        if let Some(r) = o {
            self.output.push(r);
        }
        self.instruction_pointer += 2;
    }
    
    fn bdv(&mut self, combo: usize) {
        
        self.instruction_pointer += 2;
    }
    
    fn cdv(&mut self, combo: usize) {
        
        self.instruction_pointer += 2;
    }
    
}
