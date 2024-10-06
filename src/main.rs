use rand::Rng;

fn main() {
    let row = 10;
    let column = 10;
    
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; column]; row];
    let mut rng = rand::thread_rng();

    
    for i in 0..row {
        for j in 0..column {
            matrix[i][j] = rng.gen_range(1..=15);
        }
    }
    
    for i in 0..row {
        for j in 0..column {
            print!("{} ", matrix[i][j])
        }
        println!("")
    }
    
    let mut multiplication = 1;
    for i in 0..row {
        multiplication *= matrix[i][i];
    }
    
    println!("{:?}", multiplication);
    
    
    let mut submult = 1;
    for i in 0..row {
        submult *= matrix[i][row - i - 1];
    }
    println!("{:?}", submult);
    
    for i in 0..row {
        for j in 0..column {
            if j > row - i - 1 {
                matrix[i][j] = 0;
            }
        }
    }
    
    for i in 0..row {
        for j in 0..column {
            print!("{} ", matrix[i][j])
        }
        println!("")
    }
    
}


