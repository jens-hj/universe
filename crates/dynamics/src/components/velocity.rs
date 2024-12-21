use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self { value: Vec3::ZERO }
    }
}
