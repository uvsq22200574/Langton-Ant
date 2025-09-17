use crate::gamemodes::langton::*;
use crate::objects::functions::*;

use macroquad::prelude::*;
use thousands::Separable;

pub struct CCamera {
    x: f32,
    y: f32,
    cell_size: usize,
    zoom: f32,
    speed: f32,
}

impl CCamera {
    pub fn new() -> Self {
        CCamera {
            x: 0.0,
            y: 0.0,
            cell_size: 10,
            zoom: 3.0,
            speed: 5.0,
        }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn get_cell_size(&self) -> usize {
        self.cell_size
    }

    pub fn get_scaled_cell_size(&self) -> f32 {
        self.cell_size as f32 * self.zoom
    }

    /// Sets a shift in the both axis of the Camera, based on the zoom
    pub fn move_camera(&mut self, direction_x: f32, direction_y: f32) {
        // Calculate the adjusted speed based on zoom
        let adjusted_speed = self.speed.powf(2.0) / self.zoom;

        // Update the camera position based on the direction and adjusted speed
        self.x += direction_x * adjusted_speed;
        self.y += direction_y * adjusted_speed;
    }

    pub fn set_zoom(&mut self, value: f32, cursor_pos: (f32, f32)) {
        // Calculate the world position of the cursor before zooming
        let (cursor_x, cursor_y) = cursor_pos;
        let world_x_before = self.x + cursor_x / self.zoom;
        let world_y_before = self.y + cursor_y / self.zoom;

        // Apply the zoom change
        self.zoom *= value;

        // Clamp the zoom level to the desired range
        if self.zoom > 20.0 {
            self.zoom = 20.0;
        }
        if self.zoom < 0.2 {
            self.zoom = 0.2;
        }

        // Calculate the new camera position to keep the cursor in the same world position
        self.x = world_x_before - cursor_x / self.zoom;
        self.y = world_y_before - cursor_y / self.zoom;
    }

    pub fn screen_to_grid_position(&self, coordinates: (f32, f32), cell_size: usize) -> (i32, i32) {
        let (x, y) = coordinates;

        // Convert screen coordinates to world coordinates
        let world_x = (x) / self.zoom + self.x;
        let world_y = (y) / self.zoom + self.y;

        // Convert world coordinates to grid coordinates
        let grid_x = (world_x / cell_size as f32).floor() as i32;
        let grid_y = (world_y / cell_size as f32).floor() as i32;

        (grid_x, grid_y)
    }

    pub fn grid_to_screen_position(
        &self,
        grid_coordinates: (i32, i32),
        cell_size: usize,
    ) -> (f32, f32) {
        let (grid_x, grid_y) = grid_coordinates;

        // Convert grid position to world position
        let world_x = grid_x as f32 * cell_size as f32;
        let world_y = grid_y as f32 * cell_size as f32;

        // Apply camera offset and zoom
        let screen_x = (world_x - self.x) * self.zoom;
        let screen_y = (world_y - self.y) * self.zoom;

        (screen_x, screen_y)
    }

    pub fn get_visible_range(&self, cell_size: f32) -> (i32, i32, i32, i32) {
        let start_x = (self.x / (cell_size as f32)).floor() as i32;
        let start_y = (self.y / (cell_size as f32)).floor() as i32;
        let end_x = ((self.x + screen_width() / self.zoom) / (cell_size as f32)).ceil() as i32;
        let end_y = ((self.y + screen_height() / self.zoom) / (cell_size as f32)).ceil() as i32;
        (start_x, start_y, end_x, end_y)
    }
}

pub struct LangtonRenderer {}

impl LangtonRenderer {
    // Constructor for LangtonRenderer
    pub fn new() -> Self {
        Self {}
    }

    /// Draws graphical elements
    pub fn render(&mut self, camera: &CCamera, gamestate: &Gamestate) {
        let zoom = camera.get_zoom();
        let cell_size = camera.get_cell_size();
        let scaled_cell_size = camera.get_scaled_cell_size();
        let (start_x, start_y, end_x, end_y) = camera.get_visible_range(cell_size as f32);

        // Draw the grid
        draw_cell_grid(camera, start_x, start_y, end_x, end_y);

        // Draw cells
        for ((x, y), state) in gamestate
            .get_grid()
            .iter()
            .filter(|((x, y), _)| *x >= start_x && *x <= end_x && *y >= start_y && *y <= end_y)
            .map(|((x, y), state)| ((*x, *y), *state))
        {
            let screen_x = (x as f32 * cell_size as f32 - camera.get_x()) * zoom;
            let screen_y = (y as f32 * cell_size as f32 - camera.get_y()) * zoom;

            draw_rectangle(
                screen_x,
                screen_y,
                scaled_cell_size,
                scaled_cell_size,
                *gamestate.get_rule().get_rule_color(state as usize),
            );
        }

        // Draw ants in visible region
        for ant in &gamestate.get_ants_in_region(start_x, end_x, start_y, end_y) {
            let screen_x = (ant.x as f32 * cell_size as f32 - camera.get_x()) * zoom
                + cell_size as f32 * zoom / 2.0;
            let screen_y = (ant.y as f32 * cell_size as f32 - camera.get_y()) * zoom
                + cell_size as f32 * zoom / 2.0;

            let ant_color = match ant.direction {
                Direction::Up => RED,
                Direction::Right => GREEN,
                Direction::Down => BLUE,
                Direction::Left => YELLOW,
            };

            let ant_size = camera.get_scaled_cell_size() / 2.0;

            let (p1_x, p1_y, p2_x, p2_y, p3_x, p3_y) = match ant.direction {
                Direction::Up => (
                    screen_x,
                    screen_y - ant_size,
                    screen_x - ant_size,
                    screen_y + ant_size,
                    screen_x + ant_size,
                    screen_y + ant_size,
                ),
                Direction::Right => (
                    screen_x + ant_size,
                    screen_y,
                    screen_x - ant_size,
                    screen_y - ant_size,
                    screen_x - ant_size,
                    screen_y + ant_size,
                ),
                Direction::Down => (
                    screen_x,
                    screen_y + ant_size,
                    screen_x - ant_size,
                    screen_y - ant_size,
                    screen_x + ant_size,
                    screen_y - ant_size,
                ),
                Direction::Left => (
                    screen_x - ant_size,
                    screen_y,
                    screen_x + ant_size,
                    screen_y - ant_size,
                    screen_x + ant_size,
                    screen_y + ant_size,
                ),
            };

            draw_triangle(
                Vec2::new(p1_x, p1_y),
                Vec2::new(p2_x, p2_y),
                Vec2::new(p3_x, p3_y),
                ant_color,
            );
        }

        // Draw cursor
        draw_cursor(&camera, gamestate.get_cursor_x(), gamestate.get_cursor_y());
    }

    pub fn draw_texts(&self, camera: &CCamera, gamestate: &Gamestate) {
        // Camera
        let camera_text = &format!("Zoom {:.2}x", camera.get_zoom());
        draw_text(
            camera_text,
            screen_width() - measure_text(camera_text, None, 45, 1.0).width,
            42.0,
            45.0,
            DARKPURPLE,
        );
        // Cells
        let cell_text = &format!(
            "Cells:{}",
            gamestate.get_grid().len().separate_with_spaces()
        );
        draw_text(
            cell_text,
            screen_width() - measure_text(cell_text, None, 45, 1.0).width,
            100.0,
            45.0,
            DARKPURPLE,
        );
        // Ants
        let ant_text = &format!(
            "Ants:{}/{}",
            gamestate
                .get_total_visible_ants(camera.get_visible_range(camera.get_cell_size() as f32))
                .separate_with_spaces(),
            gamestate.get_total_ants().separate_with_spaces()
        );
        draw_text(
            ant_text,
            screen_width() - measure_text(ant_text, None, 45, 1.0).width,
            150.0,
            45.0,
            DARKPURPLE,
        );
        // Mouse
        draw_text(
            &format!(
                "{:?}/{:?}",
                camera.screen_to_grid_position(mouse_position(), camera.get_cell_size()),
                gamestate.get_cursor_dimensions()
            ),
            mouse_position().0,
            mouse_position().1,
            24.0,
            DARKPURPLE,
        );
        // FPS
        draw_text(&format!("{}", get_fps()), 10.0, 42.0, 42.0, RED);
        // Iteration
        let iteration_text = &format!(
            "Iter:{} at {} ({}/s)",
            gamestate.get_iteration().separate_with_spaces(),
            gamestate.get_speed().separate_with_spaces(),
            gamestate.get_update_speed().separate_with_spaces()
        );
        draw_text(
            iteration_text,
            (screen_width() - measure_text(&iteration_text, None, 42, 1.0).width) / 2.0,
            42.0,
            42.0,
            RED,
        );
    }
}
