# learn-rust 🦀
Repositorio para aprender Rust desde cero.

## ¿Por qué Rust?

Rust es un lenguaje de programación diseñado para brindar un rendimiento excepcional, confiabilidad y productividad. A continuación, se presentan algunas razones por las cuales deberías considerar Rust para tu próximo proyecto:

### Rendimiento
Rust es extremadamente rápido y eficiente en el uso de la memoria. Sin un tiempo de ejecución o recolector de basura, puede impulsar servicios críticos para el rendimiento, ejecutarse en dispositivos integrados y integrarse fácilmente con otros lenguajes.

### Confiabilidad
El rico sistema de tipos y el modelo de propiedad de Rust garantizan la seguridad de la memoria y la seguridad de la concurrencia, lo que te permite eliminar muchas clases de errores en tiempo de compilación.

### Productividad
Rust cuenta con una excelente documentación, un compilador amigable con mensajes de error útiles y herramientas de primera calidad. Incluye un administrador de paquetes integrado, una herramienta de construcción, soporte inteligente para múltiples editores con autocompletado e inspecciones de tipos, un formateador automático y más.

## Referencias

- Sitio web oficial: [Rust](https://www.rust-lang.org/)
    - Learn: [Learn Rust](https://www.rust-lang.org/learn)
    - Libro de Rust: [The Rust Programming Language](https://doc.rust-lang.org/book/)

**Ejemplos:**
- Sigue la estructura del libro con ejemplos: [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## Instalaciones

- Instrucciones en el libro: [https://doc.rust-lang.org/book/ch01-01-installation.html](https://doc.rust-lang.org/book/ch01-01-installation.html)
- Versión corta en web: [Instrucciones de instalación](https://www.rust-lang.org/tools/install)

### Windows

### Mac

```bash
    # Instalamos Rust:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
    # Recargamos el shell:
    source "$HOME/.cargo/env"
```

```bash
    # Probamos la instalación:
    rustc --version
```

### Linux

## Programas

Para compilar un programa:

```bash
    # Compilamos el programa:
    rustc <carpeta>/<programa | main>.rs -o bin/<programa.bin | programa>
```

Para ejecutar un programa:

```bash
    # Ejecutamos el programa:
    ./bin/<programa.bin | programa>
```

| Tema | Enlace | Archivo |
|------|--------|---------|
|01. Hello, world    | https://doc.rust-lang.org/rust-by-example/hello.html      | [01.hello_world/main.rs](01.hello_world/main.rs)       |
| 1.1 Comentarios    | https://doc.rust-lang.org/rust-by-example/hello/comment.html      |   [01.hello_world/comments.rs](01.hello_world/comments.rs)     |
| 1.2 Formatted print    | https://doc.rust-lang.org/rust-by-example/hello/print.html      | [01.hello_world/formatted_print.rs](01.hello_world/formatted_print.rs)       |
| 1.2.0 Ejemplo print_struct     | -      | [01.hello_world/print_struct.rs](01.hello_world/print_struct.rs)       |
| 1.2.1  Debug I  | https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html      | [01.hello_world/debug_1.rs](01.hello_world/debug_1.rs)       |
| 1.2.1  Debug II | https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html      | [01.hello_world/debug_2.rs](01.hello_world/debug_2.rs)       |
| 1.2.1  Debug III | https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html      | [01.hello_world/debug_3.rs](01.hello_world/debug_3.rs)       |
| 1.2.2 Display | https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html | [01.hello_world/display_1.rs](01.hello_world/display_1.rs) |
| 1.2.2 Display | https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html | [01.hello_world/display_2.rs](01.hello_world/display_2.rs) |
| Activity: Display | https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html | [01.hello_world/display_3.rs](01.hello_world/display_3.rs) |
| 1.2.2.1 Testcase: List | https://doc.rust-lang.org/rust-by-example/hello/print/print_display/testcase_list.html | [01.hello_world/testcase_list_1.rs](01.hello_world/testcase_list_1.rs) |
| Activity: Testcase List | https://doc.rust-lang.org/rust-by-example/hello/print/print_display/testcase_list.html | [01.hello_world/testcase_list_2.rs](01.hello_world/testcase_list_2.rs) |
| 1.2.3 Formatting | https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html | [01.hello_world/formatting_1.rs](01.hello_world/formatting_1.rs) |
| Activity: Formatting | https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html | [01.hello_world/formatting_2.rs](01.hello_world/formatting_2.rs) |
| 2 | 2 | 2 |
