fn main() {
    let number = [0, 0, 0, 0];
    let mut name = ['a', 'a', 'a', 'a'];
    println!(
        "numbers: {},{},{},{}",
        number[0], number[1], number[2], number[3]
    );
    println!("name each: {},{},{},{}", name[0], name[1], name[2], name[3]);
    name[0] = 'z';
    name[1] = 'e';
    name[2] = 'd';
    name[3] = '\0';
    println!("name each: {},{},{},{}", name[0], name[1], name[2], name[3]);
    println!("name each: {:#?}", name);
}
