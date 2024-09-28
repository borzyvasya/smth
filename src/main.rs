use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("msg");
    let n: i32 = input.trim().parse().expect("msg");
    
    for i in 1..=n {
        for j in 1..=i {
            print!("*");
        }
        println!("")
    }
}


