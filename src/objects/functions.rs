use macroquad::prelude::*;

use crate::objects::structures::CCamera;

pub fn draw_cell_grid(camera: &CCamera, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
    // Draw the grid when relevant
    if camera.get_zoom() > 0.5 {
        // Draw vertical lines
        for grid_x in start_x..=end_x {
            let (screen_x, _) = camera.grid_to_screen_position((grid_x, 0), camera.get_cell_size() as usize);
            draw_line(screen_x, 0.0, screen_x, screen_height(), 1.0, GRAY);
        }

        // Draw horizontal lines
        for grid_y in start_y..=end_y {
            let (_, screen_y) = camera.grid_to_screen_position((0, grid_y), camera.get_cell_size() as usize);
            draw_line(0.0, screen_y, screen_width(), screen_y, 1.0, GRAY);
        }
    }
}

pub fn draw_cursor (camera: &CCamera, x: u8, y: u8) {
    let (mouse_x, mouse_y) = camera.grid_to_screen_position(camera.screen_to_grid_position(mouse_position(), camera.get_cell_size()), camera.get_cell_size());
    draw_rectangle_lines(mouse_x as f32, mouse_y as f32, x as f32 * camera.get_scaled_cell_size(), y as f32 * camera.get_scaled_cell_size(), 6.0, RED);
}