use raylib::prelude::*;
use raylib::color::Color;

pub fn plot(image: &mut Image, x: i32, y: i32) {
    let y_corrected = image.height as i32 - 1 - y;
    image.draw_pixel(x, y_corrected, Color::BLACK);
}

pub fn draw_line(image: &mut Image, x0: i32, x1: i32, y0: i32, y1: i32) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        plot(image, x0, y0);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_polygon(image: &mut Image, points: &Vec<(i32, i32)>) {
    let len = points.len();
    for i in 0..len {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % len];
        draw_line(image, x0, x1, y0, y1);
    }
}

pub fn fill_polygon_scanline(image: &mut Image, points: &Vec<(i32, i32)>) {
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;

    // Encuentra los límites verticales del polígono
    for (_, y) in points {
        ymin = ymin.min(*y);
        ymax = ymax.max(*y);
    }

    for y in ymin..=ymax {
        let mut intersecciones = vec![];

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            if (y0 < y && y1 >= y) || (y1 < y && y0 >= y) {
                // Evita divisiones por 0
                if y1 != y0 {
                    let x = x0 + ((y - y0) * (x1 - x0)) / (y1 - y0);
                    intersecciones.push(x);
                }
            }
        }

        intersecciones.sort();

        // Rellena entre pares de intersección
        for i in (0..intersecciones.len()).step_by(2) {
            if i + 1 < intersecciones.len() {
                let x_start = intersecciones[i];
                let x_end = intersecciones[i + 1];
                for x in x_start..=x_end {
                    plot(image, x, y);
                }
            }
        }
    }
}

pub fn fill_polygon_scanline_with_color(image: &mut Image, points: &Vec<(i32, i32)>, color: Color) {
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;

    for &(_, y) in points {
        ymin = ymin.min(y);
        ymax = ymax.max(y);
    }

    for y in ymin..=ymax {
        let mut intersections = vec![];

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            if (y0 < y && y1 >= y) || (y1 < y && y0 >= y) {
                if y1 != y0 {
                    let x = x0 + ((y - y0) * (x1 - x0)) / (y1 - y0);
                    intersections.push(x);
                }
            }
        }

        intersections.sort();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                for x in x_start..=x_end {
                    let y_corrected = image.height as i32 - 1 - y;
                    image.draw_pixel(x, y_corrected, color);
                }
            }
        }
    }
}
