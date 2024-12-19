use bevy::prelude::*;

use super::particle::{Charge, Particle};

pub const NEUTRON_MASS: f32 = 1.67492749804e-27;
pub const NEUTRON_CHARGE: Charge = Charge::Neutral;

#[derive(Component, Debug, Clone, Copy)]
#[require(Particle)]
pub struct Neutron;

// impl Particle for Neutron {
//     fn kind(&self) -> Kind {
//         Kind::Neutron
//     }

//     fn mass(&self) -> f32 {
//         NEUTRON_MASS
//     }

//     fn charge(&self) -> Charge {
//         NEUTRON_CHARGE
//     }
// }
