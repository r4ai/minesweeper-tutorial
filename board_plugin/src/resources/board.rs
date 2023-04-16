use crate::{bounds::Bounds2, components::Coordinates};
use bevy::prelude::*;

use super::tile_map::TileMap;

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
}

impl Board {
    pub fn init() -> Self {
        Board {
            tile_map: TileMap::empty(0, 0),
            bounds: Bounds2 {
                position: Vec2 { x: 0., y: 0. },
                size: Vec2 { x: 0., y: 0. },
            },
            tile_size: 0.,
        }
    }

    /// Translates a mouse position to board coordinates.
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        // Window to world space
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;

        // Bounds check
        if !self.bounds.in_bounds(position) {
            return None;
        }
        // World space to board space
        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }
}