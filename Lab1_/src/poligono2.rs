use raylib::prelude::*;
use raylib::color::Color;

// mod utils;
// use utils::{plot, draw_line, draw_polygon, fill_polygon_scanline};

use crate::utils::{plot, draw_line, draw_polygon, fill_polygon_scanline};

pub fn poligono2() {
    let screen_width = 500;
    let screen_height = 500;

    let poligono2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    let (mut rl, _thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Exportar Imagen")
        .build();

    // Crear imagen en blanco
    let mut image = Image::gen_image_color(screen_width, screen_height, Color::RAYWHITE);

    // Rellenar y luego dibujar borde
    fill_polygon_scanline(&mut image, &poligono2);

    draw_polygon(&mut image, &poligono2);

    let output_file_name = "poligono2.png";

    image.export_image(output_file_name);

    println!("Imagen exportada exitosamente como '{}'", output_file_name);
}

