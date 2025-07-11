use raylib::prelude::*;
use raylib::color::Color;
use crate::utils::{draw_polygon, fill_polygon_scanline, fill_polygon_scanline_with_color};

pub fn poligono4() {
    let screen_width = 800;
    let screen_height = 600;

    let poligono4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180)
    ];

    let poligono5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    let (_rl, _thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Polígono con agujero")
        .build();

    let mut image = Image::gen_image_color(screen_width, screen_height, Color::RAYWHITE);

    // Rellenar el polígono exterior (Polígono 4)
    fill_polygon_scanline(&mut image, &poligono4);

    // Rellenar el agujero con el color de fondo (Polígono 5)
    fill_polygon_scanline_with_color(&mut image, &poligono5, Color::RAYWHITE);

    // Dibujar bordes
    draw_polygon(&mut image, &poligono4);
    draw_polygon(&mut image, &poligono5);

    let output_file_name = "poligono4_con_agujero.png";
    image.export_image(output_file_name);

    println!("Imagen exportada como '{}'", output_file_name);
}
