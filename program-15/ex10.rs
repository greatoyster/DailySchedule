use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("{}th arg: {}", i, arg);
    }
    let states = ["C", "O", "W", "T"].iter();
    for i in states {
        println!("{}", i);
    }
}
