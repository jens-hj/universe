use bevy::prelude::*;

use super::particle::{Charge, Particle};

// with proton mass set at 1.0, electron mass is 0.00054858
pub const ELECTRON_MASS: f32 = 0.00054858;
pub const ELECTRON_CHARGE: Charge = Charge::Negative(-1.0);

#[derive(Component, Debug, Clone, Copy)]
#[require(Particle)]
pub struct Electron;
