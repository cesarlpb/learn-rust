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

[Instrucciones de instalación](https://www.rust-lang.org/tools/install)

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
| 3    | 3      | 3       |
