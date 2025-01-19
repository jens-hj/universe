use bevy::prelude::*;
use bevy_dynamics::Acceleration;
use strum_macros::EnumIter;

use super::{
    electron::{ELECTRON_CHARGE, ELECTRON_MASS},
    neutron::{NEUTRON_CHARGE, NEUTRON_MASS},
    proton::{PROTON_CHARGE, PROTON_MASS},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
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

impl Charge {
    pub fn charge(&self) -> f32 {
        match self {
            Charge::Positive(value) => *value,
            Charge::Negative(value) => *value,
            Charge::Neutral => 0.0,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
#[require(Acceleration)]
pub struct Particle {
    pub kind: Kind,
    pub mass: f32,
    pub radius: f32,
    pub charge: Charge,
}

impl Particle {
    pub fn proton() -> Self {
        Self {
            kind: Kind::Proton,
            mass: PROTON_MASS,
            radius: 1.0,
            charge: PROTON_CHARGE,
        }
    }

    pub fn neutron() -> Self {
        Self {
            kind: Kind::Neutron,
            mass: NEUTRON_MASS,
            radius: 1.0,
            charge: NEUTRON_CHARGE,
        }
    }

    pub fn electron() -> Self {
        Self {
            kind: Kind::Electron,
            mass: ELECTRON_MASS,
            radius: 0.5,
            charge: ELECTRON_CHARGE,
        }
    }

    pub fn photon() -> Self {
        Self {
            kind: Kind::Photon,
            mass: 0.0,
            radius: 0.1,
            charge: Charge::Neutral,
        }
    }
}

impl Default for Particle {
    fn default() -> Self {
        Particle::proton()
    }
}

pub trait GetColor {
    fn get_color(&self) -> Color;
}

impl GetColor for Particle {
    fn get_color(&self) -> Color {
        match self.kind {
            Kind::Proton => Color::srgb_u8(243, 139, 168), // red
            Kind::Neutron => Color::srgb_u8(180, 190, 254), // blue
            Kind::Electron => Color::srgb_u8(166, 227, 161), // green
            Kind::Photon => Color::srgb_u8(249, 226, 175), // yellow
        }
    }
}
