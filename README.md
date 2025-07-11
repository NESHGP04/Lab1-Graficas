# Laboratorio 1 - Gráficas 🖼️ - Polígonos con Raylib + Rust

Este proyecto fue desarrollado como parte del curso de **Gráficas por Computadora**, y tiene como objetivo implementar algoritmos básicos de rasterización y relleno de polígonos utilizando **Rust** y **Raylib**.

---

## ✨ Características

- Dibujo de líneas utilizando el algoritmo de Bresenham.
- Construcción de polígonos con vértices definidos.
- Relleno de polígonos usando el algoritmo **Scanline Fill**.
- Soporte para **agujeros internos** (polígonos anidados).
- Exportación del resultado a imágenes `.png`.

---

## 📦 Requisitos

- Rust (estable): [https://rust-lang.org](https://rust-lang.org)
- Raylib para Rust:
  ```bash
  cargo add raylib
  brew install raylib
  ```
  
  ## 🚀  Ejecución
    ```bash
  cargo run
  ```
- Esto generará cada una de las imágenes de todos los polígonos!

## 🗂️ Estructura
.
├── src/
│   ├── main.rs          # Llama a todos los polígonos
│   ├── poligono1.rs     # Define y dibuja el polígono 1
│   ├── poligono2.rs     # ...
│   ├── utils.rs         # Funciones comunes (plot, draw_line, fill, etc.)
│   └── ...
├── Cargo.toml
├── .gitignore
└── README.md

## 🧠 Notas

- Los polígonos se exportan en orientación correcta (eje Y invertido para PNG).
- El polígono 5 se dibuja como agujero dentro del polígono 4 y no es rellenado.

## 👩‍💻 Autor
Marines Garcia Paredes
GitHub


