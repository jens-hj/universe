use bevy::prelude::*;

use crate::element::Element;

#[derive(Component, Debug)]
pub struct Atom {
    pub element: Element,
    pub nucleus_center: Vec3,
    pub proton_count: u32,
    pub neutron_count: u32,
    pub electron_count: u32,
    pub constituent_particles: Vec<Entity>,
}

impl Atom {
    pub fn new(
        nucleus_center: Vec3,
        proton_count: u32,
        neutron_count: u32,
        electron_count: u32,
        constituent_particles: Vec<Entity>,
    ) -> Result<Self, String> {
        let element = Element::from_proton_count(proton_count as usize)
            .ok_or(format!("Invalid proton count: {}", proton_count))?;
        Ok(Self {
            element,
            nucleus_center,
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
        // self.count() as f32 * 0.6204
        self.count() as f32
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:12} | {:3} P, {:3} N, {:3} e",
            self.element, self.proton_count, self.neutron_count, self.electron_count
        )
    }
}

#[derive(Component, Debug)]
pub struct AtomHitbox;
