#![allow(non_snake_case)]

extern crate simple_simplex as simplex;
use simplex::NoiseConfig;

#[test]
fn main() {
    let vector: Vec<char> = vec![' ', '.', '-', '=', 'z', 'X', '#'];

    let config: NoiseConfig = NoiseConfig::new(
        1,                                // Octaves
        0.015,                            // X-Frequency
        0.015,                            // Y-Frequency
        0.05,                             // Amplitude
        2.5,                              // Lacunarity
        0.5,                              // Gain
        (0.0, (vector.len() - 1) as f32), // range
        97838586,                         // seed
    );

    config.output(300, &vector);

    // This is a change. if this shows up the push worked.
}
