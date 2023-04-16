use bevy::{log, prelude::*};

use crate::resources::Board;

pub fn input_handling(
    windows: Query<&Window>,
    board: Res<Board>,
    buttons: Res<Input<MouseButton>>,
) {
    let window = windows.single();
    let position = window.cursor_position();
    if let Some(pos) = position {
        let tile_coordinates = board.mouse_position(window, pos);
        if let Some(coordinates) = tile_coordinates {
            if buttons.just_pressed(MouseButton::Left) {
                log::info!("Trying to uncover tile on {}", coordinates);
                // TODO: generate an event
            }
            if buttons.just_pressed(MouseButton::Right) {
                log::info!("Trying to mark tile on {}", coordinates);
                // TODO: generate an event
            }
        }
    }
}
