// Conversor de Farenheit a Celsius
use std::io;

struct ConversorFarenheitCelsius {
    farenheit: f32,
    celsius: f32,
}

impl ConversorFarenheitCelsius {
    fn convertir_celsius_farenheit(&mut self) {
        self.farenheit = (self.celsius * 9.0 / 5.0) + 32.0;
    }

    fn convertir_farenheit_celsius(&mut self) {
        self.celsius = (self.farenheit - 32.0) * 5.0 / 9.0;
    }
}

fn main() {
    let mut temperatura = ConversorFarenheitCelsius {
        farenheit: 0.0,
        celsius: 0.0,
    };

    println!("Ingrese la temperatura en grados Celsius (C):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");

    // Convertimos el input string a f32
    temperatura.celsius = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor ingrese un número válido.");
            return;
        }
    };

    temperatura.convertir_celsius_farenheit();
    println!(
        "La temperatura en Farenheit (F) es: {:.2} F",
        temperatura.farenheit
    );

    temperatura.convertir_farenheit_celsius();
    println!(
        "La temperatura en Celsius (C) es: {:.2} C",
        temperatura.celsius
    );
}
