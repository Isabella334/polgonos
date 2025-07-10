mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::polygon::draw_polygon_outline;
use crate::polygon::fill_polygon_scaled;


fn main() {
    let width: i32 = 800;
    let height: i32 = 600;
    let mut framebuffer = Framebuffer::new(width as u32, height as u32);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    // POLÍGONO 1
    framebuffer.set_current_color(Color::YELLOW);
    fill_polygon_scaled(&mut framebuffer, &[
        rvec2(165.0, 380.0), rvec2(185.0, 360.0), rvec2(180.0, 330.0), rvec2(207.0, 345.0),
        rvec2(233.0, 330.0), rvec2(230.0, 360.0), rvec2(250.0, 380.0), rvec2(220.0, 385.0),
        rvec2(205.0, 410.0), rvec2(193.0, 383.0)
    ], 350000);

    framebuffer.set_current_color(Color::WHITE);
    draw_polygon_outline(&mut framebuffer, &[
        rvec2(165.0, 380.0), rvec2(185.0, 360.0), rvec2(180.0, 330.0),
        rvec2(207.0, 345.0), rvec2(233.0, 330.0), rvec2(230.0, 360.0),
        rvec2(250.0, 380.0), rvec2(220.0, 385.0), rvec2(205.0, 410.0),
        rvec2(193.0, 383.0)
    ]);


    // POLÍGONO 2
    framebuffer.set_current_color(Color::BLUE);
    fill_polygon_scaled(&mut framebuffer, &[
        rvec2(321.0, 335.0), rvec2(288.0, 286.0), rvec2(339.0, 251.0), rvec2(374.0, 302.0)
    ], 350000);

    // POLÍGONO 3
    framebuffer.set_current_color(Color::RED);
    fill_polygon_scaled(&mut framebuffer, &[
        rvec2(377.0, 249.0), rvec2(411.0, 197.0), rvec2(436.0, 249.0)
    ], 350000);

    // POLÍGONO 4
    framebuffer.set_current_color(Color::GREEN);
    fill_polygon_scaled(&mut framebuffer, &[
        rvec2(413.0, 177.0), rvec2(448.0, 159.0), rvec2(502.0, 88.0), rvec2(553.0, 53.0),
        rvec2(535.0, 36.0), rvec2(676.0, 37.0), rvec2(660.0, 52.0), rvec2(750.0, 145.0),
        rvec2(761.0, 179.0), rvec2(672.0, 192.0), rvec2(659.0, 214.0), rvec2(615.0, 214.0),
        rvec2(632.0, 230.0), rvec2(580.0, 230.0), rvec2(597.0, 215.0), rvec2(552.0, 214.0),
        rvec2(517.0, 144.0), rvec2(466.0, 180.0)
    ], 350000);

    // POLÍGONO 5
    framebuffer.set_current_color(Color::ORANGE);
    fill_polygon_scaled(&mut framebuffer, &[
        rvec2(682.0, 175.0), rvec2(708.0, 120.0), rvec2(735.0, 148.0), rvec2(739.0, 170.0)
    ], 350000);

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Polígonos")
        .build();

    let texture = rl.load_texture_from_image(&thread, &framebuffer.buffer).unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
