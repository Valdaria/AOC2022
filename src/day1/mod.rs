pub mod day1 {



    #[warn(dead_code)]
    #[cfg(test)]
    pub fn solve(input: &str) {

        #[cfg(windows)]
        const LINE_ENDING: &'static str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &'static str = "\n";

        let mut elf_index = 0;
        let mut max_elf_index = 0;
        let mut max_elf_calories = 0;

        let mut top_three_elves_calories : Vec<i32> = Vec::new();

        for input_for_elf in input.split(format!("{}{}", LINE_ENDING, LINE_ENDING).as_str()).into_iter() {
            let mut total_elf_calories : i32 = 0;
            for calories_of_elf in input_for_elf.split(format!("{}", LINE_ENDING).as_str()).into_iter() {                
                let calories = calories_of_elf.parse::<i32>().unwrap();
                total_elf_calories += calories;
            }

            top_three_elves_calories.push(total_elf_calories);

            if total_elf_calories > max_elf_calories {
                max_elf_calories = total_elf_calories;
                max_elf_index = elf_index;
            }

            elf_index += 1;
            }

        top_three_elves_calories.sort_by(|a, b| b.cmp(a)); // Sort descending
        let sum_three_top_calories = &top_three_elves_calories[..=2]
        .iter()
        .fold(0, |acc, calories| acc + calories);


         println!("PART ONE : Elf carrying the most calories is Elf no.{}({} calories)", max_elf_index, max_elf_calories);
         println!("PART TWO : The sum of the three top calories is {}", sum_three_top_calories);
    }

}

#[cfg(test)]
mod tests {
    use crate::{utils, day1::day1::solve};
    #[test]
    fn test_day1() {
        let input = utils::get_input_of_the_day("day1\\input.txt");
        solve(input.as_str());
    }

}