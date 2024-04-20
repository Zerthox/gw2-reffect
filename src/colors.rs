#![allow(unused)]

use nexus::imgui::ImColor32;

pub type Color = [f32; 4];

/// Creates a color from RGB values.
pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    [r, g, b, 1.0]
}

/// Creates a color from RGBA values.
pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    [r, g, b, a]
}

/// Adjusts the alpha value of a color.
pub const fn with_alpha(mut color: Color, alpha: f32) -> Color {
    color[3] = alpha;
    color
}

/// Linearly interpolates between two [`f32`]s.
pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

/// Linearly interpolates between two colors.
pub fn lerp(a: Color, b: Color, t: f32) -> Color {
    [
        lerp_f32(a[0], b[0], t),
        lerp_f32(a[1], b[1], t),
        lerp_f32(a[2], b[2], t),
        lerp_f32(a[3], b[3], t),
    ]
}

pub const TRANSPARENT: Color = rgba(0.0, 0.0, 0.0, 0.0);

pub const BLACK: Color = rgb(0.0, 0.0, 0.0);

pub const WHITE: Color = rgb(1.0, 1.0, 1.0);

pub const RED: Color = rgb(1.0, 0.0, 0.0);

pub const GREEN: Color = rgb(0.0, 1.0, 0.0);

pub const BLUE: Color = rgb(0.0, 1.0, 0.0);

pub const YELLOW: Color = rgb(1.0, 1.0, 0.0);

pub const CYAN: Color = rgb(0.0, 1.0, 1.0);

pub const MAGENTA: Color = rgb(1.0, 0.0, 1.0);

pub const LIGHT_GREY: Color = rgb(0.75, 0.75, 0.75);

pub const GREY: Color = rgb(0.5, 0.5, 0.5);

pub const GUARDIAN: Color = rgb(0.45, 0.76, 0.85);

pub const WARRIOR: Color = rgb(1.00, 0.82, 0.40);

pub const REVENANT: Color = rgb(0.82, 0.43, 0.35);

pub const ENGINEER: Color = rgb(0.82, 0.61, 0.35);

pub const RANGER: Color = rgb(0.55, 0.86, 0.51);

pub const THIEF: Color = rgb(0.75, 0.56, 0.58);

pub const ELEMENTALIST: Color = rgb(0.97, 0.54, 0.53);

pub const MESMER: Color = rgb(0.71, 0.48, 0.84);

pub const NECROMANCER: Color = rgb(0.32, 0.66, 0.44);

pub const RAPTOR: Color = rgb(0.95, 0.43, 0.40);

pub const SPRINGER: Color = rgb(0.96, 0.84, 0.32);

pub const SKIMMER: Color = rgb(0.46, 0.54, 0.90);

pub const JACKAL: Color = rgb(0.42, 0.67, 0.71);

pub const GRIFFON: Color = rgb(0.56, 0.48, 0.93);

pub const ROLLER_BEETLE: Color = rgb(0.85, 0.56, 0.28);

pub const WARCLAW: Color = rgb(0.50, 0.75, 0.72);

pub const SKYSCALE: Color = rgb(0.69, 0.43, 0.78);

pub const SIEGE_TURTLE: Color = rgb(0.00, 0.67, 0.30);
