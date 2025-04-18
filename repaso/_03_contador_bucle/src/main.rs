// Contador con bucle for

fn contador_for() {
    let mut msg = String::new();
    for i in 1..11 {
        msg.push_str(&format!(" {}", i));
    }
    println!("for:   {}", msg);
}

fn contador_while() {
    let mut i = 1;
    let mut msg = String::new();
    while i < 11 {
        msg.push_str(&format!(" {}", i));
        i += 1;
    }
    println!("while: {}", msg);
}

fn numeros_for_in() -> String {
    let mut msg = String::new();

    // 1..=10 = rango inclusivo del 1 al 10
    for x in 1..=10 {
        msg.push_str(&format!("{} ", x));
    }

    // Elimina el espacio final y guarda el resultado limpio
    let msg = msg.trim_end().to_string();

    // Imprime en pantalla
    println!("for in: {}", msg);

    msg
}

fn main() {
    contador_for();
    contador_while();
    numeros_for_in();
}
