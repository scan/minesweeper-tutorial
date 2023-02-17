use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
#[cfg_attr(feature = "debug", reflect(Component, InspectorOptions))]
pub struct BombNeighbour {
    pub count: u8,
}
