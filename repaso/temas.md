# Temas de Repaso de Rust

Tengo poco tiempo así que aquí va un resumen de temas para repasar Rust.

> It's rusting time! 🦀

---

## 🧱 1. Structs y Métodos

- Definición de estructuras con struct.
- Métodos asociados con impl.
- Métodos que toman &self o &mut self.
- Uso de println!, formateo con {} y tipos como String, u32.

## 🔁 2. Funciones y Tipado Estricto

- Declaración de funciones fn nombre() -> tipo {}.
- Tipos primitivos: i32, f64, bool, char, String, etc.
- Conversión de tipos (as, parse::<f64>(), etc.).
- Uso de match para control de flujo.

## 🌀 3. Mutabilidad y Bucles

- Uso de mut para variables mutables.
- Bucles: loop, while, for.
- Palabras clave: break, continue.

## 📏 4. Condicionales y Rangos

- if, else if, else.
- Rangos con 1..=100.
- Módulo % para lógica como FizzBuzz.
- Formateo de strings: println!("Hola {}", nombre).

## 📦 5. Manejo de Errores con Result

- Patrón Result<T, E>.
- ? operator para propagación de errores.
- Lectura de archivos: std::fs::read_to_string(), File::open().
- Manejo de errores con match, unwrap_or, etc.

## 📜 6. CLI y Argumentos

- Acceso a argumentos con std::env::args().
- Parseo de argumentos con .nth(), .collect::<Vec<\_>>().
- Validación básica de entradas desde CLI.

## 🎲 7. Entrada de Usuario e Interacción

- Lectura de input con std::io::stdin().read_line(&mut buffer).
- Conversión con .trim().parse::<T>().
- Uso del crate rand (rand::thread_rng().gen_range()).

## 🧮 8. Vectores y Promedios

- Vec<T>: creación, push, iter, etc.
- .iter(), .sum(), .len() para calcular promedio.
- Ownership: &vec vs vec, clones si es necesario.

## 🤷 9. Option y Match

- Uso de Option<T>: Some(valor) o None.
- Métodos como map, unwrap_or, is_some, is_none.
- Control de flujo con match sobre Option.

## 📁 10. Módulos y Estructura de Proyecto

- Crear módulos con mod.
- Visibilidad con pub y privacidad por defecto.
- Estructura del proyecto: main.rs, lib.rs, submódulos.
- use crate::modulo::funcion.
