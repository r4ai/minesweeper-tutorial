use bevy::prelude::Component;
use bevy::reflect::Reflect;

#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectorOptions;

/// Uncover component
#[cfg_attr(feature = "debug", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Uncover;
