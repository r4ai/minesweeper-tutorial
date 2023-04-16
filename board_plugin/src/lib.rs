mod bounds;
pub mod components;
mod events;
pub mod resources;
mod systems;

use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashMap;
use resources::tile::Tile;
use resources::tile_map::TileMap;
use resources::BoardOptions;

use crate::bounds::Bounds2;
use crate::components::*;
use crate::events::TileTriggerEvent;
use crate::resources::Board;
use crate::resources::BoardPosition;
use crate::resources::TileSize;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::init())
            .add_system(systems::input::input_handling)
            .add_system(systems::uncover::trigger_event_handler)
            .add_system(systems::uncover::uncover_tiles)
            .add_event::<TileTriggerEvent>()
            .add_startup_system(Self::create_board);
        log::info!("Loaded board plugin");

        #[cfg(feature = "debug")]
        {
            app.register_type::<Coordinates>();
            app.register_type::<Bomb>();
            app.register_type::<BombNeighbor>();
            app.register_type::<Uncover>();
        }
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        mut board: ResMut<Board>,
        windows: Query<&Window>,
        assert_server: Res<AssetServer>,
    ) {
        let options = match board_options {
            Some(o) => o.clone(),
            None => BoardOptions::default(),
        };
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        let font: Handle<Font> = assert_server.load("fonts/pixeled.ttf");
        let bomb_image: Handle<Image> = assert_server.load("sprites/bomb.png");

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let window = windows.single();
        let tile_size = match options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(window, (min, max), (tile_map.width(), tile_map.height()))
            }
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        #[cfg(feature = "debug")]
        log::info!("Board size: {:?}", board_size);

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };
        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
        let mut safe_start = None;

        commands
            .spawn(SpriteBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(board_position),
                ..Default::default()
            })
            .insert(Name::new("Board"))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));
            })
            .with_children(|parent| {
                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    bomb_image,
                    font,
                    Color::DARK_GRAY,
                    &mut covered_tiles,
                    &mut safe_start,
                );
            });
        board.tile_map = tile_map;
        board.bounds = Bounds2 {
            position: board_position.truncate(),
            size: board_size,
        };
        board.tile_size = tile_size;
        board.covered_tiles = covered_tiles;

        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }
    }

    fn adaptive_tile_size(
        window: &Window,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let max_width = window.width() / width as f32;
        let max_height = window.height() / height as f32;
        max_width.min(max_height).clamp(min, max)
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                let mut cmd = parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size - padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..Default::default()
                });
                cmd.insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .insert(coordinates)
                    .with_children(|parent| {
                        let entity = parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    color: covered_tile_color,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 2.),
                                ..Default::default()
                            })
                            .insert(Name::new("Tile Cover"))
                            .id();
                        covered_tiles.insert(coordinates, entity);
                        if safe_start_entity.is_none() && *tile == Tile::Empty {
                            *safe_start_entity = Some(entity);
                        }
                    });
                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbor(v) => {
                        cmd.insert(BombNeighbor { count: *v });
                        cmd.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *v,
                                font.clone(),
                                size - padding,
                            ));
                        });
                    }
                    Tile::Empty => (),
                };
            }
        }
    }

    /// Generates the bomb counter text 2D Bundle for a given value
    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            },
        );
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        font,
                        font_size: size,
                        color,
                    },
                }],
                alignment: TextAlignment::Center,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }
}
