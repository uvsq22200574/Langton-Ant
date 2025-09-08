use macroquad::prelude::*;
use crate::gamemodes::langton::Gamestate;
use crate::objects::structures::CCamera;

pub fn handle_input(
    camera: &mut CCamera,
    gamestate: &mut Gamestate,
) {

    // Handle zoom with mouse wheel
    camera.set_zoom(1.0 + mouse_wheel().1 * 0.1, mouse_position());

    // Camera movement
    if is_key_down(KeyCode::Z) {
        camera.move_camera(0.0, -1.0);
    }
    if is_key_down(KeyCode::S) {
        camera.move_camera(0.0, 1.0);
    }
    if is_key_down(KeyCode::Q) {
        camera.move_camera(-1.0, 0.0);
    }
    if is_key_down(KeyCode::D) {
        camera.move_camera(1.0, 0.0);
    }


    if is_key_pressed(KeyCode::Up) && gamestate.get_cursor_y() < 10 {
        gamestate.increment_cursor_y(1);
    }
    if is_key_pressed(KeyCode::Down) && gamestate.get_cursor_y() > 1 {
        gamestate.decrement_cursor_y(1);
    }
    if is_key_pressed(KeyCode::Left) && gamestate.get_cursor_x() > 1 {
        gamestate.decrement_cursor_x(1);
    }
    if is_key_pressed(KeyCode::Right) && gamestate.get_cursor_x() < 10 {
        gamestate.increment_cursor_x(1);
    }


    // Gameplay
    // 0->UP 1->RIGHT 2->DOWN 3->LEFT
    if is_mouse_button_pressed(MouseButton::Left) || (is_key_down(KeyCode::LeftShift)) && (is_mouse_button_down(MouseButton::Left)) {
        gamestate.add_ants(camera.screen_to_grid_position(mouse_position(), camera.get_cell_size()));
    }

    if is_key_pressed(KeyCode::R) {
        gamestate.clear_ants();
    }

    if is_key_pressed(KeyCode::T) {
        gamestate.clear_grid();
    }

    if is_key_pressed(KeyCode::Space) {
        gamestate.invert_pause_state();
    }

    if is_key_pressed(KeyCode::F) {
        gamestate.set_pause_state(true);
    }

    if is_key_pressed(KeyCode::F1) {gamestate.select_rule(0);gamestate.reset();}
    if is_key_pressed(KeyCode::F2) {gamestate.select_rule(1);gamestate.reset();}
    if is_key_pressed(KeyCode::F3) {gamestate.select_rule(2);gamestate.reset();}
    if is_key_pressed(KeyCode::F4) {gamestate.select_rule(3);gamestate.reset();}
    if is_key_pressed(KeyCode::F5) {gamestate.select_rule(4);gamestate.reset();}
    if is_key_pressed(KeyCode::F6) {gamestate.select_rule(5);gamestate.reset();}
    if is_key_pressed(KeyCode::F7) {gamestate.select_rule(6);gamestate.reset();}
    if is_key_pressed(KeyCode::F8) {gamestate.select_rule(7);gamestate.reset();}
    if is_key_pressed(KeyCode::F9) {gamestate.select_rule(8);gamestate.reset();}
    if is_key_pressed(KeyCode::F10) {gamestate.select_rule(9);gamestate.reset();}
    if is_key_pressed(KeyCode::F11) {gamestate.select_rule(10);gamestate.reset();}
    if is_key_pressed(KeyCode::F12) {gamestate.select_rule(11);gamestate.reset();}

    if is_key_pressed(KeyCode::J) {gamestate.set_speed((gamestate.get_speed() as f32 * 2.0) as u32);}
    if is_key_pressed(KeyCode::K) {gamestate.set_speed(1);}
}