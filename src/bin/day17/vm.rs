use std::fs;

#[derive(Debug)]
pub struct VM {
    instruction_pointer: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    program: Vec<usize>,
    output: Vec<usize>,
}

impl VM {
    pub(crate) fn new(file: &str) -> Self {
        let mut reg_a = 0;
        let mut reg_b = 0;
        let mut reg_c = 0;
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

    pub(crate) fn run(&mut self) -> String {
        while self.instruction_pointer < self.program.len() {
            // dbg!(&self);
            let instruction = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];
            match instruction {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => (),
            }
        }
        self.output.iter().map(|x| x.to_string() + ",")
            .collect::<String>()
            .trim_end_matches(',')
            .to_string()
    }

    fn adv(&mut self, combo: usize) {
        if let Some(o) = self.combo_value(combo) {
            self.reg_a = self.reg_a / 2u64.pow(o as u32);
        }
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self, literal: usize) { 
        self.reg_b = self.reg_b ^ literal as u64;
        self.instruction_pointer += 2;
    }

    fn bst(&mut self, combo: usize) {
        if let Some(o) = self.combo_value(combo) {
            self.reg_b = o.rem_euclid(8);
        }
        self.instruction_pointer += 2;
    }

    fn jnz(&mut self, literal: usize) {
        if self.reg_a != 0 {
            self.instruction_pointer = literal;
        } else {
            self.instruction_pointer += 2;
        }
    }

    fn bxc(&mut self, _operand: usize) {
        self.reg_b = self.reg_b ^ self.reg_c;
        self.instruction_pointer += 2;
    }

    fn out(&mut self, combo: usize) {
        // println!("      A : {:o}", self.reg_a);
        if let Some(o) = self.combo_value(combo) {
            self.output.push(o.rem_euclid(8) as usize);
        }
        self.instruction_pointer += 2;
    }

    fn bdv(&mut self, combo: usize) {
        if let Some(o) = self.combo_value(combo) {
            self.reg_b = self.reg_a / 2u64.pow(o as u32);
        }
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self, combo: usize) {
        if let Some(o) = self.combo_value(combo) {
            self.reg_c = self.reg_a / 2u64.pow(o as u32);
        }
        self.instruction_pointer += 2;
    }
    
    fn combo_value(&mut self, combo: usize) -> Option<u64> {
        match combo {
            0..=3 => Some(combo as u64),
            4 => Some(self.reg_a),
            5 => Some(self.reg_b),
            6 => Some(self.reg_c),
            _ => None,
        }
    }
    
    // The obligatory doomed brute-force approach
    pub(crate) fn repeat_run(&mut self, lower: u64, upper: u64) -> Option<u64> {
        // Save initial state for rerun
        let reg_b = self.reg_b;
        let reg_c = self.reg_c;
    
        for reg_a in lower..upper {
            self.reg_a = reg_a;
            self.reg_b = reg_b;
            self.reg_c = reg_c;
            self.instruction_pointer = 0;
            self.output.clear();
            _ = self.run();
            // println!("program : {:?}", self.program);
            // println!(" output : {:?}\n", self.output);
            if self.program == self.output {
                return Some(reg_a);
            }
        }
        None
    }
    
    fn reconstruct(&mut self) -> Option<u64> {
        
        
        
        Some(123)
    }
}
