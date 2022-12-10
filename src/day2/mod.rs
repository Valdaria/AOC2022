pub mod day2 {



    #[warn(dead_code)]
    #[cfg(test)]
    pub fn solve(input: &str) {
        use std::io;


        #[cfg(windows)]
        const LINE_ENDING: &'static str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &'static str = "\n";


        pub enum Shapes {
            ROCK = 1,
            PAPER = 2,
            SCISSORS = 3
        }

        pub enum EndOfRound {
            LOOSE = 0,
            DRAW = 3,
            WIN = 6
        }

        let mut sum_of_points = 0;

        for plays in input.split(format!("{}", LINE_ENDING).as_str()).into_iter() {
            let mut splitted_plays = plays.split_whitespace();
            let opponent_play = match splitted_plays.next().unwrap() {
                "A" => Shapes::ROCK,
                "B" => Shapes::PAPER,
                "C" => Shapes::SCISSORS,
                _ => panic!("Impossible")
            };
            let our_play = match splitted_plays.next().unwrap() {
                "X" => EndOfRound::LOOSE,
                "Y" => EndOfRound::DRAW,
                "Z" => EndOfRound::WIN,
                _ => panic!("Impossible")
            };

            sum_of_points += match opponent_play {
                Shapes::ROCK => match our_play {
                    EndOfRound::LOOSE => EndOfRound::LOOSE as i32 + Shapes::SCISSORS as i32,
                    EndOfRound::DRAW  => EndOfRound::DRAW  as i32 + Shapes::ROCK     as i32,
                    EndOfRound::WIN   => EndOfRound::WIN   as i32 + Shapes::PAPER    as i32
                },
                Shapes::PAPER =>  match our_play {
                    EndOfRound::LOOSE => EndOfRound::LOOSE as i32 + Shapes::ROCK     as i32,
                    EndOfRound::DRAW  => EndOfRound::DRAW  as i32 + Shapes::PAPER    as i32,
                    EndOfRound::WIN   => EndOfRound::WIN   as i32 + Shapes::SCISSORS as i32
                },
                Shapes::SCISSORS =>  match our_play {
                    EndOfRound::LOOSE => EndOfRound::LOOSE as i32 + Shapes::PAPER    as i32,
                    EndOfRound::DRAW  => EndOfRound::DRAW  as i32 + Shapes::SCISSORS as i32,
                    EndOfRound::WIN   => EndOfRound::WIN   as i32 + Shapes::ROCK     as i32
                },
            };

        }
        println!("Total points part 2: {}", sum_of_points);
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