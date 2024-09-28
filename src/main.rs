use std::io;

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("");
    let a = input_a.trim().parse::<f32>();
    
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("");
    let b = input_b.trim().parse::<f32>();
    
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).expect("");
    let c = input_c.trim().parse::<f32>();
    
    solver(a.expect("msg"), b.expect("msg"), c.expect("msg"));
 
}

fn solver(a: f32, b: f32, c: f32) {
    let discriminant: f32 = b*b - 4.0 * a * c;
    println!("{}", discriminant);
        
    if discriminant < 0.0 {
        println!("no solution")
    }
        
    match discriminant {
        d if d > 0.0 => {
            let t1 = (-b + d.sqrt()) / (2.0*a);
            let t2 = (-b - d.sqrt()) / (2.0*a); 
                
            let x1: f32 = t1.sqrt();
            let x2: f32 = t2.sqrt(); 
                
            println!("{}, {}, {}, {}", -x1, -x2, x1, x2)
        }
        0.0 => {
            let x = b/(2.0*a);
            println!("{}, {}", -x, x);
        }
        _ => println!("no solution"),
    }
}


