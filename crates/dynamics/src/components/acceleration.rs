use bevy::prelude::*;

#[derive(Component)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

impl Default for Acceleration {
    fn default() -> Self {
        Self { value: Vec3::ZERO }
    }
}
