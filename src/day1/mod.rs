pub mod day1 {

    #[cfg(windows)]
    const LINE_ENDING: &'static str = "\r\n";
    #[cfg(not(windows))]
    const LINE_ENDING: &'static str = "\n";

    #[warn(dead_code)]
    #[cfg(test)]
    pub fn solve(input: &str) {
        let mut elf_index = 0;
        let mut max_elf_index = 0;
        let mut max_elf_calories = 0;
        for input_for_elf in input.split(format!("{}{}", LINE_ENDING, LINE_ENDING).as_str()).into_iter() {
            let mut total_elf_calories : i32 = 0;
            for calories_of_elf in input_for_elf.split(format!("{}", LINE_ENDING).as_str()).into_iter() {                
                let calories = calories_of_elf.parse::<i32>().unwrap();
                total_elf_calories += calories;
            }
            // println!("Elf {} carry {} calories", elf_index, total_elf_calories);

            if total_elf_calories > max_elf_calories {
                max_elf_calories = total_elf_calories;
                max_elf_index = elf_index;
            }

            elf_index += 1;
         }
         println!("PART ONE :  elf carrying the most calories is Elf no.{}({} calories)", max_elf_index, max_elf_calories);
    }

}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;
    //D:\Dev\Rust\AOC2022\src\day1\input.txt
    //D:\Dev\Rust\AOC2022\src\day1\input.txt
    use crate::{utils, day1::day1::solve};
    #[test]
    fn test_day1() {
        let input = utils::get_input_of_the_day("day1\\input.txt");
        solve(input.as_str());
    }

}