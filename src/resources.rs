use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Square {
    pub mesh: Handle<Mesh>,
}

#[derive(Resource, Default)]
pub struct Materials {
    pub red: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub yellow: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
}
