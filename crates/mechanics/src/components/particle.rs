use bevy::prelude::*;

use super::{
    neutron::{NEUTRON_CHARGE, NEUTRON_MASS},
    proton::{PROTON_CHARGE, PROTON_MASS},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Proton,
    Neutron,
    Electron,
    Photon,
}

#[derive(Debug, Clone, Copy)]
pub enum Charge {
    Positive(f32),
    Negative(f32),
    Neutral,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Particle {
    pub kind: Kind,
    pub mass: f32,
    pub charge: Charge,
}

impl Particle {
    pub fn proton() -> Self {
        Self {
            kind: Kind::Proton,
            mass: PROTON_MASS,
            charge: PROTON_CHARGE,
        }
    }

    pub fn neutron() -> Self {
        Self {
            kind: Kind::Neutron,
            mass: NEUTRON_MASS,
            charge: NEUTRON_CHARGE,
        }
    }
}

impl Default for Particle {
    fn default() -> Self {
        Particle::proton()
    }
}

// pub trait Particle {
//     fn kind(&self) -> Kind;
//     fn mass(&self) -> f32;
//     fn charge(&self) -> Charge;
// }

// #[derive(Component, Debug, Clone, Copy)]
// pub struct ParticleMarker;
