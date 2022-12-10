pub mod day2 {



    #[warn(dead_code)]
    #[cfg(test)]
    pub fn solve(input: &str) {

        #[cfg(windows)]
        const LINE_ENDING: &'static str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &'static str = "\n";

        const ROCK_POINTS: i32 = 1;
        const PAPER_POINTS: i32 = 2;
        const SCISSORS_POINTS: i32 = 3;

        const LOOSE_POINTS: i32 = 0;
        const DRAW_POINTS: i32 = 3;
        const WIN_POINTS: i32 = 6;

        pub enum Possibilities {
            ROCK,
            PAPER,
            SCISSORS
        }

        let mut sum_of_points = 0;

        for plays in input.split(format!("{}", LINE_ENDING).as_str()).into_iter() {
            let mut splitted_plays = plays.split_whitespace();
            let opponent_play = match splitted_plays.next().unwrap() {
                "A" => Possibilities::ROCK,
                "B" => Possibilities::PAPER,
                "C" => Possibilities::SCISSORS,
                _ => panic!("Impossible")
            };
            let our_play = match splitted_plays.next().unwrap() {
                "X" => Possibilities::ROCK,
                "Y" => Possibilities::PAPER,
                "Z" => Possibilities::SCISSORS,
                _ => panic!("Impossible")
            };

            match opponent_play {
                Possibilities::ROCK => sum_of_points += match our_play {
                    Possibilities::ROCK  => ROCK_POINTS + DRAW_POINTS,
                    Possibilities::PAPER  => PAPER_POINTS + WIN_POINTS,
                    Possibilities::SCISSORS  => SCISSORS_POINTS + LOOSE_POINTS
                },
                Possibilities::PAPER =>  sum_of_points += match our_play {
                    Possibilities::ROCK  => ROCK_POINTS + LOOSE_POINTS,
                    Possibilities::PAPER  => PAPER_POINTS + DRAW_POINTS,
                    Possibilities::SCISSORS  => SCISSORS_POINTS + WIN_POINTS
                },
                Possibilities::SCISSORS =>  sum_of_points += match our_play {
                    Possibilities::ROCK  => ROCK_POINTS + WIN_POINTS,
                    Possibilities::PAPER  => PAPER_POINTS + LOOSE_POINTS,
                    Possibilities::SCISSORS  => SCISSORS_POINTS + DRAW_POINTS
                },
            };

        }
        println!("Total points : {}", sum_of_points);
    }

}

#[cfg(test)]
mod tests {
    use crate::{utils, day2::day2::solve};
    #[test]
    fn test_day2() {
        let input = utils::get_input_of_the_day("day2\\input.txt");
        solve(input.as_str());
    }

}