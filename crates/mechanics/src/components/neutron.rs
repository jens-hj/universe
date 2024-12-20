use bevy::prelude::*;

use super::particle::{Charge, Particle};

// with proton mass set at 1.0, neutron mass is 1.008665
pub const NEUTRON_MASS: f32 = 1.008665;
pub const NEUTRON_CHARGE: Charge = Charge::Neutral;

#[derive(Component, Debug, Clone, Copy)]
#[require(Particle)]
pub struct Neutron;
