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

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let shared_status = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut m = shared_status.lock().unwrap();
            (*m).jobs_completed += 1;
            println!("{}", m.jobs_completed);
        }
    });
    {
        
        while (*status).lock().unwrap().jobs_completed < 10 {
            thread::sleep(Duration::from_millis(500));
        }
    }
}
