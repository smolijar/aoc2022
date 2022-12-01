mod days;
use days::*;
mod inputs;



fn main() {
    println!("result={}", day1_calory_counting::calory_counting(&inputs::task_input(1)));
}
