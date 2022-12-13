pub mod day5 {
    use std::fmt::Debug;


    
    #[cfg(windows)]
    const LINE_ENDING: &'static str = "\r\n";
    #[cfg(not(windows))]
    const LINE_ENDING: &'static str = "\n";

    pub struct Instructions {
        count: usize,
        start_stack: usize,
        end_stack: usize
    }

    impl Debug for Instructions {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Instructions").field("count", &self.count).field("start_stack", &self.start_stack).field("end_stack", &self.end_stack).finish()
        }
    }

    #[warn(dead_code)]
    #[cfg(test)]
    pub fn solve_part_one(input: &str) {


        let split_format = format!("{}{}", LINE_ENDING, LINE_ENDING);
        let mut splitted_input = input.split(split_format.as_str());
        let initial_setup_str: &str = splitted_input.next().unwrap();
        let instructions_str = splitted_input.next().unwrap();

        let initial_setup = parse_initial_setup(initial_setup_str);
        let instructions = parse_instructions(instructions_str);
        // println!("{:?}", instructions);

    }

    #[warn(dead_code)]
    #[cfg(test)]
    pub fn solve_part_two(input: &str) {
        #[cfg(windows)]
        const LINE_ENDING: &'static str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &'static str = "\n";

    }

    #[warn(dead_code)]
    #[cfg(test)]
    fn parse_initial_setup(initial_setup : &str) -> Vec<Vec<char>> {
        let mut setup: Vec<Vec<char>>  =  initial_setup.split(LINE_ENDING).into_iter()
        .map(|line| {
            let replaced = line
                .replace("[", "")
                .replace("]", "")
                .replace("    ", "_");
            let trimmed = replaced
                .trim_end()
                .replace(" ", "");
            println!("{}", trimmed);
            trimmed.chars().collect()
        }).collect();

        setup
    }

    #[warn(dead_code)]
    #[cfg(test)]
    fn parse_instructions(instructions : &str) -> Vec<Instructions> {
        
        instructions.split(LINE_ENDING).into_iter()
        .map(|line| {

            let parsed_line = line
                .replace("move ", "")
                .replace(" from ", "-")
                .replace(" to ", "-");
            let mut splitted_line = parsed_line.split("-");

            Instructions{
                count: splitted_line.next().unwrap().parse::<usize>().unwrap(),
                start_stack: splitted_line.next().unwrap().parse::<usize>().unwrap(),
                end_stack: splitted_line.next().unwrap().parse::<usize>().unwrap()
            }
        })
        .collect()
    }


    

}

#[cfg(test)]
mod tests {
    use crate::{utils, day5::day5::{solve_part_one, solve_part_two}};

    #[test]
    fn test_day5() {
        let input = utils::get_input_of_the_day("day5\\input.txt");
        solve_part_one(input.as_str());
        solve_part_two(input.as_str());
    }

}