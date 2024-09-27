use std::io;
use rand::Rng;


fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("длину");
    let arr: usize = input.trim().parse().expect("введи число пж");
    
    let mut arr: Vec<i32> = vec![0 ; arr];
    
    let mut rng = rand::thread_rng();
    for i in 0..arr.len() {
        arr[i] = rng.gen_range(0..=15)
    }
    
    println!("Сам список: \n{:?}\n", arr);
    
    bubsort(arr);
}

fn bubsort(mut array: Vec<i32>) {
    for i in 0..array.len() {
        for j in 0..array.len()-1-i {
            if array[j] > array[j + 1] {
                let temp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = temp;
            }
        }
    }
    
    println!("Отсортированный список: \n{:?}", array);
}