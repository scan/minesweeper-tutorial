use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[derive(Debug, Clone, Copy, Resource, Serialize, Deserialize)]
#[cfg_attr(feature = "debug", reflect(Resource, InspectorOptions))]
pub enum TileSize {
    Fixed(f32),
    Adaptive { min: f32, max: f32 },
}

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
#[cfg_attr(feature = "debug", reflect(Resource, InspectorOptions))]
pub enum BoardPosition {
    Centered { offset: Vec3 },
    Custom(Vec3),
}

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
#[cfg_attr(feature = "debug", reflect(Resource, InspectorOptions))]
pub struct BoardOptions {
    pub map_size: (u16, u16),
    pub bomb_count: u16,
    pub position: BoardPosition,
    pub tile_size: TileSize,
    pub tile_padding: f32,
    pub safe_start: bool,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            bomb_count: 30,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.0,
            safe_start: false,
        }
    }
}
