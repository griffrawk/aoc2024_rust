use std::fs;
use num::ToPrimitive;

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

    pub fn run(&mut self) -> String {
        while self.instruction_pointer < self.program.len() {
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
            .trim_end_matches(",")
            .to_string()
        // "4,6,3,5,6,3,5,2,1,0"
    }

    // opcode 0 - divide A
    fn adv(&mut self, combo: usize) {
        if let Some(o) = self.combo_value(combo) {
            self.reg_a = (self.reg_a as i32 / 2i32.pow(o as u32)).to_usize().unwrap();
        }
        self.instruction_pointer += 2;
    }

    // opcode 1 - B XOR literal
    fn bxl(&mut self, literal: usize) { 
        self.reg_b = self.reg_b ^ literal;
        self.instruction_pointer += 2;
    }

    // opcode 3 - reg_b = combo mod 8
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
        if let Some(o) = self.combo_value(combo) {
            self.output.push(o.rem_euclid(8));
        }
        self.instruction_pointer += 2;
    }

    fn bdv(&mut self, combo: usize) {
        if let Some(o) = self.combo_value(combo) {
            self.reg_b = (self.reg_a as i32 / 2i32.pow(o as u32)).to_usize().unwrap();
        }
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self, combo: usize) {
        if let Some(o) = self.combo_value(combo) {
            self.reg_c = (self.reg_a as i32 / 2i32.pow(o as u32)).to_usize().unwrap();
        }
        self.instruction_pointer += 2;
    }
    
    fn combo_value(&mut self, combo: usize) -> Option<usize> {
        match combo {
            0..=3 => Some(combo),
            4 => Some(self.reg_a),
            5 => Some(self.reg_b),
            6 => Some(self.reg_c),
            _ => None,
        }
    }
}
