use bevy::{log, prelude::*};

use crate::{
    events::{TileMarkEvent, TileTriggerEvent},
    resources::Board,
};

pub fn input_handling(
    windows: Query<&Window>,
    board: Res<Board>,
    buttons: Res<Input<MouseButton>>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
    mut tile_mark_ewr: EventWriter<TileMarkEvent>,
) {
    let window = windows.single();
    let position = window.cursor_position();
    if let Some(pos) = position {
        let tile_coordinates = board.mouse_position(window, pos);
        if let Some(coordinates) = tile_coordinates {
            if buttons.just_pressed(MouseButton::Left) {
                log::info!("Trying to uncover tile on {}", coordinates);
                tile_trigger_ewr.send(TileTriggerEvent(coordinates));
            }
            if buttons.just_pressed(MouseButton::Right) {
                log::info!("Trying to mark tile on {}", coordinates);
                tile_mark_ewr.send(TileMarkEvent(coordinates));
            }
        }
    }
}
