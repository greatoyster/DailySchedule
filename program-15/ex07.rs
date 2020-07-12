fn main() {
    let bugs = 100;
    let bug_rate: f64 = 1.2;
    println!("you have {} bugs at imaginary rate of {}", bugs, bug_rate);
    let universe_of_defects: i64 = 1 * 1024 * 1024 * 1024;
    println!("the entire universe has {} bugs", universe_of_defects);
    let expected_bugs: f64 = bugs as f64 * bug_rate;
    println!("you are expected to have {} bugs", expected_bugs);
    let part_of_universe: f64 = expected_bugs / universe_of_defects as f64;
    println!("than is only a {:e}% portion of universe", part_of_universe);
}
