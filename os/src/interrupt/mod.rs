mod handler;
mod context;

pub fn init(){
    handler::init();
    println!("Mod interrupt initialized!");
}
