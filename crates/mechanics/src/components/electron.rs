use bevy::prelude::*;

use super::particle::{Charge, Particle};

// with proton mass set at 1.0
// mass of an electron is 9.109×10^−31
// mass of a proton is 1.67262192×10^-27
pub const ELECTRON_MASS: f32 = 0.000545;
pub const ELECTRON_CHARGE: Charge = Charge::Negative(-1.0);

#[derive(Component, Debug, Clone, Copy)]
#[require(Particle)]
pub struct Electron;
