use std::fs::File;
use std::io::Read;

struct Computer {
    program: Vec<u64>,
    instruction_pointer: usize,
    a: u64,
    b: u64,
    c: u64,
}

impl Computer {
    fn run(&mut self) -> Vec<u64> {
        let mut output = Vec::new();
        while self.instruction_pointer < self.program.len() {
            let literal_operand = self.program[self.instruction_pointer + 1];
            let combo_operand = match literal_operand {
                0 | 1 | 2 | 3 => literal_operand,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                7 => panic!("Operand 7 is reserved and shouldn't occur in valid programs."),
                _ => panic!("Unknown operand {}", literal_operand)
            };

            let instruction = self.program[self.instruction_pointer];
            match instruction {
                0 => self.adv(combo_operand),
                1 => self.bxl(literal_operand),
                2 => self.bst(combo_operand),
                3 => self.jnz(literal_operand),
                4 => self.bxc(),
                5 => output.push(self.out(combo_operand)),
                6 => self.bdv(combo_operand),
                7 => self.cdv(combo_operand),
                _ => panic!("Unknown instruction {}", self.program[self.instruction_pointer])
            }

            if instruction != 3 || (instruction == 3 && self.a == 0) {
                self.instruction_pointer += 2;
            }
        }

        output
    }

    fn adv(&mut self, operand: u64) {
        // Currently, if operand is greater than
        // the number of bytes, it does a cyclic shift
        self.a = self.a / (1 << (operand % 64));
    }

    fn bxl(&mut self, operand: u64) {
        self.b = self.b ^ operand;
    }

    fn bst(&mut self, operand: u64) {
        self.b = operand % 8;
    }

    fn jnz(&mut self, operand: u64) {
        if self.a == 0 {
            return;
        }

        self.instruction_pointer = operand as usize;
    }

    fn bxc(&mut self) {
        self.b = self.b ^ self.c;
    }

    fn out(&self, operand: u64) -> u64 {
        operand % 8
    }

    fn bdv(&mut self, operand: u64) {
        // Currently, if operand is greater than
        // the number of bytes, it does a cyclic shift
        self.b = self.a / (1 << (operand % 64));
    }

    fn cdv(&mut self, operand: u64) {
        // Currently, if operand is greater than
        // the number of bytes, it does a cyclic shift
        self.c = self.a / (1 << (operand % 64));
    }
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let mut splitted = value.split("\n\n");
        let registers = splitted
            .next()
            .unwrap()
            .lines()
            .map(|line| line.split(": ").nth(1).unwrap().parse().unwrap())
            .collect::<Vec<_>>();

        let program = splitted.next().unwrap().split(": ").nth(1).unwrap().split(",").map(|byte| byte.parse().unwrap()).collect();

        Self {
            program,
            instruction_pointer: 0,
            a: registers[0],
            b: registers[1],
            c: registers[2],
        }
    }
}

fn solve_backwards(a: u64, program: &Vec<u64>, current: i64, stop: i64) -> Vec<u64> {
    if current < stop  {
        return vec![a];
    }

    let mut possible_numbers = Vec::new();
    for last_digits in 0..8 {
        let next_a = (a << 3) + last_digits;
        let mut b = last_digits;
        b = b ^ 1;
        let c = next_a >> b;
        b = b ^ c;
        b = b ^ 4;
        if b % 8 == program[current as usize] {
            let next_a = (a << 3) + last_digits;
            possible_numbers.extend(solve_backwards(next_a, program, current - 1, stop));
        }
    }

    possible_numbers
}

fn part_two(program: &Vec<u64>) -> u64 {
    let mut possible_numbers = solve_backwards(0, program, (program.len() - 1) as i64, 0);
    println!("{:?}", possible_numbers);
    possible_numbers.len() as u64
}

fn main() {
    let mut content = String::new();
    File::open("input").unwrap().read_to_string(&mut content).unwrap();
    let mut computer = Computer::from(&content[..]);
    let output = computer.run();
    println!("{}", output.into_iter().map(|o| o.to_string()).collect::<Vec<_>>().join(","));
    println!("{}", part_two(&computer.program));
}
