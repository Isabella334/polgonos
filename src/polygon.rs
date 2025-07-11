use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

/// Dibuja un polígono conectando todos sus puntos
pub fn draw_polygon_outline(framebuffer: &mut Framebuffer, vertices: &[Vector2]) {
    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = vertices[(i + 1) % vertices.len()];
        line(framebuffer, start, end);
    }
}

/// Dibuja un polígono escalándolo hacia adentro paso a paso para rellenar.
pub fn fill_polygon_scaled(framebuffer: &mut Framebuffer, vertices: &[Vector2], steps: usize) {
    let mut current_vertices = vertices.to_vec();
    for _ in 0..steps {
        draw_polygon_outline(framebuffer, &current_vertices);
        current_vertices = shrink_polygon(&current_vertices, 0.9999);
    }
}

/// Calcula una versión más pequeña del polígono acercando cada punto al centroide.
fn shrink_polygon(vertices: &[Vector2], factor: f32) -> Vec<Vector2> {
    let n = vertices.len() as f32;
    let centroid = vertices.iter().fold(Vector2::zero(), |acc, v| acc + *v) / n;

    vertices
        .iter()
        .map(|v| {
            let direction = *v - centroid;
            centroid + direction * factor
        })
        .collect()
}
