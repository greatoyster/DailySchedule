use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let distance: i32 = 100;
    if args.len() <= 1 {
        println!("You are {} miles away.", distance);
    } else {
        println!("You are {} miles away.", args[1]);
    }
}
