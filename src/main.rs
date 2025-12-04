use day_four::DayFour;
use day_three::DayThree;

mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod utils;

fn main() {
    // let day_one_input = include_str!("../inputs/day_one.txt");
    // let mut day_one_solver = day_one::DayOne::new(&day_one_input);
    // day_one_solver.calculate_position();
    // println!("Day One: {:?}", day_one_solver.get_number_of_zeroes());

    // let day_two_input = include_str!("../inputs/day_two.txt");
    // let mut day_two_solver = day_two::DayTwo::new(&day_two_input);
    // println!("Day Two: {:?}", day_two_solver);
    // day_two_solver.find_wrong_ids();
    // day_two_solver.print_wrong_ids();
    // println!("Day Two Answer: {:?}", day_two_solver.get_total_wrong_ids());
    //

    // let mut day_four = DayFour::new(include_str!("../inputs/day_four.txt"));
    // day_four.repeat_until_no_changes();
    // println!("Day Four Answer: {:?}", day_four.get_total());

    let mut day_three = DayThree::new(include_str!("../inputs/day_three.txt"));
    day_three.process_input(12);
    println!("Day Three Answer Part 1: {:?}", day_three.get_total());
}
