use std::io;

fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        println!("got a line: {}", line.unwrap());
    }
}
