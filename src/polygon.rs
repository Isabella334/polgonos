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
