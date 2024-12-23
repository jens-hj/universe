use bevy::prelude::*;

use crate::element::Element;

#[derive(Component, Debug)]
#[require(Transform)]
pub struct Atom {
    pub element: Element,
    pub proton_count: u32,
    pub neutron_count: u32,
    pub electron_count: u32,
    pub constituent_particles: Vec<Entity>,
}

impl Atom {
    pub fn new(
        proton_count: u32,
        neutron_count: u32,
        electron_count: u32,
        constituent_particles: Vec<Entity>,
    ) -> Result<Self, String> {
        let element = Element::from_proton_count(proton_count as usize)
            .ok_or(format!("Invalid proton count: {}", proton_count))?;
        Ok(Self {
            element,
            proton_count,
            neutron_count,
            electron_count,
            constituent_particles,
        })
    }

    pub fn count(&self) -> u32 {
        self.proton_count + self.neutron_count
    }

    pub fn radius(&self) -> f32 {
        // Nuclear radius follows R = r0 * A^(1/3) where:
        // - r0 is approximately 2.4 femtometers (we'll use 2.4 as our unit scale)
        // - A is the mass number (total nucleons)
        const R0: f32 = 2.4;
        let mass_number = self.proton_count + self.neutron_count;
        R0 * (mass_number as f32).powf(1.0 / 3.0)
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<13} | {:>3} P, {:>3} N, {:>3} e",
            self.element, self.proton_count, self.neutron_count, self.electron_count
        )
    }
}

#[derive(Component, Debug, Default)]
pub struct AtomHitbox {
    pub selected: bool,
}
