use std::env;

mod day_eight;
mod day_eleven;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_three;
mod day_two;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day_number>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1];

    match day.as_str() {
        "1" => {
            let day_one_input = utils::file_to_string("./inputs/day_one.txt");
            let mut day_one_solver = day_one::DayOne::new(&day_one_input);
            day_one_solver.calculate_position();
            println!("Day One: {:?}", day_one_solver.get_number_of_zeroes());
        }
        "2" => {
            let day_two_input = utils::file_to_string("./inputs/day_two.txt");
            let mut day_two_solver = day_two::DayTwo::new(&day_two_input);
            day_two_solver.find_wrong_ids();
            day_two_solver.print_wrong_ids();
            println!("Day Two Answer: {:?}", day_two_solver.get_total_wrong_ids());
        }
        "3" => {
            let day_three_input = utils::file_to_string("./inputs/day_three.txt");
            let mut day_three = day_three::DayThree::new(&day_three_input);
            day_three.process_input(12);
            println!("Day Three Answer Part 1: {:?}", day_three.get_total());
        }
        "4" => {
            let day_four_input = utils::file_to_string("./inputs/day_four.txt");
            let mut day_four = day_four::DayFour::new(&day_four_input);
            day_four.repeat_until_no_changes();
            println!("Day Four Answer: {:?}", day_four.get_total());
        }
        "5" => {
            let day_five_input = utils::file_to_string("./inputs/day_five.txt");
            let mut day_five = day_five::DayFive::new(&day_five_input);
            day_five.calc_total();
            println!("Day Five Answer: {:?}", day_five.get_total_valid());
            day_five.calc_covered_ranges();
            println!(
                "Valid {}",
                day_five.calculate_all_valid_ids_from_covered_range()
            );
        }
        "6" => {
            let day_six_input = utils::file_to_string("./inputs/day_six.txt");
            let day_six = day_six::DaySix::new(&day_six_input);
            day_six.process_grid();
        }
        "7" => {
            let mut day_seven = day_seven::DaySeven::new(day_seven::GRID_DATA);
            day_seven.draw_beams();
            println!(
                "Day Seven Result:\n{}",
                day_seven.tree_to_string(&day_seven.tree)
            );
            println!("Day Seven Splits Done: {}", day_seven.get_splits_done());

            let tree = day_seven.tree_to_binary_tree();

            // Display the binary tree with proper formatting
            if let Some(tree_display) = day_seven.display_binary_tree() {
                println!("\nBinary Tree Visualization:");
                println!("{}", tree_display);
            }

            let paths = day_seven.calculate_all_possible_paths(&tree.unwrap());
            println!("Possible Paths: {:?}", paths);
        }
        "8" => {
            let day_eight_input = utils::file_to_string("./inputs/day_eight.txt");
            let mut day_eight = day_eight::DayEight::new(&day_eight_input);
            day_eight.find_closest_boxes(1000);
            day_eight.print_junctions();
        }
        "9" => {
            let day_nine_input = utils::file_to_string("./inputs/day_nine.txt");
            let day_nine = day_nine::DayNine::new(&day_nine_input);
            println!("Part 1 - {}", day_nine.find_largest_rectangle());
            let timer = utils::Timer::start();
            let part_2_val = day_nine.find_largest_rectangle_inside_polygon();
            timer.finish_and_print("Day 9 Part 2");
            println!(
                "Part 2 - Largest rectangle (only red/green tiles): {}",
                part_2_val
            );
        }
        "11" => {
            let day_eleven_input = utils::file_to_string("./inputs/day_eleven.txt");
            let mut day_eleven = day_eleven::DayEleven::new(&day_eleven_input);
            println!("Result: {}", day_eleven.depth_first_search("you"));
        }
        _ => {
            eprintln!("Invalid day: {}. Please choose 1-12.", day);
            std::process::exit(1);
        }
    }
}
