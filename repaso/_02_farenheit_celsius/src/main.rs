// Conversor de Farenheit a Celsius
use std::io;

fn c_to_f(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("¿Convertir de (1) C→F o (2) F→C?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            println!("Ingresa la temperatura en °C:");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let c: f32 = buf.trim().parse().expect("Número inválido");
            println!("{} °C -> {:.2} °F", c, c_to_f(c));
        }
        "2" => {
            println!("Ingresa la temperatura en °F:");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let f: f32 = buf.trim().parse().expect("Número inválido");
            println!("{} °F -> {:.2} °C", f, f_to_c(f));
        }
        _ => println!("Opción no válida."),
    }
}
