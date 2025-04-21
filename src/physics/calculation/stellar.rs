use crate::utils::misc::float_interpolate;
use bevy::prelude::Color;

// Planck's constant × Speed of light / Boltzmann constant
const HC_K: f64 = 1.4388e7; // nm·K

pub fn stellar_surface_temperature_from_mass(mass: f64) -> f64 {
    let exponent = stellar_surface_temperature_exponent(mass);
    5800.0 * mass.powf(exponent)
}

pub fn stellar_surface_temperature_exponent(mass: f64) -> f64 {
    match mass {
        0.0..0.5 => float_interpolate(mass, 0.0, 0.5, 0.4, 0.475),
        0.5..1.5 => float_interpolate(mass, 0.5, 1.5, 0.475, 0.525),
        1.5..10.0 => float_interpolate(mass, 1.5, 10.0, 0.525, 0.6),
        10.0..150.0 => float_interpolate(mass, 10.0, 150.0, 0.6, 0.7),
        _ => 0.7,
    }
}

pub fn stellar_radius_from_mass(mass: f64) -> f64 {
    let exponent = stellar_radius_exponent(mass);
    mass.powf(exponent)
}

fn stellar_radius_exponent(mass: f64) -> f64 {
    1.0 - match mass {
        0.0..0.5 => float_interpolate(mass, 0.0, 0.5, 0.0, 0.1),
        0.5..2.0 => float_interpolate(mass, 0.5, 2.0, 0.1, 0.25),
        2.0..5.0 => float_interpolate(mass, 2.0, 5.0, 0.25, 0.4),
        5.0..150.0 => float_interpolate(mass, 5.0, 150.0, 0.4, 0.5),
        _ => 0.5,
    }
}

pub fn stellar_luminosity_from_mass(mass: f64) -> f64 {
    let exponent = stellar_luminosity_exponent(mass);
    mass.powf(exponent)
}

fn stellar_luminosity_exponent(mass: f64) -> f64 {
    match mass {
        0.0..0.5 => float_interpolate(mass, 0.0, 0.5, 2.0, 2.3),
        0.5..1.0 => float_interpolate(mass, 0.5, 1.0, 2.3, 3.5),
        1.0..10.0 => float_interpolate(mass, 1.0, 10.0, 3.5, 3.8),
        10.0..150.0 => float_interpolate(mass, 10.0, 150.0, 3.8, 4.0),
        _ => 4.0,
    }
}

pub fn stellar_color_from_surface_temperature(temp_kelvin: f64) -> Color {
    const WAVELENGTH_RED_NM: f64 = 650.0;
    const WAVELENGTH_GREEN_NM: f64 = 550.0;
    const WAVELENGTH_BLUE_NM: f64 = 450.0;
    const GAMMA: f64 = 0.8; // Adjustable (usually between 0.4-1.0)

    // RGB intensities based on blackbody radiation
    let red = spectral_intensity(WAVELENGTH_RED_NM, temp_kelvin);
    let green = spectral_intensity(WAVELENGTH_GREEN_NM, temp_kelvin);
    let blue = spectral_intensity(WAVELENGTH_BLUE_NM, temp_kelvin);

    let max_intensity = red.max(green).max(blue);

    let r = (red / max_intensity).powf(GAMMA);
    let g = (green / max_intensity).powf(GAMMA);
    let b = (blue / max_intensity).powf(GAMMA);

    Color::linear_rgb(r as f32, g as f32, b as f32)
}

fn spectral_intensity(wavelength_nm: f64, temp_kelvin: f64) -> f64 {
    let exponent = HC_K / (wavelength_nm * temp_kelvin);
    1.0 / (wavelength_nm.powi(5) * (f64::exp(exponent) - 1.0))
}
