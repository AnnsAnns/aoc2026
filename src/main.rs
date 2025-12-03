mod day_one;
mod utils;

fn main() {
    let day_one_input = utils::file_to_string("inputs/day_one.txt");
    let mut day_one_solver = day_one::DayOne::new(&day_one_input);
    day_one_solver.calculate_position();
    println!("Day One: {:?}", day_one_solver.get_number_of_zeroes());
}
