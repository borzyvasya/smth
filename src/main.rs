fn main() {
    let p0: f32 = 1.25;
    let z: f32 = 1.21*10.0f32.powf(-4.0f32);
    
    println!("Высота (м)\tПлотность (кг/м^3)");
    for h in (0..1000).step_by(100) {
        let p = p0 * (-h as f32 * z).exp();
        println!("{}\t\t{}", h, p)
    }
}


