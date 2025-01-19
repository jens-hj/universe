use bevy::prelude::Color;
use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

// pub const MAX_ATOMIC_NUMBER: usize = 119;

#[derive(EnumString, Debug, EnumIter)]
pub enum Element {
    Hydrogen,      // 1
    Helium,        // 2
    Lithium,       // 3
    Beryllium,     // 4
    Boron,         // 5
    Carbon,        // 6
    Nitrogen,      // 7
    Oxygen,        // 8
    Fluorine,      // 9
    Neon,          // 10
    Sodium,        // 11
    Magnesium,     // 12
    Aluminum,      // 13
    Silicon,       // 14
    Phosphorus,    // 15
    Sulfur,        // 16
    Chlorine,      // 17
    Argon,         // 18
    Potassium,     // 19
    Calcium,       // 20
    Scandium,      // 21
    Titanium,      // 22
    Vanadium,      // 23
    Chromium,      // 24
    Manganese,     // 25
    Iron,          // 26
    Cobalt,        // 27
    Nickel,        // 28
    Copper,        // 29
    Zinc,          // 30
    Gallium,       // 31
    Germanium,     // 32
    Arsenic,       // 33
    Selenium,      // 34
    Bromine,       // 35
    Krypton,       // 36
    Rubidium,      // 37
    Strontium,     // 38
    Yttrium,       // 39
    Zirconium,     // 40
    Niobium,       // 41
    Molybdenum,    // 42
    Technetium,    // 43
    Ruthenium,     // 44
    Rhodium,       // 45
    Palladium,     // 46
    Silver,        // 47
    Cadmium,       // 48
    Indium,        // 49
    Tin,           // 50
    Antimony,      // 51
    Tellurium,     // 52
    Iodine,        // 53
    Xenon,         // 54
    Caesium,       // 55
    Barium,        // 56
    Lanthanum,     // 57
    Cerium,        // 58
    Praseodymium,  // 59
    Neodymium,     // 60
    Promethium,    // 61
    Samarium,      // 62
    Europium,      // 63
    Gadolinium,    // 64
    Terbium,       // 65
    Dysprosium,    // 66
    Holmium,       // 67
    Erbium,        // 68
    Thulium,       // 69
    Ytterbium,     // 70
    Lutetium,      // 71
    Hafnium,       // 72
    Tantalum,      // 73
    Tungsten,      // 74
    Rhenium,       // 75
    Osmium,        // 76
    Iridium,       // 77
    Platinum,      // 78
    Gold,          // 79
    Mercury,       // 80
    Thallium,      // 81
    Lead,          // 82
    Bismuth,       // 83
    Polonium,      // 84
    Astatine,      // 85
    Radon,         // 86
    Francium,      // 87
    Radium,        // 88
    Actinium,      // 89
    Thorium,       // 90
    Protactinium,  // 91
    Uranium,       // 92
    Neptunium,     // 93
    Plutonium,     // 94
    Americium,     // 95
    Curium,        // 96
    Berkelium,     // 97
    Californium,   // 98
    Einsteinium,   // 99
    Fermium,       // 100
    Mendelevium,   // 101
    Nobelium,      // 102
    Lawrencium,    // 103
    Rutherfordium, // 104
    Dubnium,       // 105
    Seaborgium,    // 106
    Bohrium,       // 107
    Hassium,       // 108
    Meitnerium,    // 109
    Darmstadtium,  // 110
    Roentgenium,   // 111
    Copernicium,   // 112
    Nihonium,      // 113
    Flerovium,     // 114
    Moscovium,     // 115
    Livermorium,   // 116
    Tennessine,    // 117
    Oganesson,     // 118
    Kevorkium,     // 119
}

impl Element {
    pub fn from_proton_count(proton_count: usize) -> Option<Self> {
        if proton_count == 0 || proton_count > 118 {
            return None;
        }
        Self::iter().nth(proton_count - 1)
    }

    pub fn symbol(&self) -> ElementSymbol {
        let string = self.to_string();
        ElementSymbol::from_str(&string)
            .expect(format!("Element {} has no symbol", self).as_str())
    }

    pub fn color(&self) -> Color {
        use Element::*;

        // The CPK color scheme for elements
        // https://en.wikipedia.org/wiki/CPK_coloring
        // The comments are in the format: // CPK Color (Catppuccin Colour)
        match self {
            Hydrogen => Color::srgb_u8(205, 214, 244), // White (Text)
            Carbon => Color::srgb_u8(17, 17, 27),      // Black (Crust)
            Nitrogen => Color::srgb_u8(137, 180, 250), // Blue
            Oxygen => Color::srgb_u8(235, 160, 172),   // Red (Maroon)
            Fluorine | Chlorine => Color::srgb_u8(166, 227, 161), // Green
            Bromine => Color::srgb_u8(243, 139, 168),  // Dark Red (Red)
            Iodine | Kevorkium => Color::srgb_u8(203, 166, 247), // Dark Violet (Mauve)
            Helium | Neon | Argon | Krypton | Xenon | Radon => {
                Color::srgb_u8(148, 226, 213)
            } // Cyan (Teal)
            Phosphorus => Color::srgb_u8(250, 179, 135), // Orange (Peach)
            Sulfur => Color::srgb_u8(249, 226, 175),     // Yellow (Yellow)
            Boron => Color::srgb_u8(245, 224, 220),      // Beige (Rosewater)
            Lithium | Sodium | Potassium | Rubidium | Caesium | Francium => {
                Color::srgb_u8(245, 194, 231)
            } // Violet (Pink)
            Beryllium | Magnesium | Calcium | Strontium | Barium | Radium => {
                Color::srgb_u8(166, 227, 161)
            } // Dark Green (Green)
            Titanium => Color::srgb_u8(147, 153, 178),   // Gray (Overlay2)
            Iron => Color::srgb_u8(250, 179, 135),       // Dark Orange (Peach)
            _ => Color::srgb_u8(242, 205, 205),          // Pink (Flamingo)
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:?}", self).fmt(f)
    }
}

#[derive(EnumString, Debug, EnumIter)]
pub enum ElementSymbol {
    #[strum(serialize = "H", serialize = "Hydrogen")]
    H, // 1 Hydrogen
    #[strum(serialize = "He", serialize = "Helium")]
    He, // 2 Helium
    #[strum(serialize = "Li", serialize = "Lithium")]
    Li, // 3 Lithium
    #[strum(serialize = "Be", serialize = "Beryllium")]
    Be, // 4 Beryllium
    #[strum(serialize = "B", serialize = "Boron")]
    B, // 5 Boron
    #[strum(serialize = "C", serialize = "Carbon")]
    C, // 6 Carbon
    #[strum(serialize = "N", serialize = "Nitrogen")]
    N, // 7 Nitrogen
    #[strum(serialize = "O", serialize = "Oxygen")]
    O, // 8 Oxygen
    #[strum(serialize = "F", serialize = "Fluorine")]
    F, // 9 Fluorine
    #[strum(serialize = "Ne", serialize = "Neon")]
    Ne, // 10 Neon
    #[strum(serialize = "Na", serialize = "Sodium")]
    Na, // 11 Sodium
    #[strum(serialize = "Mg", serialize = "Magnesium")]
    Mg, // 12 Magnesium
    #[strum(serialize = "Al", serialize = "Aluminum")]
    Al, // 13 Aluminum
    #[strum(serialize = "Si", serialize = "Silicon")]
    Si, // 14 Silicon
    #[strum(serialize = "P", serialize = "Phosphorus")]
    P, // 15 Phosphorus
    #[strum(serialize = "S", serialize = "Sulfur")]
    S, // 16 Sulfur
    #[strum(serialize = "Cl", serialize = "Chlorine")]
    Cl, // 17 Chlorine
    #[strum(serialize = "Ar", serialize = "Argon")]
    Ar, // 18 Argon
    #[strum(serialize = "K", serialize = "Potassium")]
    K, // 19 Potassium
    #[strum(serialize = "Ca", serialize = "Calcium")]
    Ca, // 20 Calcium
    #[strum(serialize = "Sc", serialize = "Scandium")]
    Sc, // 21 Scandium
    #[strum(serialize = "Ti", serialize = "Titanium")]
    Ti, // 22 Titanium
    #[strum(serialize = "V", serialize = "Vanadium")]
    V, // 23 Vanadium
    #[strum(serialize = "Cr", serialize = "Chromium")]
    Cr, // 24 Chromium
    #[strum(serialize = "Mn", serialize = "Manganese")]
    Mn, // 25 Manganese
    #[strum(serialize = "Fe", serialize = "Iron")]
    Fe, // 26 Iron
    #[strum(serialize = "Co", serialize = "Cobalt")]
    Co, // 27 Cobalt
    #[strum(serialize = "Ni", serialize = "Nickel")]
    Ni, // 28 Nickel
    #[strum(serialize = "Cu", serialize = "Copper")]
    Cu, // 29 Copper
    #[strum(serialize = "Zn", serialize = "Zinc")]
    Zn, // 30 Zinc
    #[strum(serialize = "Ga", serialize = "Gallium")]
    Ga, // 31 Gallium
    #[strum(serialize = "Ge", serialize = "Germanium")]
    Ge, // 32 Germanium
    #[strum(serialize = "As", serialize = "Arsenic")]
    As, // 33 Arsenic
    #[strum(serialize = "Se", serialize = "Selenium")]
    Se, // 34 Selenium
    #[strum(serialize = "Br", serialize = "Bromine")]
    Br, // 35 Bromine
    #[strum(serialize = "Kr", serialize = "Krypton")]
    Kr, // 36 Krypton
    #[strum(serialize = "Rb", serialize = "Rubidium")]
    Rb, // 37 Rubidium
    #[strum(serialize = "Sr", serialize = "Strontium")]
    Sr, // 38 Strontium
    #[strum(serialize = "Y", serialize = "Yttrium")]
    Y, // 39 Yttrium
    #[strum(serialize = "Zr", serialize = "Zirconium")]
    Zr, // 40 Zirconium
    #[strum(serialize = "Nb", serialize = "Niobium")]
    Nb, // 41 Niobium
    #[strum(serialize = "Mo", serialize = "Molybdenum")]
    Mo, // 42 Molybdenum
    #[strum(serialize = "Tc", serialize = "Technetium")]
    Tc, // 43 Technetium
    #[strum(serialize = "Ru", serialize = "Ruthenium")]
    Ru, // 44 Ruthenium
    #[strum(serialize = "Rh", serialize = "Rhodium")]
    Rh, // 45 Rhodium
    #[strum(serialize = "Pd", serialize = "Palladium")]
    Pd, // 46 Palladium
    #[strum(serialize = "Ag", serialize = "Silver")]
    Ag, // 47 Silver
    #[strum(serialize = "Cd", serialize = "Cadmium")]
    Cd, // 48 Cadmium
    #[strum(serialize = "In", serialize = "Indium")]
    In, // 49 Indium
    #[strum(serialize = "Sn", serialize = "Tin")]
    Sn, // 50 Tin
    #[strum(serialize = "Sb", serialize = "Antimony")]
    Sb, // 51 Antimony
    #[strum(serialize = "Te", serialize = "Tellurium")]
    Te, // 52 Tellurium
    #[strum(serialize = "I", serialize = "Iodine")]
    I, // 53 Iodine
    #[strum(serialize = "Xe", serialize = "Xenon")]
    Xe, // 54 Xenon
    #[strum(serialize = "Cs", serialize = "Caesium")]
    Cs, // 55 Caesium
    #[strum(serialize = "Ba", serialize = "Barium")]
    Ba, // 56 Barium
    #[strum(serialize = "La", serialize = "Lanthanum")]
    La, // 57 Lanthanum
    #[strum(serialize = "Ce", serialize = "Cerium")]
    Ce, // 58 Cerium
    #[strum(serialize = "Pr", serialize = "Praseodymium")]
    Pr, // 59 Praseodymium
    #[strum(serialize = "Nd", serialize = "Neodymium")]
    Nd, // 60 Neodymium
    #[strum(serialize = "Pm", serialize = "Promethium")]
    Pm, // 61 Promethium
    #[strum(serialize = "Sm", serialize = "Samarium")]
    Sm, // 62 Samarium
    #[strum(serialize = "Eu", serialize = "Europium")]
    Eu, // 63 Europium
    #[strum(serialize = "Gd", serialize = "Gadolinium")]
    Gd, // 64 Gadolinium
    #[strum(serialize = "Tb", serialize = "Terbium")]
    Tb, // 65 Terbium
    #[strum(serialize = "Dy", serialize = "Dysprosium")]
    Dy, // 66 Dysprosium
    #[strum(serialize = "Ho", serialize = "Holmium")]
    Ho, // 67 Holmium
    #[strum(serialize = "Er", serialize = "Erbium")]
    Er, // 68 Erbium
    #[strum(serialize = "Tm", serialize = "Thulium")]
    Tm, // 69 Thulium
    #[strum(serialize = "Yb", serialize = "Ytterbium")]
    Yb, // 70 Ytterbium
    #[strum(serialize = "Lu", serialize = "Lutetium")]
    Lu, // 71 Lutetium
    #[strum(serialize = "Hf", serialize = "Hafnium")]
    Hf, // 72 Hafnium
    #[strum(serialize = "Ta", serialize = "Tantalum")]
    Ta, // 73 Tantalum
    #[strum(serialize = "W", serialize = "Tungsten")]
    W, // 74 Tungsten
    #[strum(serialize = "Re", serialize = "Rhenium")]
    Re, // 75 Rhenium
    #[strum(serialize = "Os", serialize = "Osmium")]
    Os, // 76 Osmium
    #[strum(serialize = "Ir", serialize = "Iridium")]
    Ir, // 77 Iridium
    #[strum(serialize = "Pt", serialize = "Platinum")]
    Pt, // 78 Platinum
    #[strum(serialize = "Au", serialize = "Gold")]
    Au, // 79 Gold
    #[strum(serialize = "Hg", serialize = "Mercury")]
    Hg, // 80 Mercury
    #[strum(serialize = "Tl", serialize = "Thallium")]
    Tl, // 81 Thallium
    #[strum(serialize = "Pb", serialize = "Lead")]
    Pb, // 82 Lead
    #[strum(serialize = "Bi", serialize = "Bismuth")]
    Bi, // 83 Bismuth
    #[strum(serialize = "Po", serialize = "Polonium")]
    Po, // 84 Polonium
    #[strum(serialize = "At", serialize = "Astatine")]
    At, // 85 Astatine
    #[strum(serialize = "Rn", serialize = "Radon")]
    Rn, // 86 Radon
    #[strum(serialize = "Fr", serialize = "Francium")]
    Fr, // 87 Francium
    #[strum(serialize = "Ra", serialize = "Radium")]
    Ra, // 88 Radium
    #[strum(serialize = "Ac", serialize = "Actinium")]
    Ac, // 89 Actinium
    #[strum(serialize = "Th", serialize = "Thorium")]
    Th, // 90 Thorium
    #[strum(serialize = "Pa", serialize = "Protactinium")]
    Pa, // 91 Protactinium
    #[strum(serialize = "U", serialize = "Uranium")]
    U, // 92 Uranium
    #[strum(serialize = "Np", serialize = "Neptunium")]
    Np, // 93 Neptunium
    #[strum(serialize = "Pu", serialize = "Plutonium")]
    Pu, // 94 Plutonium
    #[strum(serialize = "Am", serialize = "Americium")]
    Am, // 95 Americium
    #[strum(serialize = "Cm", serialize = "Curium")]
    Cm, // 96 Curium
    #[strum(serialize = "Bk", serialize = "Berkelium")]
    Bk, // 97 Berkelium
    #[strum(serialize = "Cf", serialize = "Californium")]
    Cf, // 98 Californium
    #[strum(serialize = "Es", serialize = "Einsteinium")]
    Es, // 99 Einsteinium
    #[strum(serialize = "Fm", serialize = "Fermium")]
    Fm, // 100 Fermium
    #[strum(serialize = "Md", serialize = "Mendelevium")]
    Md, // 101 Mendelevium
    #[strum(serialize = "No", serialize = "Nobelium")]
    No, // 102 Nobelium
    #[strum(serialize = "Lr", serialize = "Lawrencium")]
    Lr, // 103 Lawrencium
    #[strum(serialize = "Rf", serialize = "Rutherfordium")]
    Rf, // 104 Rutherfordium
    #[strum(serialize = "Db", serialize = "Dubnium")]
    Db, // 105 Dubnium
    #[strum(serialize = "Sg", serialize = "Seaborgium")]
    Sg, // 106 Seaborgium
    #[strum(serialize = "Bh", serialize = "Bohrium")]
    Bh, // 107 Bohrium
    #[strum(serialize = "Hs", serialize = "Hassium")]
    Hs, // 108 Hassium
    #[strum(serialize = "Mt", serialize = "Meitnerium")]
    Mt, // 109 Meitnerium
    #[strum(serialize = "Ds", serialize = "Darmstadtium")]
    Ds, // 110 Darmstadtium
    #[strum(serialize = "Rg", serialize = "Roentgenium")]
    Rg, // 111 Roentgenium
    #[strum(serialize = "Cn", serialize = "Copernicium")]
    Cn, // 112 Copernicium
    #[strum(serialize = "Nh", serialize = "Nihonium")]
    Nh, // 113 Nihonium
    #[strum(serialize = "Fl", serialize = "Flerovium")]
    Fl, // 114 Flerovium
    #[strum(serialize = "Mc", serialize = "Moscovium")]
    Mc, // 115 Moscovium
    #[strum(serialize = "Lv", serialize = "Livermorium")]
    Lv, // 116 Livermorium
    #[strum(serialize = "Ts", serialize = "Tennessine")]
    Ts, // 117 Tennessine
    #[strum(serialize = "Og", serialize = "Oganesson")]
    Og, // 118 Oganesson
    #[strum(serialize = "Kv", serialize = "Kevorkium")]
    Kv, // 119 Kevorkium
}

impl std::fmt::Display for ElementSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:?}", self).fmt(f)
    }
}
