# Enunciados de ejercicios

## 1. Hello Struct: saludo personalizado

Objetivo: Crear una estructura User con nombre y edad, y mostrar un saludo.

Aprendes:

- [x] Structs
- [x] impl y métodos asociados
- [x] Manejo básico de println! y tipos
- Bonus: lectura desde consola
- [01_hello_world](./_01_hello_world)

## 2. Conversión de temperatura (Celsius <-> Fahrenheit)

Objetivo: Recibir un valor y convertirlo entre unidades.

Aprendes:

- [x] Funciones puras
- [x] Tipado estricto
- [x] Pattern matching con match

## 3. Contador mutable con loop

Objetivo: Contador que incrementa hasta 10 y se imprime.

Aprendes:

- [x] Mutabilidad (mut)
- [x] for, while, for in
- Control de flujo (break, continue)

## 4. FizzBuzz

Objetivo: Imprimir los números del 1 al 100, con reglas FizzBuzz.

Aprendes:

- [x] Condicionales (if, else if)
- [x] Rango de números (1..=100)
- [x] Formateo de cadenas

## 5. Manejo de errores con Result

Objetivo: Leer un archivo o retornar un error si no existe.

Aprendes:

- Result<T, E>
- ? operator
- std::fs y std::io

## 6. Mini CLI Tool con std::env

Objetivo: Crear un CLI que recibe nombre de archivo como argumento.

Aprendes:

- std::env::args()
- Parseo de argumentos
- Control básico de entrada

## 7. Juego de adivinar número

Objetivo: Juego clásico que recibe input del usuario.

Aprendes:

- Input del usuario (stdin)
- match y parse::<T>()
- rand crate (si quieres hacerlo más divertido)

## 8. Vector dinámico + iteradores

Objetivo: Pedir 5 números al usuario, guardarlos en un vector, luego calcular promedio.

Aprendes:

- Vec<T>
- .iter(), .sum(), .len()
- Ownership y borrowing (&vec vs vec)

## 9. Uso básico de Option

Objetivo: Función que retorna Some(x) si el número es par, None si no.

Aprendes:

- Tipos algebraicos (Option)
- map, unwrap_or, match con Option

## 10. Mini módulo con funciones separadas

Objetivo: Separar funciones en archivos y módulos.

Aprendes:

- Módulos (mod)
- Uso de pub y privacidad
- Estructura de proyectos (lib.rs, main.rs)
