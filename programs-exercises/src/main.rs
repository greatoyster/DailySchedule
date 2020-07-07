use std::io::Write;

#[allow(dead_code)]
fn create_file_structure() -> Result<(), std::io::Error> {
    for i in 0..15 {
        let path = format!("Program-{}", i);
        let mut f = std::fs::File::create(path)?;
        f.write(format!("//This is the program-{}\n", i).as_bytes())?;
        f.write(String::from("//Input:\n").as_bytes())?;
        f.write(String::from("//Output:\n").as_bytes())?;
        f.write(String::from("//Description:\n").as_bytes())?;
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    // create_file_structure()?;
    Ok(())
}
