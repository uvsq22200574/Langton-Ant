use macroquad::prelude::*;

mod objects;
use objects::structures::*;

mod gamemodes;
use gamemodes::langton::Gamestate;

mod input;
use input::functions::handle_input;

#[macroquad::main("Langton's Ant")]
async fn main() {
    // Fullscreen
    macroquad::window::set_fullscreen(true);
    while screen_width() == 800.0 || screen_height() == 600.0 {
        next_frame().await;
    }

    // Structures
    let mut game_data = Gamestate::new();
    let mut camera = CCamera::new();
    let mut render = LangtonRenderer::new();

    loop {
        handle_input(&mut camera, &mut game_data);

        game_data.update(game_data.get_speed());

        clear_background(Color::from_hex(0x666666));

        // Stop rendering for performance gains
        if !is_key_down(KeyCode::Tab) {
            render.render(&camera, &game_data);
        }

        render.draw_texts(&camera, &game_data);

        next_frame().await;
    }
}
