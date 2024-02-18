use std::fmt;

// Implemento el trait `fmt::Display` para la estructura `Structure`.
// `fmt::Display` es un trait que define el comportamiento de la cadena de
// formato `{}`.
// Es decir, permite que la estructura se imprima con el formato `{}`:
//

fn main() {
    struct Structure(i32);
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    let s = Structure(42);
    println!("El valor interno del struct `Structure` s es: {}", s);
}
