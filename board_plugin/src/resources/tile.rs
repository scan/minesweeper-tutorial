use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;
#[cfg(feature = "debug")]
use colored::Colorize;

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Resource)]
#[cfg_attr(feature = "debug", reflect(Resource, InspectorOptions))]
pub enum Tile {
    Empty,
    Bomb,
    BombNeighbour(u8),
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

impl Tile {
    /// Is the tile a bomb?
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbour(v) => match v {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => v.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }
}
