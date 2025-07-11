use raylib::prelude::*;
use raylib::color::Color;

// mod utils;
// use utils::{plot, draw_line, draw_polygon, fill_polygon_scanline};
use crate::utils::{plot, draw_line, draw_polygon, fill_polygon_scanline};

pub fn poligono3() {
    let screen_width = 500;
    let screen_height = 500;

    let poligono3 = vec![
        (377, 249), (411, 197), (436, 249),
    ];

    let (mut rl, _thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Exportar Imagen")
        .build();

    // Crear imagen en blanco
    let mut image = Image::gen_image_color(screen_width, screen_height, Color::RAYWHITE);

    // Rellenar y luego dibujar borde
    fill_polygon_scanline(&mut image, &poligono3);

    draw_polygon(&mut image, &poligono3);

    let output_file_name = "poligono3.png";

    image.export_image(output_file_name);

    println!("Imagen exportada exitosamente como '{}'", output_file_name);
}

