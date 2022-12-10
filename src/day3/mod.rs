pub mod day3 {



    #[warn(dead_code)]
    #[cfg(test)]
    pub fn solve(input: &str) {


        #[cfg(windows)]
        const LINE_ENDING: &'static str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &'static str = "\n";

        let chars = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";


        struct Rucksack {
            first_content : String,
            second_content : String,
            priority: i32 
        }

        let mut rucksacks : Vec<Rucksack> = Vec::new();

        for (i, rucksack_content) in input.split(format!("{}", LINE_ENDING).as_str()).into_iter().enumerate() {
            let half_index : usize = rucksack_content.len()/2;
            let first_compartment_content = &rucksack_content[..half_index];
            let second_compartment_content = &rucksack_content[half_index..];
            
            assert_eq!(first_compartment_content.len(), first_compartment_content.len(), "The length of the two compartments no.{} are not the same: {} <> {}", i, first_compartment_content.len(), second_compartment_content.len());

            let priority_letter = get_same_letter(first_compartment_content, second_compartment_content);

            let p = chars.chars().position(|c| c == priority_letter).unwrap() as i32;

            let rucksack = Rucksack {
                first_content : first_compartment_content.to_string(),
                second_content : second_compartment_content.to_string(),
                priority : p
            };

            rucksacks.push(rucksack);

        }

        let sum_of_prioiities = rucksacks.iter().fold(0, |acc, rucksack| acc + rucksack.priority);
        println!("The total of priorities is : {}", sum_of_prioiities);

    }

    #[cfg(test)]
    fn get_same_letter(a : &str, b : &str) -> char {
        for letter_a in a.chars().into_iter(){
            for letter_b in b.chars().into_iter() {
                if letter_a == letter_b {return letter_a};
            }
        }
        return '_';
    }

}

#[cfg(test)]
mod tests {
    use crate::{utils, day3::day3::solve};
    #[test]
    fn test_day2() {
        let input = utils::get_input_of_the_day("day3\\input.txt");
        solve(input.as_str());
    }

}