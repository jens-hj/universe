use bevy::prelude::*;

use super::particle::{Charge, Particle};

pub const PROTON_MASS: f32 = 1.0;
pub const PROTON_CHARGE: Charge = Charge::Positive(1.0);

#[derive(Component, Debug, Clone, Copy)]
#[require(Particle)]
pub struct Proton;
