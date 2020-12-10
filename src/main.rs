mod day01;

fn main() {
    println!("Advent of Code 2020");
    println!();

    println!("-= Day 01 =-");
    let day01_input = "data\\input_01A.txt";

    match day01::solution::get_multiple_of_two_items_which_sum_2020(day01_input) {
        Ok(result) => {
            println!("Result of part A: {:?}", result);
            0
        },
        Err(error) => {
            println!("Problem opening the file {:?}: {:?}", day01_input, error);
            1
        }
    };
}
