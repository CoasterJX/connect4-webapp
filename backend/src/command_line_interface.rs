const LINE: &str = "-----------------------------------------------------";

fn log(str: &str) {
    println!("| {:?}", str);
}

pub fn welcome() {
    println!("{}", LINE);
    log("ECE 421 Project 3 - Connect4 with TOOT and OTTO");
    log("Jianxi Wang, Yihe Wang, John Yu");
    println!("{}", LINE);
}