use std::{cmp, fs::File, io::Read};

use z3::{
    Optimize,
    ast::{Bool, Int},
};

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn get_minimum_lights(
        &self,
        current: usize,
        current_lights: &mut Vec<bool>,
        pressed_buttons: usize,
    ) -> usize {
        if current_lights == &self.lights {
            return pressed_buttons;
        }

        if current == self.buttons.len() {
            return usize::MAX;
        }

        let without_current = self.get_minimum_lights(current + 1, current_lights, pressed_buttons);
        for button in &self.buttons[current] {
            current_lights[*button] = !current_lights[*button];
        }

        let with_current =
            self.get_minimum_lights(current + 1, current_lights, pressed_buttons + 1);
        for button in &self.buttons[current] {
            current_lights[*button] = !current_lights[*button];
        }
        return cmp::min(with_current, without_current);
    }

    fn get_minimum_joltage(&self) -> u64 {
        let button_asts = (0..self.buttons.len())
            .map(|b| Int::fresh_const(&format!("b{}", b)))
            .collect::<Vec<_>>();
        let solver = Optimize::new();

        // button presses need to be >= 0
        button_asts.iter().for_each(|ba| solver.assert(&ba.ge(0)));

        // generate sums of button clicks for each joltage
        let mut sums = vec![Int::from_u64(0); self.joltages.len()];
        for (i, button) in self.buttons.iter().enumerate() {
            for joltage_id in button {
                sums[*joltage_id] = &sums[*joltage_id] + &button_asts[i];
            }
        }

        //add sum constraints
        sums.iter()
            .zip(&self.joltages)
            .for_each(|(sum, joltage)| solver.assert(&sum.eq(*joltage as u64)));

        solver.minimize(&Int::add(&button_asts));
        let _result = solver.check(&[]);
        let model = solver.get_model().unwrap();

        button_asts
            .iter()
            .map(|button_ast| model.eval(button_ast, true).unwrap().as_u64().unwrap())
            .sum()
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let splitted = line.split(" ").collect::<Vec<_>>();
            let lights_str = splitted.first().unwrap();
            let buttons = splitted[1..splitted.len() - 1].iter();
            let joltages_str = splitted.last().unwrap();
            Machine {
                lights: lights_str[1..lights_str.len() - 1]
                    .chars()
                    .map(|ch| match ch {
                        '#' => true,
                        '.' => false,
                        ch => panic!("Unknown character {}", ch),
                    })
                    .collect(),
                buttons: buttons
                    .map(|button| {
                        button[1..button.len() - 1]
                            .split(",")
                            .map(|light| light.parse().unwrap())
                            .collect()
                    })
                    .collect(),
                joltages: joltages_str[1..joltages_str.len() - 1]
                    .split(",")
                    .map(|j| j.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let machines = parse(&input);
    let min_lights = machines
        .iter()
        .map(|m| m.get_minimum_lights(0, &mut vec![false; m.lights.len()], 0))
        .sum::<usize>();
    println!("{}", min_lights);
    println!(
        "{}",
        machines
            .iter()
            .map(Machine::get_minimum_joltage)
            .sum::<u64>()
    )
}
