use bevy::prelude::*;

use super::particle::{Charge, Particle};

pub const PROTON_MASS: f32 = 1.67262192369e-27;
pub const PROTON_CHARGE: Charge = Charge::Positive(1.602176634e-19);

#[derive(Component, Debug, Clone, Copy)]
#[require(Particle)]
pub struct Proton;

// impl Particle for Proton {
//     fn kind(&self) -> Kind {
//         Kind::Proton
//     }

//     fn mass(&self) -> f32 {
//         PROTON_MASS
//     }

//     fn charge(&self) -> Charge {
//         PROTON_CHARGE
//     }
// }
