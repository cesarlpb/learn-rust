// Hello world! program

use std::io;

struct User {
    name: String,
    age: u32,
}

impl User {
    fn saludar(&self) {
        println!("Hola, {}! Tienes {} años", self.name, self.age);
    }
}

fn main() {
    // Hola mundo simple:
    // println!("Hello, world!");
    //

    // Leer nombre
    println!("¿Cuál es tu nombre?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Error al leer el nombre");
    let input_name = name.trim(); // quitar salto de línea

    // Leer edad
    println!("¿Cuántos años tienes?");
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Error al leer la edad");

    // Convertir edad a número
    let age: u32 = match age_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, escribe un número válido.");
            return;
        }
    };
    let user = User {
        name: input_name.to_string(),
        age: age,
    };

    // Llamar a saludar con valores conseguidos desde consola:
    user.saludar();
}
