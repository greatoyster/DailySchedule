use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("{}th arg: {}", i, arg);
    }
    let states = ["C", "O", "W", "T"];
    for i in states.iter() {
        println!("{}", i);
    }
    let num_states: usize = 4;
    let mut i: usize = 0;

    while i < num_states {
        println!("{}", states[i]);
        i += 1;
    }
}
