pub mod day4 {



    #[warn(dead_code)]
    #[cfg(test)]
    pub fn solve_part_one(input: &str) {


        #[cfg(windows)]
        const LINE_ENDING: &'static str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &'static str = "\n";


        let result = input
        .split(LINE_ENDING)
        .into_iter()
        .filter(|input_range| is_input_range_contains_other(input_range))
        .count();

        println!("The number of range fully containing ither is: {}", result);

    }

    #[warn(dead_code)]
    #[cfg(test)]
    pub fn solve_part_two(input: &str) {
        #[cfg(windows)]
        const LINE_ENDING: &'static str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &'static str = "\n";


        let result = input
        .split(LINE_ENDING)
        .into_iter()
        .filter(|input_range| is_input_range_overlaps_other(input_range))
        .count();

        println!("The number of range overlaps ither is: {}", result);

    }

    #[warn(dead_code)]
    #[cfg(test)]
    fn is_input_range_contains_other(input_range: &str) -> bool {
        let mut splitted = input_range.split(",");
        let left = splitted.next().unwrap();
        let mut splitted_left = left.split("-");
        let left_min = splitted_left.next().unwrap().parse::<i32>().unwrap();
        let left_max = splitted_left.next().unwrap().parse::<i32>().unwrap();
        
        let right = splitted.next().unwrap();
        let mut splitted_right = right.split("-");
        let right_min = splitted_right.next().unwrap().parse::<i32>().unwrap();
        let right_max = splitted_right.next().unwrap().parse::<i32>().unwrap();
        
        (right_min >= left_min && right_max <= left_max) || (left_min >= right_min && left_max <= right_max)

    }

    #[warn(dead_code)]
    #[cfg(test)]
    fn is_input_range_overlaps_other(input_range: &str) -> bool {
        let mut splitted = input_range.split(",");
        let left = splitted.next().unwrap();
        let mut splitted_left = left.split("-");
        let left_min = splitted_left.next().unwrap().parse::<i32>().unwrap();
        let left_max = splitted_left.next().unwrap().parse::<i32>().unwrap();
        
        let right = splitted.next().unwrap();
        let mut splitted_right = right.split("-");
        let right_min = splitted_right.next().unwrap().parse::<i32>().unwrap();
        let right_max = splitted_right.next().unwrap().parse::<i32>().unwrap();
        
        (left_min >= right_min && left_min <= right_max) ||
        (left_max >= right_min && left_max <= right_max) ||
        (right_min >= left_min && right_min <= left_max) || 
        (right_max >= left_min && right_max <= left_max)

    }

}

#[cfg(test)]
mod tests {
    use crate::{utils, day4::day4::{solve_part_one, solve_part_two}};

    #[test]
    fn test_day4() {
        let input = utils::get_input_of_the_day("day4\\input.txt");
        solve_part_one(input.as_str());
        solve_part_two(input.as_str());
    }

}